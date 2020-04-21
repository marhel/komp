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
pub const NS_PER_MS: u64 = 1_000_000;

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
        &timed_events,
        one_bar,
        key,
        ms_per_quarter,
        ticks_per_quarter,
    )
}

fn ticks_to_time(offset: u64, ticks: u32, ms_per_quarter: u32, ticks_per_quarter: u32) -> u64 {
    offset + (NS_PER_MS * ticks as u64 * ms_per_quarter as u64 / ticks_per_quarter as u64)
}

fn schedule_timeslice(
    pattern_start: u64,
    now: u64,
    timeslice: u64,
    timed_events: &Vec<TimedEvent>,
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
        if event_time < now || event_time >= now + timeslice {
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

pub struct Scheduler {
    pattern_start: u64,
    slice_length: u64,
    scheduling_deadline_margin: u64,
    timed_events: Vec<TimedEvent>,
    pattern_length: u64,
    ms_per_quarter: u32,
    ticks_per_quarter: u32,
}

impl Scheduler {
    pub fn new(
        pattern_start: u64,
        slice_length: u64,
        scheduling_deadline_margin: u64,
        timed_events: Vec<TimedEvent>,
        pattern_length: u64,
        ms_per_quarter: u32,
        ticks_per_quarter: u32,
    ) -> Scheduler {
        Scheduler {
            pattern_start,
            slice_length,
            scheduling_deadline_margin,
            timed_events,
            pattern_length,
            ms_per_quarter,
            ticks_per_quarter,
        }
    }

    pub fn schedule_slice(
        &mut self,
        now: u64,
        slice_start: &mut u64,
        key: Key,
    ) -> (i64, coremidi::PacketBuffer) {
        *slice_start = std::cmp::max(*slice_start, self.pattern_start);

        let packet_buf = schedule_timeslice(
            self.pattern_start,
            *slice_start,
            self.slice_length,
            &self.timed_events,
            self.pattern_length,
            key,
            self.ms_per_quarter,
            self.ticks_per_quarter,
        );

        *slice_start += self.slice_length;
        let next_slice_due = *slice_start;
        if next_slice_due >= self.pattern_start + self.pattern_length {
            self.pattern_start += self.pattern_length;
        }
        let sleep_time: i64 = ((next_slice_due - self.scheduling_deadline_margin) - (now)) as i64;

        (sleep_time, packet_buf)
    }

    pub fn pattern_start(&self) -> u64 {
        self.pattern_start
    }
    pub fn pattern_length(&self) -> u64 {
        self.pattern_length
    }
    pub fn slice_length(&self) -> u64 {
        self.slice_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::*;
    use crate::Playing;
    use komp_core::C_KEY;
    use std::collections::HashSet;

    fn create_scheduler() -> Scheduler {
        let pattern_start = 200_000_000_000_000;
        let ticks_per_quarter = 96;
        let ms_per_quarter = 500;
        let progression = [Chord::Major(C_KEY), Chord::Major(F_KEY)];
        let timed_events = create_bars(ticks_per_quarter, &progression);
        let slice_length = 200 * NS_PER_MS;
        // two bars at this tempo is exactly 4 seconds
        let pattern_length = 4_000 * NS_PER_MS;
        let scheduling_deadline_margin = 50 * NS_PER_MS;

        Scheduler::new(
            pattern_start,
            slice_length,
            scheduling_deadline_margin,
            timed_events,
            pattern_length,
            ms_per_quarter,
            ticks_per_quarter,
        )
    }

    #[test]
    fn test_scheduler_multiple_loops() {
        let mut scheduler = create_scheduler();
        let initial_start = scheduler.pattern_start();
        let mut now = scheduler.pattern_start();
        let mut slice_start = 0;

        while slice_start + scheduler.slice_length()
            <= initial_start + 160 * scheduler.pattern_length()
        {
            let (sleep_time, _) = scheduler.schedule_slice(now, &mut slice_start, C_KEY);

            // use wrapping add to simulate adding a negative number
            now = now.wrapping_add(sleep_time as u64);
        }
        assert!(initial_start < scheduler.pattern_start());
        assert_eq!(slice_start, scheduler.pattern_start());
    }

    #[test]
    fn test_scheduler() {
        let mut scheduler = create_scheduler();
        let initial_start = scheduler.pattern_start();
        let mut now = initial_start;
        let mut playing = vec![];
        let mut played = HashSet::new();
        let wake_up_jitter = [-10_000_123, 20_123_234, -30_000_123, 10_456_234, 70_000_001];
        let mut slices = 0;
        let mut slice_start = now;

        while slice_start + scheduler.slice_length()
            <= initial_start + 2 * scheduler.pattern_length()
        {
            let (sleep_time, packet_buf) = scheduler.schedule_slice(now, &mut slice_start, C_KEY);
            for packet in packet_buf.iter() {
                for chunk in packet.data().chunks(3) {
                    crate::process_midi(chunk, &mut playing);
                    played.insert(playing.clone());
                }
            }
            // use wrapping add to simulate adding a negative number
            now = now
                .wrapping_add((sleep_time + wake_up_jitter[slices % wake_up_jitter.len()]) as u64);
            slices += 1;
        }

        assert!(slice_start == initial_start + 2 * scheduler.pattern_length());
        assert_eq!(slice_start, scheduler.pattern_start());
        assert_eq!(playing, vec![]);

        let f_major = vec![(0, NOTE_F3), (0, NOTE_A3), (0, NOTE_C4)];
        let c_major = vec![(0, NOTE_C3), (0, NOTE_E3), (0, NOTE_G3)];
        assert!(played.contains(&vec![]));
        assert!(played.contains(&c_major));
        assert!(played.contains(&f_major));
    }

    #[test]
    fn test_continual_scheduling() {
        let pattern_start = 200_000_000_000_000;
        let ticks_per_quarter = 96;
        let ms_per_quarter = 500;
        let progression = [Chord::Major(C_KEY), Chord::Major(F_KEY)];
        let timed_events = create_bars(ticks_per_quarter, &progression);
        let slice_length = 200 * NS_PER_MS;
        // two bars at this tempo is exactly 4 seconds
        let pattern_length = 4_000 * NS_PER_MS;
        let scheduling_deadline_margin = 50 * NS_PER_MS;

        let mut slice_start = pattern_start;
        let mut now = pattern_start;
        let mut playing = vec![];
        let mut played = HashSet::new();
        let wake_up_jitter = [-10_000_123, 20_123_234, -30_000_123, 10_456_234, 70_000_001];
        let mut slices = 0;
        while slice_start + slice_length <= pattern_start + 2 * pattern_length {
            let packet_buf = schedule_timeslice(
                pattern_start,
                slice_start,
                slice_length,
                &timed_events,
                pattern_length,
                C_KEY,
                ms_per_quarter,
                ticks_per_quarter,
            );

            for packet in packet_buf.iter() {
                for chunk in packet.data().chunks(3) {
                    crate::process_midi(chunk, &mut playing);
                    played.insert(playing.clone());
                }
            }

            slice_start += slice_length;
            let slice_end_time = slice_start;
            // sleep until slice_end_time - scheduling_deadline_margin
            let sleep_time = (slice_end_time - scheduling_deadline_margin) - now;
            // use wrapping add to simulate adding a negative number
            now += sleep_time.wrapping_add(wake_up_jitter[slices % wake_up_jitter.len()] as u64);
            slices += 1;
        }
        // assert!(false); // faked failure to see output

        assert!(slice_start == pattern_start + 2 * pattern_length);
        assert_eq!(playing, vec![]);

        let f_major = vec![(0, NOTE_F3), (0, NOTE_A3), (0, NOTE_C4)];
        let c_major = vec![(0, NOTE_C3), (0, NOTE_E3), (0, NOTE_G3)];
        assert!(played.contains(&vec![]));
        assert!(played.contains(&c_major));
        assert!(played.contains(&f_major));
    }

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
            &timed_events,
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
