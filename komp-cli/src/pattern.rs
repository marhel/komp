use crate::play::{Event, TimedEvent};

fn create_note(
    timing: u32,
    length: u32,
    channel: u8,
    note: u8,
    velocity: u8,
) -> (TimedEvent, TimedEvent) {
    (
        TimedEvent {
            timing: timing,
            event: Event::NoteOn {
                channel,
                note,
                velocity,
            },
        },
        TimedEvent {
            timing: timing + length,
            event: Event::NoteOff {
                channel,
                note,
                velocity: 64,
            },
        },
    )
}

#[derive(Clone, Copy, Debug)]
pub struct TimeCode {
    bar: u32,
    beat: u8,
    tick: u32,
}

impl TimeCode {
    pub fn new(bar: u32, beat: u8, tick: u32) -> TimeCode {
        TimeCode { bar, beat, tick }
    }
    pub fn ticks(&self, ticks_per_quarter: u32) -> u32 {
        (self.bar * 4 + self.beat as u32) * ticks_per_quarter + self.tick
    }
}

impl std::ops::Add for TimeCode {
    type Output = Self;
    fn add(self, rhs: TimeCode) -> TimeCode {
        TimeCode {
            bar: self.bar + rhs.bar,
            beat: self.beat + rhs.beat,
            tick: self.tick + rhs.tick,
        }
    }
}

impl std::cmp::PartialEq for TimeCode {
    fn eq(&self, rhs: &TimeCode) -> bool {
        self.ticks(1920) == rhs.ticks(1920)
    }
}

pub fn create_note_part(
    ticks_per_quarter: u32,
    offset: TimeCode,
    part: u8,
    note: u8,
) -> (TimedEvent, TimedEvent) {
    let length = 3 * ticks_per_quarter / part as u32;
    create_note(offset.ticks(ticks_per_quarter), length, 0, note, 120)
}

use komp_core::Chord;
use std::collections::BinaryHeap;

pub fn create_chord_part(
    ticks_per_quarter: u32,
    offset: TimeCode,
    part: u8,
    chord: Chord,
) -> Vec<TimedEvent> {
    let mut heap = BinaryHeap::with_capacity(10);
    for note in chord.notes(3, 0) {
        let (on, off) = create_note_part(ticks_per_quarter, offset, part, note);

        heap.push(on);
        heap.push(off);
    }

    heap.into_sorted_vec()
}

pub fn create_bar(ticks_per_quarter: u32, chord: Chord) -> Vec<TimedEvent> {
    let mut timed_events = vec![];
    for beat in 0..4 {
        let offset = TimeCode::new(0, beat, 0);
        timed_events.append(&mut create_chord_part(ticks_per_quarter, offset, 4, chord));
    }
    timed_events
}

pub fn create_bars(ticks_per_quarter: u32, chords: &[Chord]) -> Vec<TimedEvent> {
    let mut timed_events = vec![];
    for (bar, chord) in chords.iter().enumerate() {
        for beat in 0..4 {
            let offset = TimeCode::new(bar as u32, beat, 0);
            timed_events.append(&mut create_chord_part(ticks_per_quarter, offset, 4, *chord));
        }
    }
    timed_events
}

#[cfg(test)]
mod tests {
    use super::*;
    use komp_core::C_KEY;

    #[test]
    fn test_chord_part_4_timing() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(1, 0, 0);
        let notes = create_chord_part(ticks_per_quarter, offset, 4, Chord::Major(C_KEY));
        let (t1, t2, t3) = (notes[0].timing, notes[1].timing, notes[2].timing);
        assert_eq!(t1, ticks_per_quarter * 4);
        assert!(t1 == t2 && t2 == t3);
        let (t4, t5, t6) = (notes[3].timing, notes[4].timing, notes[5].timing);
        assert!(t4 == t5 && t5 == t6);

        assert!(t1 != t4);
    }

    #[test]
    fn test_chord_part_4_events() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(1, 0, 0);
        let notes = create_chord_part(ticks_per_quarter, offset, 4, Chord::Major(C_KEY));
        if let (
            Event::NoteOn { note: n1, .. },
            Event::NoteOn { note: n2, .. },
            Event::NoteOn { note: n3, .. },
        ) = (notes[0].event, notes[1].event, notes[2].event)
        {
            assert!(n1 < n2 && n2 < n3);
        } else {
            unreachable!()
        }

        if let (
            Event::NoteOff { note: n4, .. },
            Event::NoteOff { note: n5, .. },
            Event::NoteOff { note: n6, .. },
        ) = (notes[3].event, notes[4].event, notes[5].event)
        {
            assert!(n4 < n5 && n5 < n6);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_timecode_add() {
        let one_one_one = TimeCode::new(1, 1, 1);
        let two_two_two = TimeCode::new(2, 2, 2);

        assert_eq!(one_one_one + one_one_one, two_two_two);
    }

    #[test]
    fn test_timecode_ticks_zero() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(0, 0, 0);
        assert_eq!(offset.ticks(ticks_per_quarter), 0);
    }

    #[test]
    fn test_timecode_ticks_one_quarter() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(0, 1, 0);
        assert_eq!(offset.ticks(ticks_per_quarter), ticks_per_quarter);
    }

    #[test]
    fn test_timecode_ticks_four_quarters_to_a_bar() {
        let ticks_per_quarter = 96;
        let offset1 = TimeCode::new(1, 0, 0);
        let offset2 = TimeCode::new(0, 4, 0);
        assert_eq!(
            offset1.ticks(ticks_per_quarter),
            offset2.ticks(ticks_per_quarter)
        );
    }

    #[test]
    fn test_timecode_ticks_96_ticks_to_a_quarter() {
        let ticks_per_quarter = 96;
        let offset1 = TimeCode::new(0, 1, 0);
        let offset2 = TimeCode::new(0, 0, ticks_per_quarter);
        assert_eq!(
            offset1.ticks(ticks_per_quarter),
            offset2.ticks(ticks_per_quarter)
        );
    }

    #[test]
    fn test_note_part_2_timing() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(16, 2, 10);
        let (note_on, note_off) = create_note_part(ticks_per_quarter, offset, 2, 60);
        assert_eq!(note_on.timing, offset.ticks(ticks_per_quarter));
        assert!(note_off.timing > offset.ticks(ticks_per_quarter));
        assert!(note_off.timing - note_on.timing > ticks_per_quarter);
        assert!(note_off.timing - note_on.timing < ticks_per_quarter * 2);
    }

    #[test]
    fn test_note_part_4_timing() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(16, 2, 10);
        let (note_on, note_off) = create_note_part(ticks_per_quarter, offset, 4, 60);
        assert_eq!(note_on.timing, offset.ticks(ticks_per_quarter));
        assert!(note_off.timing > offset.ticks(ticks_per_quarter));
        assert!(note_off.timing - note_on.timing > ticks_per_quarter / 2);
        assert!(note_off.timing - note_on.timing < ticks_per_quarter);
    }

    #[test]
    fn test_note_part_8_timing() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(16, 2, 10);
        let (note_on, note_off) = create_note_part(ticks_per_quarter, offset, 8, 60);
        assert_eq!(note_on.timing, offset.ticks(ticks_per_quarter));
        assert!(note_off.timing > offset.ticks(ticks_per_quarter));
        assert!(note_off.timing - note_on.timing > ticks_per_quarter / 4);
        assert!(note_off.timing - note_on.timing < ticks_per_quarter / 2);
    }

    #[test]
    fn test_note_creation_note_on() {
        let (note_on, _) = create_note(80, 10, 0, 100, 120);

        assert_eq!(note_on.timing, 80);
        match note_on.event {
            Event::NoteOn {
                channel,
                note,
                velocity,
            } => {
                assert_eq!(channel, 0);
                assert_eq!(note, 100);
                assert_eq!(velocity, 120);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_note_creation_note_off() {
        let (_, note_off) = create_note(80, 10, 0, 100, 120);

        assert_eq!(note_off.timing, 80 + 10);
        match note_off.event {
            Event::NoteOff {
                channel,
                note,
                velocity,
            } => {
                assert_eq!(channel, 0);
                assert_eq!(note, 100);
                assert_eq!(velocity, 64);
            }
            _ => unreachable!(),
        }
    }
}
