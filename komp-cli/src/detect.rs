/** The Chord Change DSL.
- space separates parallel steps, assumed to be of the same duration
- [] brackets means time passes between each step,
- _ means keep the previous key pressed,
- \- means to release the previous key,
- key changes within brackets imply that the previous key is to be released first.

So "C" means pressing the C key
"C G" means pressing the C and G key together,
but "[C G]" means to first press C,
then after some unspecified time to release it and press G.

Since each step is assumed to be the same duration, we can write "C [E Eb] G" to
describe a chord change from C major to C minor. This expression is interpreted to mean
that initially C E and G is pressed down together, then after some unspecified time,
E is released and Eb is pressed instead, while C and G are kept pressed down.

The more complicated expression "[C _] [E Eb] [G _]" would mean the same thing,
as would "[C _ _] [E - Eb] [G _ _]"

The expression "[C C] [E Eb] [G G]" would also move from C major to C minor,
but in a different way - we first press C E G together, and the release all keys before
pressing C Eb G together.

"C E G [Bb -]" means going from a C7 to a C, by releasing one key
starting with C E G Bb, and then releasing Bb while keeping the other pressed.
"C E G [- Bb]" means going from a C to a C7 (adding one key)

"[C - -] [E _ -] [G _ _] [- B _] [- - D]" means going from C to Em to G,
first pressing C E and G together,
then releasing C and pressing B (while keeping G and E pressed),
then releasing E and pressing D (while keeping G and B pressed)

This can more clearly be seen if we add line breaks;

T1 2 3
[C - -]
[E _ -]
[G _ _]
[- B _]
[- - D]

At T1, we press C E and G, at T2 we release C and press B, at T3 we release E and press D.
*/

type Step = (Option<u8>, u8);
type Part = Vec<Step>;

fn split_parts(chord_change_dsl: &str) -> Vec<Vec<&str>> {
    let mut start = 0;
    let mut in_chunk = false;
    let mut was_chunk = false;
    let mut was_space = true;
    let mut res = vec![];
    let chunk_len = |i: usize, was_chunk: bool| if was_chunk { i - 1 } else { i };
    for (i, c) in chord_change_dsl.chars().enumerate() {
        if c == '[' {
            start = i + 1;
            in_chunk = true;
        }
        if c == ']' {
            in_chunk = false;
            was_chunk = true;
        }
        if c == ' ' {
            if !in_chunk {
                if !was_space {
                    res.push(steps(&chord_change_dsl[start..chunk_len(i, was_chunk)]));
                }
                start = i + 1;
            }
            was_chunk = false;
            was_space = true;
        }
        if c != ' ' {
            was_space = false;
        }
    }
    // add last segment
    if !in_chunk && start < chord_change_dsl.len() {
        res.push(steps(
            &chord_change_dsl[start..chunk_len(chord_change_dsl.len(), was_chunk)],
        ));
    }
    res
}

fn steps(chord_change: &str) -> Vec<&str> {
    chord_change.split_whitespace().collect()
}

fn interpret(chord_steps: Vec<&str>) -> Part {
    let mut res = vec![];
    for v in chord_steps {
        match v {
            "C" => res.push((Some(komp_core::NOTE_C4), 1)),
            "C#" => res.push((Some(komp_core::NOTE_CSHARP4), 1)),
            "Db" => res.push((Some(komp_core::NOTE_DFLAT4), 1)),
            "D" => res.push((Some(komp_core::NOTE_D4), 1)),
            "D#" => res.push((Some(komp_core::NOTE_DSHARP4), 1)),
            "Eb" => res.push((Some(komp_core::NOTE_EFLAT4), 1)),
            "E" => res.push((Some(komp_core::NOTE_E4), 1)),
            "F" => res.push((Some(komp_core::NOTE_F4), 1)),
            "F#" => res.push((Some(komp_core::NOTE_FSHARP4), 1)),
            "Gb" => res.push((Some(komp_core::NOTE_GFLAT4), 1)),
            "G" => res.push((Some(komp_core::NOTE_G4), 1)),
            "G#" => res.push((Some(komp_core::NOTE_GSHARP4), 1)),
            "Ab" => res.push((Some(komp_core::NOTE_AFLAT4), 1)),
            "A" => res.push((Some(komp_core::NOTE_A4), 1)),
            "A#" => res.push((Some(komp_core::NOTE_ASHARP4), 1)),
            "Bb" => res.push((Some(komp_core::NOTE_BFLAT4), 1)),
            "B" => res.push((Some(komp_core::NOTE_B4), 1)),
            "-" => res.push((None, 1)),
            "_" => {
                if let Some(last) = res.last_mut() {
                    *last = (last.0, last.1 + 1)
                }
            }
            _ => (),
        }
    }
    res
}

fn interpret_dsl(chord_change_dsl: &str) -> Vec<Part> {
    let parts = split_parts(chord_change_dsl);
    let mut interpreted_parts = vec![];
    for steps in parts {
        interpreted_parts.push(interpret(steps));
    }

    fn sum_len(s: &Part) -> u8 {
        s.iter().map(|i| i.1).sum()
    }

    let max_len: u8 = interpreted_parts
        .iter()
        .map(sum_len)
        .max()
        .expect("some steps");
    for steps in interpreted_parts.iter_mut() {
        let current_len: u8 = sum_len(steps);
        if let Some(item) = steps.last_mut() {
            *item = (item.0, max_len - current_len + item.1);
        }
    }

    interpreted_parts
}
use crate::pattern::{create_note, TimeCode};
use crate::play::TimedEvent;

fn step_to_timed_midi_events(
    step: Step,
    offset: TimeCode,
    ticks_per_quarter: u32,
) -> (Vec<TimedEvent>, TimeCode) {
    let (maybe_note, number_of_bars) = step;
    let end_time = offset + TimeCode::new(number_of_bars.into(), 0, 0);
    if let Some(note) = maybe_note {
        let last_bar_ticks = 3 * ticks_per_quarter;
        let additional_bar_ticks = (number_of_bars as u32 - 1) * 4 * ticks_per_quarter;
        let length = additional_bar_ticks + last_bar_ticks;
        let (on, off) = create_note(offset.ticks(ticks_per_quarter), length, 0, note, 100);
        (vec![on, off], end_time)
    } else {
        (vec![], end_time)
    }
}

fn part_to_timed_midi_events(
    part: &Part,
    offset: TimeCode,
    ticks_per_quarter: u32,
) -> Vec<TimedEvent> {
    let mut part_events = vec![];
    let mut current_time = offset;
    for step in part.iter() {
        let (mut step_events, end_part) =
            step_to_timed_midi_events(*step, current_time, ticks_per_quarter);
        current_time = end_part;
        part_events.append(&mut step_events);
    }

    part_events
}

use std::collections::BinaryHeap;

fn parts_to_timed_midi_events(
    parts: &Vec<Part>,
    offset: TimeCode,
    ticks_per_quarter: u32,
) -> Vec<TimedEvent> {
    let mut all_events = BinaryHeap::with_capacity(10);
    for part in parts.iter() {
        let part_events = part_to_timed_midi_events(part, offset, ticks_per_quarter);
        all_events.extend(part_events);
    }

    all_events.into_sorted_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use komp_core::NOTE_A4;
    use komp_core::NOTE_AFLAT4;
    use komp_core::NOTE_ASHARP4;
    use komp_core::NOTE_B4;
    use komp_core::NOTE_BFLAT4;
    use komp_core::NOTE_C4;
    use komp_core::NOTE_CSHARP4;
    use komp_core::NOTE_D4;
    use komp_core::NOTE_DFLAT4;
    use komp_core::NOTE_DSHARP4;
    use komp_core::NOTE_E4;
    use komp_core::NOTE_EFLAT4;
    use komp_core::NOTE_F4;
    use komp_core::NOTE_FSHARP4;
    use komp_core::NOTE_G4;
    use komp_core::NOTE_GFLAT4;
    use komp_core::NOTE_GSHARP4;

    #[test]
    fn test_chord_change_dsl_split() {
        let changes = [
            // C, no changes
            ("C E G", vec![vec!["C"], vec!["E"], vec!["G"]]),
            // C7 to C
            (
                "C E G [Bb -] ",
                vec![vec!["C"], vec!["E"], vec!["G"], vec!["Bb", "-"]],
            ),
            // C to Cm
            (
                "  C  [ E   Eb ]   G  ",
                vec![vec!["C"], vec!["E", "Eb"], vec!["G"]],
            ),
            // C to C7
            (
                "C E G [- Bb]",
                vec![vec!["C"], vec!["E"], vec!["G"], vec!["-", "Bb"]],
            ),
            // C to C
            ("[C C] E G", vec![vec!["C", "C"], vec!["E"], vec!["G"]]),
            // C7 to nothing, but keep C7
            (
                "[C -] [E -] [G -] [Bb -]",
                vec![
                    vec!["C", "-"],
                    vec!["E", "-"],
                    vec!["G", "-"],
                    vec!["Bb", "-"],
                ],
            ),
            // C to Em to G
            (
                "[C - -] [E _ -] [G _ _] [- B _] [- - D]",
                vec![
                    vec!["C", "-", "-"],
                    vec!["E", "_", "-"],
                    vec!["G", "_", "_"],
                    vec!["-", "B", "_"],
                    vec!["-", "-", "D"],
                ],
            ),
            // C to F
            (
                "C [E F] [G A]",
                vec![vec!["C"], vec!["E", "F"], vec!["G", "A"]],
            ),
            // C to F
            (
                "[C C] [E F] [G A]",
                vec![vec!["C", "C"], vec!["E", "F"], vec!["G", "A"]],
            ),
            // C to F
            (
                "[C - C] [E - F] [G - A]",
                vec![
                    vec!["C", "-", "C"],
                    vec!["E", "-", "F"],
                    vec!["G", "-", "A"],
                ],
            ),
            // C to F
            (
                "[C F] [E A] [G C']",
                vec![vec!["C", "F"], vec!["E", "A"], vec!["G", "C'"]],
            ),
        ];

        for (chord_change, components) in changes.iter() {
            // println!("{:?}", split(*chord_change));
            assert!(
                split_parts(*chord_change).eq(components),
                "{}",
                *chord_change
            );
        }
    }

    #[test]
    fn test_chord_change_dsl_steps() {
        let changes = [
            ("C", 1),
            ("C  -", 2),
            ("C D ", 2),
            (" C - D ", 3),
            ("  C   D   E  ", 3),
        ];

        for (change, len) in changes.iter() {
            assert_eq!(steps(*change).len(), *len as usize, "{}", *change);
        }
    }

    macro_rules! test_interpreter {
        ($name:ident, $change:expr, $il:expr) => {
            #[test]
            fn $name() {
                let steps = steps($change);
                let res = interpret(steps);
                let il = $il;
                assert!(
                    res.eq(&il),
                    "{} became {:?} expected {:?}",
                    $change,
                    res,
                    &il
                );
            }
        };
    }

    test_interpreter!(simple_c, "C", vec![(Some(NOTE_C4), 1)]);
    test_interpreter!(simple_csharp, "C#", vec![(Some(NOTE_CSHARP4), 1)]);
    test_interpreter!(simple_dflat, "Db", vec![(Some(NOTE_DFLAT4), 1)]);
    test_interpreter!(simple_d, "D", vec![(Some(NOTE_D4), 1)]);
    test_interpreter!(simple_dsharp, "D#", vec![(Some(NOTE_DSHARP4), 1)]);
    test_interpreter!(simple_eflat, "Eb", vec![(Some(NOTE_EFLAT4), 1)]);
    test_interpreter!(simple_e, "E", vec![(Some(NOTE_E4), 1)]);
    test_interpreter!(simple_f, "F", vec![(Some(NOTE_F4), 1)]);
    test_interpreter!(simple_fsharp, "F#", vec![(Some(NOTE_FSHARP4), 1)]);
    test_interpreter!(simple_gflat, "Gb", vec![(Some(NOTE_GFLAT4), 1)]);
    test_interpreter!(simple_g, "G", vec![(Some(NOTE_G4), 1)]);
    test_interpreter!(simple_gsharp, "G#", vec![(Some(NOTE_GSHARP4), 1)]);
    test_interpreter!(simple_aflat, "Ab", vec![(Some(NOTE_AFLAT4), 1)]);
    test_interpreter!(simple_a, "A", vec![(Some(NOTE_A4), 1)]);
    test_interpreter!(simple_asharp, "A#", vec![(Some(NOTE_ASHARP4), 1)]);
    test_interpreter!(simple_bflat, "Bb", vec![(Some(NOTE_BFLAT4), 1)]);
    test_interpreter!(simple_b, "B", vec![(Some(NOTE_B4), 1)]);
    test_interpreter!(c_len2, "C _", vec![(Some(NOTE_C4), 2)]);
    test_interpreter!(c_len3, "C _ _", vec![(Some(NOTE_C4), 3)]);
    test_interpreter!(
        double_c,
        "C C",
        vec![(Some(NOTE_C4), 1), (Some(NOTE_C4), 1)]
    );
    test_interpreter!(
        c_len2_c,
        "C _ C",
        vec![(Some(NOTE_C4), 2), (Some(NOTE_C4), 1)]
    );
    test_interpreter!(
        c_len3_c,
        "C _ _ C",
        vec![(Some(NOTE_C4), 3), (Some(NOTE_C4), 1)]
    );
    test_interpreter!(
        c_len2_c_len2,
        "C _ C _",
        vec![(Some(NOTE_C4), 2), (Some(NOTE_C4), 2)]
    );
    test_interpreter!(c_silence, "C -", vec![(Some(NOTE_C4), 1), (None, 1)]);
    test_interpreter!(
        c_silence_c,
        "C - C",
        vec![(Some(NOTE_C4), 1), (None, 1), (Some(NOTE_C4), 1)]
    );
    test_interpreter!(
        c_silence_silence_c,
        "C - - C",
        vec![(Some(NOTE_C4), 1), (None, 1), (None, 1), (Some(NOTE_C4), 1)]
    );
    test_interpreter!(
        c_silence_len2_c,
        "C - _ C",
        vec![(Some(NOTE_C4), 1), (None, 2), (Some(NOTE_C4), 1)]
    );
    test_interpreter!(
        c_silence_d,
        "C - D",
        vec![(Some(NOTE_C4), 1), (None, 1), (Some(NOTE_D4), 1)]
    );
    test_interpreter!(
        c_d_e,
        "C D E",
        vec![(Some(NOTE_C4), 1), (Some(NOTE_D4), 1), (Some(NOTE_E4), 1)]
    );

    macro_rules! test_interpret_dsl {
        ($name:ident, $change_dsl:expr, $il:expr) => {
            #[test]
            fn $name() {
                let interpreted_parts = interpret_dsl($change_dsl);
                let il = $il;
                assert!(
                    interpreted_parts.eq(&il),
                    "{} \nbecame   {:?}\nexpected {:?}",
                    $change_dsl,
                    interpreted_parts,
                    &il
                );
            }
        };
    }

    test_interpret_dsl!(
        c_to_cm_len2,
        "C [E Eb] G",
        vec![
            vec![(Some(NOTE_C4), 2)],
            vec![(Some(NOTE_E4), 1), (Some(NOTE_EFLAT4), 1)],
            vec![(Some(NOTE_G4), 2)]
        ]
    );

    test_interpret_dsl!(
        c_to_cm_len3,
        "C [E - Eb] [G -]",
        vec![
            vec![(Some(NOTE_C4), 3)],
            vec![(Some(NOTE_E4), 1), (None, 1), (Some(NOTE_EFLAT4), 1)],
            vec![(Some(NOTE_G4), 1), (None, 2)]
        ]
    );

    test_interpret_dsl!(
        c_to_cm_len3_variant2,
        "C [E _ Eb] [G _]",
        vec![
            vec![(Some(NOTE_C4), 3)],
            vec![(Some(NOTE_E4), 2), (Some(NOTE_EFLAT4), 1)],
            vec![(Some(NOTE_G4), 3)]
        ]
    );

    test_interpret_dsl!(
        c_to_cm_len3_variant3,
        "C [E Eb _] [G -]",
        vec![
            vec![(Some(NOTE_C4), 3)],
            vec![(Some(NOTE_E4), 1), (Some(NOTE_EFLAT4), 2)],
            vec![(Some(NOTE_G4), 1), (None, 2)]
        ]
    );

    use crate::play::Event::*;
    use crate::play::TimedEvent;

    #[test]
    fn test_silent_step_to_timed_midi_events() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(1, 0, 0);
        let expected_end_part = TimeCode::new(4, 0, 0);
        let (data, end_part) = step_to_timed_midi_events((None, 3), offset, ticks_per_quarter);

        assert_eq!(end_part, expected_end_part);
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_note_step_to_timed_midi_events() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(1, 0, 0);
        let expected_end_note = TimeCode::new(3, 3, 0);
        let expected_end_part = TimeCode::new(4, 0, 0);
        let (data, end_part) =
            step_to_timed_midi_events((Some(NOTE_C4), 3), offset, ticks_per_quarter);

        assert_eq!(end_part, expected_end_part);
        assert_eq!(
            data,
            vec![
                TimedEvent {
                    timing: offset.ticks(ticks_per_quarter),
                    event: NoteOn {
                        channel: 0,
                        note: NOTE_C4,
                        velocity: 100,
                    }
                },
                TimedEvent {
                    timing: expected_end_note.ticks(ticks_per_quarter),
                    event: NoteOff {
                        channel: 0,
                        note: 60,
                        velocity: 64
                    }
                }
            ]
        );
    }

    #[test]
    fn test_part_to_timed_midi_events() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(1, 0, 0);
        let parts = vec![(Some(NOTE_E4), 1), (None, 1), (Some(NOTE_EFLAT4), 1)];
        let events = part_to_timed_midi_events(&parts, offset, ticks_per_quarter);

        assert_eq!(events.len(), 4);
    }

    #[test]
    fn test_parts_to_timed_midi_events() {
        let ticks_per_quarter = 96;
        let offset = TimeCode::new(1, 0, 0);
        let parts = vec![
            vec![(Some(NOTE_C4), 2)],
            vec![(Some(NOTE_E4), 1), (Some(NOTE_EFLAT4), 1)],
            vec![(Some(NOTE_G4), 2)],
        ];

        let events = parts_to_timed_midi_events(&parts, offset, ticks_per_quarter);

        assert_eq!(events.len(), 8);
    }
}
