use crate::key::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Chord {
    None(Key),
    Major(Key),
    Minor(Key),
    Aug(Key),
    Dim(Key),
    Dim7(Key),
    Sus2(Key),
    Sus4(Key),
    Five(Key),
    SevenSus4(Key),
    Major6(Key),
    Minor6(Key),
    Major6_9(Key),
    Minor6_9(Key),
    Major7(Key),
    Major7_9(Key),
    Major7b9(Key),
    Major7Plus9(Key),
    Major7Plus11(Key),
    Major7b13(Key),
    Major7_13(Key),
    Major7Aug(Key),
    Minor7(Key),
    Minor7_9(Key),
    Minor7_11(Key),
    Major7b5(Key),
    Minor7b5(Key),
    MajorMaj7(Key),
    MajorMaj7_9(Key),
    MajorMaj7Plus11(Key),
    MajorMaj7Aug(Key),
    MinorMaj7(Key),
    MinorMaj7_9(Key),
    MajorAdd9(Key),
    MinorAdd9(Key),
}

impl Chord {
    pub fn key(&self) -> &Key {
        match self {
            Chord::None(k) => k,
            Chord::Major(k) => k,
            Chord::Minor(k) => k,
            Chord::Aug(k) => k,
            Chord::Dim(k) => k,
            Chord::Dim7(k) => k,
            Chord::Sus2(k) => k,
            Chord::Sus4(k) => k,
            Chord::Five(k) => k,
            Chord::SevenSus4(k) => k,
            Chord::Major6(k) => k,
            Chord::Minor6(k) => k,
            Chord::Major6_9(k) => k,
            Chord::Minor6_9(k) => k,
            Chord::Major7(k) => k,
            Chord::Major7_9(k) => k,
            Chord::Major7b9(k) => k,
            Chord::Major7Plus9(k) => k,
            Chord::Major7Plus11(k) => k,
            Chord::Major7b13(k) => k,
            Chord::Major7_13(k) => k,
            Chord::Major7Aug(k) => k,
            Chord::Minor7(k) => k,
            Chord::Minor7_9(k) => k,
            Chord::Minor7_11(k) => k,
            Chord::Major7b5(k) => k,
            Chord::Minor7b5(k) => k,
            Chord::MajorMaj7(k) => k,
            Chord::MajorMaj7_9(k) => k,
            Chord::MajorMaj7Plus11(k) => k,
            Chord::MajorMaj7Aug(k) => k,
            Chord::MinorMaj7(k) => k,
            Chord::MinorMaj7_9(k) => k,
            Chord::MajorAdd9(k) => k,
            Chord::MinorAdd9(k) => k,
        }
    }

    fn template(&self) -> &'static [u8] {
        match self {
            Chord::None(_) => NONE,
            Chord::Major(_) => MAJOR,
            Chord::Minor(_) => MINOR,
            Chord::Aug(_) => AUG,
            Chord::Dim(_) => DIM,
            Chord::Dim7(_) => DIM7,
            Chord::Sus2(_) => SUS2,
            Chord::Sus4(_) => SUS4,
            Chord::Five(_) => FIVE,
            Chord::SevenSus4(_) => SEVENSUS4,
            Chord::Major6(_) => MAJOR6,
            Chord::Minor6(_) => MINOR6,
            Chord::Major6_9(_) => MAJOR6_9,
            Chord::Minor6_9(_) => MINOR6_9,
            Chord::Major7(_) => MAJOR7,
            Chord::Major7_9(_) => MAJOR7_9,
            Chord::Major7b9(_) => MAJOR7B9,
            Chord::Major7Plus9(_) => MAJOR7PLUS9,
            Chord::Major7Plus11(_) => MAJOR7PLUS11,
            Chord::Major7b13(_) => MAJOR7B13,
            Chord::Major7_13(_) => MAJOR7_13,
            Chord::Major7Aug(_) => MAJOR7AUG,
            Chord::Minor7(_) => MINOR7,
            Chord::Minor7_9(_) => MINOR7_9,
            Chord::Minor7_11(_) => MINOR7_11,
            Chord::Major7b5(_) => MAJOR7B5,
            Chord::Minor7b5(_) => MINOR7B5,
            Chord::MajorMaj7(_) => MAJORMAJ7,
            Chord::MajorMaj7_9(_) => MAJORMAJ7_9,
            Chord::MajorMaj7Plus11(_) => MAJORMAJ7PLUS11,
            Chord::MajorMaj7Aug(_) => MAJORMAJ7AUG,
            Chord::MinorMaj7(_) => MINORMAJ7,
            Chord::MinorMaj7_9(_) => MINORMAJ7_9,
            Chord::MajorAdd9(_) => MAJORADD9,
            Chord::MinorAdd9(_) => MINORADD9,
        }
    }

    fn matches(&self, proposed: &Vec<u8>) -> bool {
        self.template().len() == proposed.len()
            && self
                .template()
                .iter()
                .zip(proposed.iter())
                .all(|(a, b)| a == b)
    }

    pub fn notes(&self, octave: u8, mut inversion: u8) -> Vec<u8> {
        let mut notes: Vec<u8> = match self.key() {
            Key(k) => self
                .template()
                .iter()
                .map(|offset| {
                    let invert = if inversion > 0 {
                        inversion -= 1;
                        1
                    } else {
                        0
                    };
                    offset + k + (1 + octave + invert) * OCTAVE_STEPS
                })
                .collect(),
        };
        notes.sort();
        notes
    }
}

const NONE: &[u8] = &[NOTE_C];
const MAJOR: &[u8] = &[NOTE_C, NOTE_E, NOTE_G];
const MINOR: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_G];
const AUG: &[u8] = &[NOTE_C, NOTE_E, NOTE_AFLAT];
const DIM: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_FSHARP];
const DIM7: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_FSHARP, NOTE_A];
const SUS2: &[u8] = &[NOTE_C, NOTE_D, NOTE_G];
const SUS4: &[u8] = &[NOTE_C, NOTE_F, NOTE_G];
const FIVE: &[u8] = &[NOTE_C, NOTE_G];
const SEVENSUS4: &[u8] = &[NOTE_C, NOTE_F, NOTE_G, NOTE_BFLAT];
const MAJOR6: &[u8] = &[NOTE_C, NOTE_E, NOTE_G, NOTE_A];
const MINOR6: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_G, NOTE_A];
const MAJOR6_9: &[u8] = &[NOTE_C, NOTE_D, NOTE_E, NOTE_G, NOTE_A];
const MINOR6_9: &[u8] = &[NOTE_C, NOTE_D, NOTE_EFLAT, NOTE_G, NOTE_A];
const MAJOR7: &[u8] = &[NOTE_C, NOTE_E, NOTE_G, NOTE_BFLAT];
const MAJOR7B9: &[u8] = &[NOTE_C, NOTE_DFLAT, NOTE_E, NOTE_G, NOTE_BFLAT];
const MAJOR7_9: &[u8] = &[NOTE_C, NOTE_D, NOTE_E, NOTE_G, NOTE_BFLAT];
const MAJOR7PLUS9: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_E, NOTE_G, NOTE_BFLAT];
const MAJOR7PLUS11: &[u8] = &[NOTE_C, NOTE_E, NOTE_FSHARP, NOTE_G, NOTE_BFLAT];
const MAJOR7B13: &[u8] = &[NOTE_C, NOTE_E, NOTE_G, NOTE_AFLAT, NOTE_BFLAT];
const MAJOR7_13: &[u8] = &[NOTE_C, NOTE_E, NOTE_G, NOTE_A, NOTE_BFLAT];
const MAJOR7AUG: &[u8] = &[NOTE_C, NOTE_E, NOTE_AFLAT, NOTE_BFLAT];
const MINOR7: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_G, NOTE_BFLAT];
const MINOR7_9: &[u8] = &[NOTE_C, NOTE_D, NOTE_EFLAT, NOTE_G, NOTE_BFLAT];
const MINOR7_11: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_F, NOTE_G, NOTE_BFLAT];
const MAJOR7B5: &[u8] = &[NOTE_C, NOTE_E, NOTE_GFLAT, NOTE_BFLAT];
const MINOR7B5: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_GFLAT, NOTE_BFLAT];
const MAJORMAJ7: &[u8] = &[NOTE_C, NOTE_E, NOTE_G, NOTE_B];
const MAJORMAJ7_9: &[u8] = &[NOTE_C, NOTE_D, NOTE_E, NOTE_G, NOTE_B];
const MAJORMAJ7PLUS11: &[u8] = &[NOTE_C, NOTE_E, NOTE_FSHARP, NOTE_G, NOTE_B];
const MAJORMAJ7AUG: &[u8] = &[NOTE_C, NOTE_E, NOTE_AFLAT, NOTE_B];
const MINORMAJ7: &[u8] = &[NOTE_C, NOTE_EFLAT, NOTE_G, NOTE_B];
const MINORMAJ7_9: &[u8] = &[NOTE_C, NOTE_D, NOTE_EFLAT, NOTE_G, NOTE_B];
const MAJORADD9: &[u8] = &[NOTE_C, NOTE_D, NOTE_E, NOTE_G];
const MINORADD9: &[u8] = &[NOTE_C, NOTE_D, NOTE_EFLAT, NOTE_G];

fn chord_template(mut chord: Vec<u8>) -> (Key, Vec<u8>) {
    chord.sort();
    let root = chord.first().expect("empty-chord");
    let key = Key(root % OCTAVE_STEPS);
    let mut template: Vec<u8> = chord
        .iter()
        .map(|note| (note - root) % OCTAVE_STEPS)
        .collect();
    template.sort();
    template.dedup();
    (key, template)
}

fn generate_templates(proposed: &Vec<u8>, include_root: bool) -> Vec<(Key, Vec<u8>)> {
    let mut res = vec![];
    let mut i = proposed.len() - 1;
    let (base_key, mut inversion) = chord_template(proposed.clone());
    if include_root {
        res.push((base_key, inversion.clone()));
    }
    // allow inversions over the top note 127
    while i > 0 {
        let inverted = inversion.remove(0) + OCTAVE_STEPS;
        inversion.push(inverted);
        let (key, variant) = chord_template(inversion.clone());
        res.push((key + base_key, variant.clone()));
        i -= 1;
    }
    res
}

pub fn detect_chord(sounding: &Vec<u8>) -> Vec<Chord> {
    let mut res = vec![];
    let (key, template) = chord_template(sounding.clone());

    macro_rules! add_match {
        ($chord:expr, $temp:ident) => {
            let ch = $chord;
            if ch.matches(&$temp) {
                res.push(ch);
            }
        };
    }

    // some inversions are identical;
    // Am6/F# == F#m7b5
    // A7b5 == D#7b5
    // Am7/C == C6
    // Am7(11) = C6(9)
    // Adim7 == Cdim7 == D#dim7 == F#dim7
    // ASus2 == ESus4
    // AAug == C#Aug == FAug
    add_match!(Chord::Major(key), template);
    add_match!(Chord::Minor(key), template);
    add_match!(Chord::Aug(key), template);
    add_match!(Chord::Dim(key), template);
    add_match!(Chord::Dim7(key), template);
    add_match!(Chord::Sus2(key), template);
    add_match!(Chord::Sus4(key), template);
    add_match!(Chord::Five(key), template);
    add_match!(Chord::SevenSus4(key), template);
    add_match!(Chord::Major6(key), template);
    add_match!(Chord::Minor6(key), template);
    add_match!(Chord::Major6_9(key), template);
    add_match!(Chord::Minor6_9(key), template);
    add_match!(Chord::Major7(key), template);
    add_match!(Chord::Major7_9(key), template);
    add_match!(Chord::Major7b9(key), template);
    add_match!(Chord::Major7Plus9(key), template);
    add_match!(Chord::Major7Plus11(key), template);
    add_match!(Chord::Major7b13(key), template);
    add_match!(Chord::Major7_13(key), template);
    add_match!(Chord::Major7Aug(key), template);
    add_match!(Chord::Minor7(key), template);
    add_match!(Chord::Minor7_9(key), template);
    add_match!(Chord::Minor7_11(key), template);
    add_match!(Chord::Major7b5(key), template);
    add_match!(Chord::Minor7b5(key), template);
    add_match!(Chord::MajorMaj7(key), template);
    add_match!(Chord::MajorMaj7_9(key), template);
    add_match!(Chord::MajorMaj7Plus11(key), template);
    add_match!(Chord::MajorMaj7Aug(key), template);
    add_match!(Chord::MinorMaj7(key), template);
    add_match!(Chord::MinorMaj7_9(key), template);
    add_match!(Chord::MajorAdd9(key), template);
    add_match!(Chord::MinorAdd9(key), template);
    add_match!(Chord::None(key), template);

    if res.len() == 0 {
        for (key, t) in generate_templates(sounding, false) {
            add_match!(Chord::Major(key), t);
            add_match!(Chord::Minor(key), t);
            // add_match!(Chord::Aug(key), t);
            add_match!(Chord::Dim(key), t);
            // add_match!(Chord::Dim7(key), t);
            // add_match!(Chord::Sus2(key), t);
            add_match!(Chord::Sus4(key), t);
            add_match!(Chord::Five(key), t);
            add_match!(Chord::SevenSus4(key), t);
            // add_match!(Chord::Major6(key), t);
            // add_match!(Chord::Minor6(key), t);
            // add_match!(Chord::Major6_9(key), t);
            add_match!(Chord::Minor6_9(key), t);
            add_match!(Chord::Major7(key), t);
            add_match!(Chord::Major7_9(key), t);
            add_match!(Chord::Major7b9(key), t);
            add_match!(Chord::Major7Plus9(key), t);
            add_match!(Chord::Major7Plus11(key), t);
            add_match!(Chord::Major7b13(key), t);
            add_match!(Chord::Major7_13(key), t);
            add_match!(Chord::Major7Aug(key), t);
            add_match!(Chord::Minor7(key), t);
            add_match!(Chord::Minor7_9(key), t);
            add_match!(Chord::Minor7_11(key), t);
            add_match!(Chord::Major7b5(key), t);
            add_match!(Chord::Minor7b5(key), t);
            add_match!(Chord::MajorMaj7(key), t);
            add_match!(Chord::MajorMaj7_9(key), t);
            add_match!(Chord::MajorMaj7Plus11(key), t);
            add_match!(Chord::MajorMaj7Aug(key), t);
            add_match!(Chord::MinorMaj7(key), t);
            add_match!(Chord::MinorMaj7_9(key), t);
            add_match!(Chord::MajorAdd9(key), t);
            add_match!(Chord::MinorAdd9(key), t);
            add_match!(Chord::None(key), t);
        }
    }

    // Resolve alternate chord interpretations
    if res.len() == 2 {
        match res[0] {
            Chord::Major7b5(_key1) => match res[1] {
                Chord::Major7b5(_key2) => {
                    res.remove(0);
                    ()
                }
                _ => (),
            },
            _ => (),
        };
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! count {
        (@single $_t:tt) => { () };
        // construct an unit array literal (with as many unit literals
        // as tokens the macro is called with) and call the len method
        ($($tts:tt)*) => {<[()]>::len(&[$(count!(@single $tts)),*])};
    }
    macro_rules! test_key {
        ($name:ident, $chord:ident, $key:ident, $tchord:ident, $tkey:ident, $($inversion:literal)*) => {
            #[test]
            fn $name() {
                let chord = Chord::$chord($key);
                let target_chord = Chord::$tchord($key + $tkey);
                // count number of inversion tokens
                let inversion = count!($($inversion)*) as u8;
                assert_eq!(
                    detect_chord(&chord.notes(4, inversion)),
                    vec![target_chord],
                    "inversion {}", inversion
                );
            }
        };
        ($name:ident, $chord:ident, $key:ident, $inversion:literal) => {
            #[test]
            fn $name() {
                let chord = Chord::$chord($key);
                assert_eq!(detect_chord(&chord.notes(4, $inversion)), vec![chord]);
            }
        };
        ($name:ident, $chord:ident, $key:ident, $inversion:literal, $tchord:ident, $tkey:ident) => {
            #[test]
            fn $name() {
                let chord = Chord::$chord($key);
                let target_chord = Chord::$tchord($key + $tkey);
                assert_eq!(
                    detect_chord(&chord.notes(4, $inversion)),
                    vec![target_chord]
                );
            }
        };
    }

    macro_rules! inversion_test {
      ($modname:ident, $chord:ident, $key:ident, all) => {
          mod $modname {
              use super::*;
              test_key!(i0, $chord, $key, 0);
              test_key!(i1, $chord, $key, 1);
              test_key!(i2, $chord, $key, 2);
              test_key!(i3, $chord, $key, 3);
              test_key!(i4, $chord, $key, 4);
          }
      };
      (@recursive $chord:ident, $key:ident, $($inversion:literal)*, [ $iname:ident => $tchord:ident/$tkey:ident ]) => {
          // keep track of inversion number by passing on that many tokens
          test_key!($iname, $chord, $key, $tchord, $tkey, $($inversion)*);
      };
      (@recursive $chord:ident, $key:ident, $($inversion:literal)*, [ $iname:ident => $tchord:ident/$tkey:ident, $($riname:ident => $rtchord:ident/$rtkey:ident),* ]) => {
          // keep track of inversion number by passing on that many tokens
          test_key!($iname, $chord, $key, $tchord, $tkey, $($inversion)*);
          // and add one more 1u8 token for the next recursion
          inversion_test!(@recursive $chord, $key, $($inversion)* 1u8, [ $($riname => $rtchord/$rtkey),* ]);
      };
      ($modname:ident, $chord:ident, $key:ident, [ $($iname:ident => $tchord:ident/$tkey:ident),* ]) => {
          mod $modname {
              use super::*;
              test_key!(i0, $chord, $key, 0);
              // keep track of inversion number by passing on that many tokens, here a single 1u8
              inversion_test!(@recursive $chord, $key, 1u8, [ $($iname => $tchord/$tkey),* ]);
          }
      };
  }

    macro_rules! detect {
      ($($name:ident : $chord:ident $($type:tt)/+),+) => {
          $(
              mod $name {
                  use super::*;
                  inversion_test!(c_key, $chord, C_KEY, $($type)*);
                  inversion_test!(csharp_key, $chord, CSHARP_KEY, $($type)*);
                  inversion_test!(d_key, $chord, D_KEY, $($type)*);
                  inversion_test!(dsharp_key, $chord, DSHARP_KEY, $($type)*);
                  inversion_test!(e_key, $chord, E_KEY, $($type)*);
                  inversion_test!(f_key, $chord, F_KEY, $($type)*);
                  inversion_test!(fsharp_key, $chord, FSHARP_KEY, $($type)*);
                  inversion_test!(g_key, $chord, G_KEY, $($type)*);
                  inversion_test!(gsharp_key, $chord, GSHARP_KEY, $($type)*);
                  inversion_test!(a_key, $chord, A_KEY, $($type)*);
                  inversion_test!(asharp_key, $chord, ASHARP_KEY, $($type)*);
                  inversion_test!(b_key, $chord, B_KEY, $($type)*);
              }
          )*
      };
  }
    detect! {
        detect_major: Major all,
        detect_minor: Minor all,
        detect_aug: Aug [ i1 => Aug/E_KEY, i2 => Aug/GSHARP_KEY ],
        detect_dim: Dim all,
        detect_dim7: Dim7 [ i1 => Dim7/DSHARP_KEY, i2 => Dim7/FSHARP_KEY, i3 => Dim7/A_KEY ],
        detect_sus2: Sus2 [ i1 => Sus4/G_KEY, i2 => Sus4/G_KEY ], /* i1: 1 => sus4|sus2, i2: 2 => sus4 */
        detect_sus4: Sus4 [ i1 => Sus2/F_KEY, i2 => Sus4/C_KEY ], /* i1: 1 => sus2, i2: 2 => sus4|sus4 */
        detect_five: Five all,
        detect_sevensus4: SevenSus4 all,
        detect_major6: Major6 [ i1 => Minor7/A_KEY, i2 => Minor7/A_KEY, i3 => Minor7/A_KEY ], /* i1: 1 => m7|6, i2: 2 => m7|6, i3: 3 => m7 */
        detect_minor6: Minor6 [ i1 => Minor7b5/A_KEY, i2 => Minor7b5/A_KEY, i3 => Minor7b5/A_KEY ], /* i1: 1 => m7b5|m6, i2: 2 => m7b5|m6, i3: 3 => m7b5 */
        detect_major6_9: Major6_9 [ i1 => Minor7_11/A_KEY, i2 => Minor7_11/A_KEY, i3 => Minor7_11/A_KEY, i4 => Minor7_11/A_KEY ], /* i1: 1 => m7_11|6, i2: 2 => m7_11|6, i3: 3 => m7_11|6, i4: 4 => m7_11 */
        detect_minor6_9: Minor6_9 all,
        detect_major7: Major7 all,
        detect_major7_9: Major7_9 all,
        detect_major7b9: Major7b9 all,
        detect_major7plus9: Major7Plus9 all,
        detect_major7plus11: Major7Plus11 all,
        detect_major7b13: Major7b13 all,
        detect_major7_13: Major7_13 all,
        detect_major7aug: Major7Aug all,
        detect_minor7: Minor7 [ i1 => Major6/DSHARP_KEY, i2 => Minor7/C_KEY, i3 => Minor7/C_KEY ], /* i1: 1 => 6, i2: 2 => m7|6, i3: 3 => m7|6 */
        detect_minor7_9: Minor7_9 all,
        detect_minor7_11: Minor7_11 [ i1 => Major6_9/DSHARP_KEY, i2 => Minor7_11/C_KEY, i3 => Minor7_11/C_KEY, i4 => Minor7_11/C_KEY ], /* i1: 1 => 6_9, i2: 2 => m7_11|6_9, i3: 3 => m7_11|6_9, i4: 4 => m7_11|6_9 */
        detect_major7b5: Major7b5 [ i1 => Major7b5/C_KEY, i2 => Major7b5/FSHARP_KEY, i3 => Major7b5/FSHARP_KEY ],
        detect_minor7b5: Minor7b5 [ i1 => Minor6/DSHARP_KEY, i2 => Minor7b5/C_KEY, i3 => Minor7b5/C_KEY ], /* i1: 1 => m6, i2: 2 => m6|m7b5, i3: 3 => m6|m7b5 */
        detect_majormaj7: MajorMaj7 all,
        detect_majormaj7_9: MajorMaj7_9 all,
        detect_majormaj7plus11: MajorMaj7Plus11 all,
        detect_majormaj7aug: MajorMaj7Aug all,
        detect_minormaj7: MinorMaj7 all,
        detect_minormaj7_9: MinorMaj7_9 all,
        detect_majoradd9: MajorAdd9 all,
        detect_minoradd9: MinorAdd9 all
    }

    #[test]
    fn detect_multi_octave_chord() {
        let chord1 = vec![NOTE_C3, NOTE_E3, NOTE_G3, NOTE_ASHARP3, NOTE_D4];
        assert_eq!(detect_chord(&chord1), vec![Chord::Major7_9(C_KEY),]);
    }

    #[test]
    fn detect_chord_with_duplicated_notes() {
        // the root note C is duplicated one octave higher
        let chord1 = vec![NOTE_C3, NOTE_E3, NOTE_G3, NOTE_ASHARP3, NOTE_C4];
        assert_eq!(detect_chord(&chord1), vec![Chord::Major7(C_KEY),]);
    }

    #[test]
    fn detect_single_key() {
        let chord1 = vec![NOTE_C4];
        assert_eq!(detect_chord(&chord1), vec![Chord::None(C_KEY),]);
    }

    #[test]
    fn chord_templates() {
        let f_major = vec![NOTE_F3, NOTE_A3, NOTE_C4];
        assert_eq!(
            generate_templates(&f_major, true),
            vec![
                (F_KEY, vec![0, 4, 7]),
                (A_KEY, vec![0, 3, 8]),
                (C_KEY, vec![0, 5, 9]),
            ]
        );
    }

    #[test]
    fn chord_normalization() {
        let chord = vec![114, 118, 121, 124]; // 114 = FSHARP
        assert_eq!(chord_template(chord), (FSHARP_KEY, vec![0, 4, 7, 10]));
    }

    #[test]
    fn test_extract_chord_notes() {
        assert_eq!(
            Chord::Major7(F_KEY).notes(3, 0),
            vec![
                5 + 0 + 4 * OCTAVE_STEPS,
                5 + 4 + 4 * OCTAVE_STEPS,
                5 + 7 + 4 * OCTAVE_STEPS,
                5 + 10 + 4 * OCTAVE_STEPS,
            ]
        );
    }

    #[test]
    fn test_extract_chord_notes_first_inversion() {
        assert_eq!(
            Chord::Major7(F_KEY).notes(3, 1),
            vec![
                5 + 4 + 4 * OCTAVE_STEPS,
                5 + 7 + 4 * OCTAVE_STEPS,
                5 + 10 + 4 * OCTAVE_STEPS,
                5 + 0 + 5 * OCTAVE_STEPS,
            ]
        );
    }

    #[test]
    fn test_extract_chord_notes_second_inversion() {
        assert_eq!(
            Chord::Major7(F_KEY).notes(3, 2),
            vec![
                5 + 7 + 4 * OCTAVE_STEPS,
                5 + 10 + 4 * OCTAVE_STEPS,
                5 + 0 + 5 * OCTAVE_STEPS,
                5 + 4 + 5 * OCTAVE_STEPS,
            ]
        );
    }

    #[test]
    fn test_extract_chord_notes_third_inversion() {
        assert_eq!(
            Chord::Major7(F_KEY).notes(3, 3),
            vec![
                5 + 10 + 4 * OCTAVE_STEPS,
                5 + 0 + 5 * OCTAVE_STEPS,
                5 + 4 + 5 * OCTAVE_STEPS,
                5 + 7 + 5 * OCTAVE_STEPS,
            ]
        );
    }
}
