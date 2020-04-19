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
    use crate::pattern::*;
    use komp_core::C_KEY;

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

        let mut timings = vec![];
        for packet in packet_buf.iter() {
            timings.push(packet.timestamp());
        }

        timings.sort();
        timings.dedup();
        let ns_per_ms = 1_000_000;

        // even timings are note ons, odds are note offs
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
