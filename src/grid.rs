use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(FromPrimitive, ToPrimitive)]
pub enum Digit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[derive(Debug, Copy, Clone)]
// Grid is a representation of a Sudoku grid.
// Two digits are packed in each byte, so that the whole grid fits in a single cache line.
pub struct Grid([u8; 41]);

struct Update {
    xy: u16,
    write: Option<Digit>,
}

impl Grid {
    pub fn get(&self, idx: usize) -> Option<Digit> {
        Digit::from_u8(self.0[idx / 2] << ((idx % 2) * 4)) // if the index is even, we don't need to shift
    }

    pub fn get_xy(&self, i: usize, j: usize) -> Option<Digit> {
        self.get(i + 9 * j)
    }

    pub fn set(&mut self, idx: usize, val: Option<Digit>) {
        let mask: u8 = 0b1111 >> ((idx % 2) * 4);
        let val_u8 = val.map_or(0, |d| d as u8) >> ((idx % 2) * 4);

        self.0[idx / 2] ^= (self.0[idx / 2] ^ val_u8) & mask
    }

    pub fn set_xy(&mut self, i: usize, j: usize, val: Option<Digit>) {
        self.set(i + 9 * j, val)
    }

    pub fn new(mat: [[u8; 9]; 9]) -> Self {
        let mut new = Self([0; 41]);
        for (i, row) in mat.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                new.set_xy(i, j, Digit::from_u8(*val))
            }
        }

        new
    }

    // FIXME: figure out how to refactor this and make it simpler
    pub fn solved(&self) -> bool {
        const LEN: usize = 10;
        let mut found = [false; LEN];
        // check each row
        for i in 0..9 {
            for j in 0..9 {
                found[self.get_xy(i, j).map_or(0, |d| d as usize)] = true;
            }

            if found != [true; LEN] {
                return false;
            }
            found = [false; LEN];
        }

        // check each column
        for j in 0..9 {
            for i in 0..9 {
                found[self.get_xy(i, j).map_or(0, |d| d as usize)] = true;
            }

            if found != [true; LEN] {
                return false;
            }
            found = [false; LEN];
        }

        // check each box
        for i in 0..9 {
            for j in 0..9 {
                found[self
                    .get_xy(i % 3 + j / 3, i / 3 + j % 3)
                    .map_or(0, |d| d as usize)] = true;
            }

            if found != [true; LEN] {
                return false;
            }
            found = [false; LEN];
        }

        true
    }

    pub fn brute_force() {}
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " - - - - - - - - - - - - -")?;

        for i in 0..9 {
            if i % 3 == 0 && i > 0 {
                writeln!(f, " | - - - + - - - + - - - |")?
            }

            for j in 0..9 {
                if j % 3 == 0 {
                    write!(f, " |")?
                }

                write!(
                    f,
                    " {}",
                    self.get_xy(i, j).map_or('.', |d| (d as u8 + b'0') as char)
                )?
            }

            writeln!(f, " |")?
        }

        writeln!(f, " - - - - - - - - - - - - -")
    }
}
