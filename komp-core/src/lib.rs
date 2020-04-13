mod chord;
mod key;

pub use chord::*;
pub use key::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_chord() {
        let chord = vec![NOTE_G3, NOTE_ASHARP3, NOTE_C4, NOTE_DSHARP4];
        assert_eq!(detect_chord(&chord), vec![Chord::Minor7(C_KEY)]);
    }
    #[test]
    fn test_chord_key() {
        assert_eq!(Chord::Minor7(C_KEY).key(), &C_KEY);
    }
}
