#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct TimedEvent {
    pub timing: u32,
    pub event: Event,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum Event {
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8, velocity: u8 },
}

use komp_core::*;
const NS_PER_MS: u64 = 1_000_000;

pub fn schedule(
    offset: u64,
    timed_events: Vec<TimedEvent>,
    key: Key,
    ms_per_quarter: u32,
    ticks_per_quarter: u32,
) -> coremidi::PacketBuffer {
    let one_bar = ms_per_quarter as u64 * 4 * NS_PER_MS;
    schedule_timeslice(
        offset,
        offset,
        one_bar,
        timed_events,
        one_bar,
        key,
        ms_per_quarter,
        ticks_per_quarter,
    )
}

fn ticks_to_time(offset: u64, ticks: u32, ms_per_quarter: u32, ticks_per_quarter: u32) -> u64 {
    offset + (NS_PER_MS * ticks as u64 * ms_per_quarter as u64 / ticks_per_quarter as u64)
}

pub fn schedule_timeslice(
    pattern_start: u64,
    now: u64,
    timeslice: u64,
    timed_events: Vec<TimedEvent>,
    pattern_length: u64,
    key: Key,
    ms_per_quarter: u32,
    ticks_per_quarter: u32,
) -> coremidi::PacketBuffer {
    let mut packet_buf = coremidi::PacketBuffer::with_capacity(512);
    for te in timed_events.iter() {
        let mut event_time =
            ticks_to_time(pattern_start, te.timing, ms_per_quarter, ticks_per_quarter);
        if now + timeslice >= pattern_start + pattern_length {
            while event_time < now {
                event_time += pattern_length;
            }
        }
        if event_time < now || event_time > now + timeslice {
            continue;
        }
        let data = match te.event {
            Event::NoteOn {
                channel,
                note,
                velocity,
            } => [
                0x90 | (channel & 0x0f),
                (key.0 + note) & 0x7f,
                velocity & 0x7f,
            ],
            Event::NoteOff {
                channel,
                note,
                velocity,
            } => [
                0x80 | (channel & 0x0f),
                (key.0 + note) & 0x7f,
                velocity & 0x7f,
            ],
        };
        packet_buf.push_data(event_time, &data);
    }

    packet_buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::*;
    use crate::Playing;
    use komp_core::C_KEY;

    fn extract_timings(packet_buf: &coremidi::PacketBuffer) -> Vec<u64> {
        let mut timings = vec![];
        for packet in packet_buf.iter() {
            timings.push(packet.timestamp());
        }

        timings.sort();
        timings.dedup();
        timings
    }

    fn package_pattern_timeslice(pattern_start: u64, now: u64) -> coremidi::PacketBuffer {
        let ticks_per_quarter = 96;
        let ms_per_quarter = 500;
        let progression = [Chord::Major(C_KEY), Chord::Major(F_KEY)];
        let timed_events = create_bars(ticks_per_quarter, &progression);
        let two_hundred_and_fifty_ms = 250 * NS_PER_MS;
        // two bars at this tempo is exactly 4 seconds
        let pattern_length = 4_000 * NS_PER_MS;
        schedule_timeslice(
            pattern_start,
            now,
            two_hundred_and_fifty_ms,
            timed_events,
            pattern_length,
            C_KEY,
            ms_per_quarter,
            ticks_per_quarter,
        )
    }

    fn verify_playing(
        packet_buf: &coremidi::PacketBuffer,
        mut playing: Playing,
        afterwards: Playing,
    ) {
        for packet in packet_buf.iter() {
            for chunk in packet.data().chunks(3) {
                crate::process_midi(chunk, &mut playing);
            }
        }
        assert_eq!(playing, afterwards);
    }

    #[test]
    fn test_partial_scheduling_events() {
        let pattern_start = 200_000_000_000_000;
        // just before the last note-offs in the first bar (C key)
        // this slice extends over to the first note-ons of the second bar (F key)
        let now = pattern_start + 1_800 * NS_PER_MS;
        let packet_buf = package_pattern_timeslice(pattern_start, now);

        // pretend that a C Major chord is playing in octave 3
        // afterwards the currently playing notes should be a F Major chord
        let playing = vec![(0, NOTE_C3), (0, NOTE_E3), (0, NOTE_G3)];
        let afterwards = vec![(0, NOTE_F3), (0, NOTE_A3), (0, NOTE_C4)];

        verify_playing(&packet_buf, playing, afterwards);
    }

    #[test]
    fn test_partial_scheduling_timing() {
        let pattern_start = 200_000_000_000_000;
        // just before the last note-offs in the first bar (C key)
        // this slice extends over to the first note-ons of the second bar (F key)
        let now = pattern_start + 1_800 * NS_PER_MS;
        let packet_buf = package_pattern_timeslice(pattern_start, now);
        let timings = extract_timings(&packet_buf);

        let note_offs = pattern_start + 1_875 * NS_PER_MS;
        assert_eq!(timings[0], note_offs);
        let note_ons = pattern_start + 2_000 * NS_PER_MS;
        assert_eq!(timings[1], note_ons);
    }

    #[test]
    fn test_partial_scheduling_events_with_loop() {
        let pattern_start = 200_000_000_000_000;
        // just before the last note-offs in the second bar (F key)
        // this slice extends over to the first note-ons of the repeated first bar (C key)
        let now = pattern_start + 3_800 * NS_PER_MS;
        let packet_buf = package_pattern_timeslice(pattern_start, now);

        // pretend that a F Major chord is playing in octave 3
        // afterwards the currently playing notes should be a C Major chord
        let playing = vec![(0, NOTE_F3), (0, NOTE_A3), (0, NOTE_C4)];
        let afterwards = vec![(0, NOTE_C3), (0, NOTE_E3), (0, NOTE_G3)];

        verify_playing(&packet_buf, playing, afterwards);
    }

    #[test]
    fn test_partial_scheduling_timing_with_loop() {
        let pattern_start = 200_000_000_000_000;
        // just before the last note-offs in the second bar (F key)
        // this slice extends over to the first note-ons of the repeated first bar (C key)
        let now = pattern_start + 3_800 * NS_PER_MS;
        let packet_buf = package_pattern_timeslice(pattern_start, now);
        let timings = extract_timings(&packet_buf);

        let note_offs = pattern_start + 3_875 * NS_PER_MS;
        assert_eq!(timings[0], note_offs);
        let note_ons = pattern_start + 4_000 * NS_PER_MS;
        assert_eq!(timings[1], note_ons);
    }

    fn create_packets(
        ticks_per_quarter: u32,
        ms_per_quarter: u32,
    ) -> (u64, coremidi::PacketBuffer) {
        let timed_events = create_bar(ticks_per_quarter, Chord::Major(C_KEY));
        let timestamp = crate::now();
        let packet_buf = schedule(
            timestamp,
            timed_events,
            C_KEY,
            ms_per_quarter,
            ticks_per_quarter,
        );

        (timestamp, packet_buf)
    }

    fn assert_timings(packet_buf: coremidi::PacketBuffer, timestamp: u64, ms_per_quarter: u32) {
        assert_ne!(packet_buf.len(), 0);

        let timings = extract_timings(&packet_buf);

        // even timings are note ons, odds are note offs
        assert_eq!(timings[0] - timestamp, 0 as u64);
        assert_eq!(
            timings[2] - timestamp,
            1 * ms_per_quarter as u64 * NS_PER_MS
        );
        assert_eq!(
            timings[4] - timestamp,
            2 * ms_per_quarter as u64 * NS_PER_MS
        );
        assert_eq!(
            timings[6] - timestamp,
            3 * ms_per_quarter as u64 * NS_PER_MS
        );
    }

    #[test]
    fn test_chord_part_scheduled_timing_lores() {
        let ticks_per_quarter = 16;
        let ms_per_quarter = 500;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }

    #[test]
    fn test_chord_part_scheduled_timing_lores_slow() {
        let ticks_per_quarter = 16;
        let ms_per_quarter = 500_000;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }

    #[test]
    fn test_chord_part_scheduled_timing_hires() {
        let ticks_per_quarter = 96_000;
        let ms_per_quarter = 500;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }

    #[test]
    fn test_chord_part_scheduled_timing_hires_fast() {
        let ticks_per_quarter = 96_000;
        let ms_per_quarter = 5;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }
}
