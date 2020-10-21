#[derive(PartialEq, Clone, Copy)]
pub struct Key(pub u8);

pub const OCTAVE_STEPS: u8 = 12;

pub const C_KEY: Key = Key(0);
pub const CSHARP_KEY: Key = Key(1);
pub const D_KEY: Key = Key(2);
pub const DSHARP_KEY: Key = Key(3);
pub const E_KEY: Key = Key(4);
pub const F_KEY: Key = Key(5);
pub const FSHARP_KEY: Key = Key(6);
pub const G_KEY: Key = Key(7);
pub const GSHARP_KEY: Key = Key(8);
pub const A_KEY: Key = Key(9);
pub const ASHARP_KEY: Key = Key(10);
pub const B_KEY: Key = Key(11);

use std::ops::Add;
impl Add for Key {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % OCTAVE_STEPS)
    }
}

use std::fmt;

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        f.write_str(name[self.0 as usize % 12])
    }
}

// This is mostly for manually constructing chords, so it
// does not need to cover the complete range at this point
// Note that in the literature, C4 is sometimes #60 and
// sometimes #72. We use C4 = #60.
pub const NOTE_C7: u8 = 96;
pub const NOTE_B6: u8 = 95;
pub const NOTE_BFLAT6: u8 = 94;
pub const NOTE_ASHARP6: u8 = 94;
pub const NOTE_A6: u8 = 93;
pub const NOTE_AFLAT6: u8 = 92;
pub const NOTE_GSHARP6: u8 = 92;
pub const NOTE_G6: u8 = 91;
pub const NOTE_GFLAT6: u8 = 90;
pub const NOTE_FSHARP6: u8 = 90;
pub const NOTE_F6: u8 = 89;
pub const NOTE_E6: u8 = 88;
pub const NOTE_EFLAT6: u8 = 87;
pub const NOTE_DSHARP6: u8 = 87;
pub const NOTE_D6: u8 = 86;
pub const NOTE_DFLAT6: u8 = 85;
pub const NOTE_CSHARP6: u8 = 85;
pub const NOTE_C6: u8 = 84;
pub const NOTE_B5: u8 = 83;
pub const NOTE_BFLAT5: u8 = 82;
pub const NOTE_ASHARP5: u8 = 82;
pub const NOTE_A5: u8 = 81;
pub const NOTE_AFLAT5: u8 = 80;
pub const NOTE_GSHARP5: u8 = 80;
pub const NOTE_G5: u8 = 79;
pub const NOTE_GFLAT5: u8 = 78;
pub const NOTE_FSHARP5: u8 = 78;
pub const NOTE_F5: u8 = 77;
pub const NOTE_E5: u8 = 76;
pub const NOTE_EFLAT5: u8 = 75;
pub const NOTE_DSHARP5: u8 = 75;
pub const NOTE_D5: u8 = 74;
pub const NOTE_DFLAT5: u8 = 73;
pub const NOTE_CSHARP5: u8 = 73;
pub const NOTE_C5: u8 = 72;
pub const NOTE_B4: u8 = 71;
pub const NOTE_BFLAT4: u8 = 70;
pub const NOTE_ASHARP4: u8 = 70;
pub const NOTE_A4: u8 = 69;
pub const NOTE_AFLAT4: u8 = 68;
pub const NOTE_GSHARP4: u8 = 68;
pub const NOTE_G4: u8 = 67;
pub const NOTE_GFLAT4: u8 = 66;
pub const NOTE_FSHARP4: u8 = 66;
pub const NOTE_F4: u8 = 65;
pub const NOTE_E4: u8 = 64;
pub const NOTE_EFLAT4: u8 = 63;
pub const NOTE_DSHARP4: u8 = 63;
pub const NOTE_D4: u8 = 62;
pub const NOTE_DFLAT4: u8 = 61;
pub const NOTE_CSHARP4: u8 = 61;
pub const NOTE_C4: u8 = 60;
pub const NOTE_B3: u8 = 59;
pub const NOTE_BFLAT3: u8 = 58;
pub const NOTE_ASHARP3: u8 = 58;
pub const NOTE_A3: u8 = 57;
pub const NOTE_AFLAT3: u8 = 56;
pub const NOTE_GSHARP3: u8 = 56;
pub const NOTE_G3: u8 = 55;
pub const NOTE_GFLAT3: u8 = 54;
pub const NOTE_FSHARP3: u8 = 54;
pub const NOTE_F3: u8 = 53;
pub const NOTE_E3: u8 = 52;
pub const NOTE_EFLAT3: u8 = 51;
pub const NOTE_DSHARP3: u8 = 51;
pub const NOTE_D3: u8 = 50;
pub const NOTE_DFLAT3: u8 = 49;
pub const NOTE_CSHARP3: u8 = 49;
pub const NOTE_C3: u8 = 48;
pub const NOTE_B2: u8 = 47;
pub const NOTE_BFLAT2: u8 = 46;
pub const NOTE_ASHARP2: u8 = 46;
pub const NOTE_A2: u8 = 45;
pub const NOTE_AFLAT2: u8 = 44;
pub const NOTE_GSHARP2: u8 = 44;
pub const NOTE_G2: u8 = 43;
pub const NOTE_GFLAT2: u8 = 42;
pub const NOTE_FSHARP2: u8 = 42;
pub const NOTE_F2: u8 = 41;
pub const NOTE_E2: u8 = 40;
pub const NOTE_EFLAT2: u8 = 39;
pub const NOTE_DSHARP2: u8 = 39;
pub const NOTE_D2: u8 = 38;
pub const NOTE_DFLAT2: u8 = 37;
pub const NOTE_CSHARP2: u8 = 37;
pub const NOTE_C2: u8 = 36;
pub const NOTE_B1: u8 = 35;
pub const NOTE_BFLAT1: u8 = 34;
pub const NOTE_ASHARP1: u8 = 34;
pub const NOTE_A1: u8 = 33;
pub const NOTE_AFLAT1: u8 = 32;
pub const NOTE_GSHARP1: u8 = 32;
pub const NOTE_G1: u8 = 31;
pub const NOTE_GFLAT1: u8 = 30;
pub const NOTE_FSHARP1: u8 = 30;
pub const NOTE_F1: u8 = 29;
pub const NOTE_E1: u8 = 28;
pub const NOTE_EFLAT1: u8 = 27;
pub const NOTE_DSHARP1: u8 = 27;
pub const NOTE_D1: u8 = 26;
pub const NOTE_DFLAT1: u8 = 25;
pub const NOTE_CSHARP1: u8 = 25;
pub const NOTE_C1: u8 = 24;
pub const NOTE_B0: u8 = 23;
pub const NOTE_BFLAT0: u8 = 22;
pub const NOTE_ASHARP0: u8 = 22;
pub const NOTE_A0: u8 = 21;
pub const NOTE_AFLAT0: u8 = 20;
pub const NOTE_GSHARP0: u8 = 20;
pub const NOTE_G0: u8 = 19;
pub const NOTE_GFLAT0: u8 = 18;
pub const NOTE_FSHARP0: u8 = 18;
pub const NOTE_F0: u8 = 17;
pub const NOTE_E0: u8 = 16;
pub const NOTE_EFLAT0: u8 = 15;
pub const NOTE_DSHARP0: u8 = 15;
pub const NOTE_D0: u8 = 14;
pub const NOTE_DFLAT0: u8 = 13;
pub const NOTE_CSHARP0: u8 = 13;
pub const NOTE_C0: u8 = 12;
pub const NOTE_B: u8 = 11;
pub const NOTE_BFLAT: u8 = 10;
pub const NOTE_ASHARP: u8 = 10;
pub const NOTE_A: u8 = 9;
pub const NOTE_AFLAT: u8 = 8;
pub const NOTE_GSHARP: u8 = 8;
pub const NOTE_G: u8 = 7;
pub const NOTE_GFLAT: u8 = 6;
pub const NOTE_FSHARP: u8 = 6;
pub const NOTE_F: u8 = 5;
pub const NOTE_E: u8 = 4;
pub const NOTE_EFLAT: u8 = 3;
pub const NOTE_DSHARP: u8 = 3;
pub const NOTE_D: u8 = 2;
pub const NOTE_DFLAT: u8 = 1;
pub const NOTE_CSHARP: u8 = 1;
pub const NOTE_C: u8 = 0;
