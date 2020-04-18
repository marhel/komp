#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct TimedEvent {
    pub timing: u32,
    pub event: Event,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum Event {
    Rest,
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8, velocity: u8 },
}

use komp_core::*;

fn setup_events(channel: u8) -> Vec<Event> {
    let mut events = Vec::new();
    events.push(Event::NoteOn {
        channel,
        note: NOTE_C3,
        velocity: 96,
    });
    events.push(Event::NoteOn {
        channel,
        note: NOTE_E3,
        velocity: 96,
    });
    events.push(Event::NoteOn {
        channel,
        note: NOTE_G3,
        velocity: 96,
    });
    events.push(Event::NoteOff {
        channel,
        note: NOTE_C3,
        velocity: 64,
    });
    events.push(Event::NoteOff {
        channel,
        note: NOTE_E3,
        velocity: 64,
    });
    events.push(Event::NoteOff {
        channel,
        note: NOTE_G3,
        velocity: 64,
    });
    events
}

fn distribute(events: Vec<Event>, ppq: u32, len: u8) -> Vec<TimedEvent> {
    let mut timed_events = vec![];
    for quarter in 0..4 {
        timed_events.push(TimedEvent {
            timing: quarter * ppq,
            event: events[0],
        });
        timed_events.push(TimedEvent {
            timing: quarter * ppq,
            event: events[1],
        });
        timed_events.push(TimedEvent {
            timing: quarter * ppq,
            event: events[2],
        });
        timed_events.push(TimedEvent {
            timing: quarter * ppq + len as u32,
            event: events[3],
        });
        timed_events.push(TimedEvent {
            timing: quarter * ppq + len as u32,
            event: events[4],
        });
        timed_events.push(TimedEvent {
            timing: quarter * ppq + len as u32,
            event: events[5],
        });
    }
    timed_events
}

pub fn schedule(
    offset: u64,
    timed_events: Vec<TimedEvent>,
    key: Key,
    ms_per_quarter: u32,
    ticks_per_quarter: u32,
) -> coremidi::PacketBuffer {
    let mut packet_buf = coremidi::PacketBuffer::with_capacity(512);

    for te in timed_events.iter() {
        let data = match te.event {
            Event::Rest => continue,
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
        let ns_per_ms = 1_000_000;
        packet_buf.push_data(
            offset
                + (ns_per_ms * te.timing as u64 * ms_per_quarter as u64 / ticks_per_quarter as u64),
            &data,
        );
    }

    packet_buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::external::{AudioConvertHostTimeToNanos, AudioGetCurrentHostTime};
    use crate::pattern::*;
    use komp_core::C_KEY;

    fn create_packets(
        ticks_per_quarter: u32,
        ms_per_quarter: u32,
    ) -> (u64, coremidi::PacketBuffer) {
        let timed_events = create_bar(ticks_per_quarter, Chord::Major(C_KEY));
        let timestamp = unsafe { AudioConvertHostTimeToNanos(AudioGetCurrentHostTime()) };
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

        let mut timings = vec![];
        for packet in packet_buf.iter() {
            timings.push(packet.timestamp());
        }

        timings.sort();
        timings.dedup();
        let ns_per_ms = 1_000_000;

        assert_eq!(timings[0] - timestamp, 0 as u64);
        assert_eq!(
            timings[2] - timestamp,
            1 * ms_per_quarter as u64 * ns_per_ms
        );
        assert_eq!(
            timings[4] - timestamp,
            2 * ms_per_quarter as u64 * ns_per_ms
        );
        assert_eq!(
            timings[6] - timestamp,
            3 * ms_per_quarter as u64 * ns_per_ms
        );
    }

    #[test]
    fn test_chord_part_scheduled_timing_lores() {
        let ticks_per_quarter = 16;
        let ms_per_quarter = 500_000;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }

    #[test]
    fn test_chord_part_scheduled_timing_lores_slow() {
        let ticks_per_quarter = 16;
        let ms_per_quarter = 500_000_000;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }

    #[test]
    fn test_chord_part_scheduled_timing_hires() {
        let ticks_per_quarter = 96_000;
        let ms_per_quarter = 500_000;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }

    #[test]
    fn test_chord_part_scheduled_timing_hires_fast() {
        let ticks_per_quarter = 96_000;
        let ms_per_quarter = 500;
        let (timestamp, packet_buf) = create_packets(ticks_per_quarter, ms_per_quarter);

        assert_timings(packet_buf, timestamp, ms_per_quarter)
    }
}
