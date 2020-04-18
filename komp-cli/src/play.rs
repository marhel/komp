#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct TimedEvent {
    timing: u32,
    event: Event,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
enum Event {
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

pub fn schedule_music(timestamp: u64, key: Key) -> coremidi::PacketBuffer {
    let events = setup_events(1);
    let ppq = 96;
    let tes = distribute(events, ppq, 80);

    let mut packet_buf = coremidi::PacketBuffer::with_capacity(512);
    let ms_per_quarter = 500_000;

    let us_per_tick = 1000 * ms_per_quarter / ppq;

    for te in tes.iter() {
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

        packet_buf.push_data(timestamp + te.timing as u64, &data);
    }
    packet_buf
}
