/** The Chord Change DSL
space separates parallel steps, assumed to be of the same duration
brackets means time passes between each step,
_ means keep the previous key pressed,
- means to release the previous key,
key changes within brackets imply that the previous key is to be released first.

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

fn split(chord_change_dsl: &str) -> Vec<Vec<&str>> {
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
                    res.push(parts(&chord_change_dsl[start..chunk_len(i, was_chunk)]));
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
        res.push(parts(
            &chord_change_dsl[start..chunk_len(chord_change_dsl.len(), was_chunk)],
        ));
    }
    res
}

fn parts(chord_change: &str) -> Vec<&str> {
    chord_change.split_whitespace().collect()
}

fn interpret(chord_change: &str) -> Vec<(u8, u8)> {
    vec![(crate::key::NOTE_C4, 1)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::*;

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
            assert!(split(*chord_change).eq(components), "{}", *chord_change);
        }
    }

    #[test]
    fn test_chord_change_dsl_parts() {
        let changes = [
            ("C", 1),
            ("C  -", 2),
            ("C D ", 2),
            (" C - D ", 3),
            ("  C   D   E  ", 3),
        ];

        for (change, len) in changes.iter() {
            assert_eq!(parts(*change).len(), *len as usize, "{}", *change);
        }
    }

    #[test]
    fn test_chord_change_dsl_interpreter() {
        let press = |n| (n, 1);
        let press_l = |n, l| (n, l);
        let silence = || (0, 1);
        let changes = [
            ("C", vec![press(NOTE_C4)]),
            ("C _", vec![press_l(NOTE_C4, 2)]),
            ("C _ _", vec![press_l(NOTE_C4, 3)]),
            ("C C", vec![press(NOTE_C4), press(NOTE_C4)]),
            ("C _ C", vec![press_l(NOTE_C4, 2), press(NOTE_C4)]),
            ("C _ _ C", vec![press_l(NOTE_C4, 3), press(NOTE_C4)]),
            ("C -", vec![press(NOTE_C4), silence()]),
            ("C - C", vec![press(NOTE_C4), silence(), press(NOTE_C4)]),
            ("C D ", vec![press(NOTE_C4), press(NOTE_D4)]),
            (" C - D ", vec![press(NOTE_C4), silence(), press(NOTE_D4)]),
            ("  C   D   E  ", vec![press(NOTE_C4), press(NOTE_D4), press(NOTE_E4)]),
        ];

        for (change, il) in changes.iter() {
            assert!(interpret(*change).eq(il), "{}", *change);
        }
    }
}
