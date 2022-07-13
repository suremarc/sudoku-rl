use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
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
        Digit::from_u8((self.0[idx / 2] >> ((idx % 2) * 4)) & 0b1111) // if the index is even, we don't need to shift
    }

    pub fn get_xy(&self, i: usize, j: usize) -> Option<Digit> {
        self.get(i + 9 * j)
    }

    pub fn set(&mut self, idx: usize, val: Option<Digit>) {
        let mask: u8 = 0b1111 << ((idx % 2) * 4);
        let val_u8 = val.map_or(0, |d| d as u8) << ((idx % 2) * 4);

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

    pub fn full(&self) -> bool {
        self.first_empty_cell().is_none()
    }

    // FIXME: figure out how to refactor this to make it more compact
    pub fn solved(&self) -> bool {
        let mut found: u32 = 0;
        // check each row
        for i in 0..9 {
            for j in 0..9 {
                found |= 1_u32 << 9 >> self.get_xy(i, j).map_or(0, |d| d as usize);
            }

            if found != 0 {
                return false;
            }
            found = 0;
        }

        // check each column
        for j in 0..9 {
            for i in 0..9 {
                found |= 1_u32 << 9 << self.get_xy(i, j).map_or(0, |d| d as usize);
            }

            if found != 0 {
                return false;
            }
            found = 0;
        }

        // check each box
        for i in 0..9 {
            for j in 0..9 {
                found |= 1_u32 << 9
                    >> self
                        .get_xy(i % 3 + j / 3, i / 3 + j % 3)
                        .map_or(0, |d| d as usize);
            }

            if found != 0 {
                return false;
            }
            found = 0;
        }

        true
    }

    pub fn brute_force(&mut self) -> bool {
        let idx = self.first_empty_cell();
        if idx == None {
            return self.solved();
        }

        let idx = idx.unwrap();
        let (i, j) = (idx % 9, idx / 9);

        for candidate in (1..=9).map(|d| Digit::from_u8(d).unwrap()) {
            assert!(self.get_xy(i, j) == None);
            if self.safe(i, j, candidate) {
                self.set_xy(i, j, Some(candidate));
                if self.brute_force() {
                    return true;
                }

                self.set_xy(i, j, None);
            }
        }

        false
    }

    fn first_empty_cell(&self) -> Option<usize> {
        for (idx, &val) in self.0.iter().enumerate() {
            let a = Digit::from_u8(val & 0b1111);
            if a == None {
                return Some(2 * idx);
            }

            if idx == 40 {
                break;
            }
            let b = Digit::from_u8(val >> 4);
            if b == None {
                return Some(2 * idx + 1);
            }
        }

        None
    }

    fn safe(&self, i: usize, j: usize, candidate: Digit) -> bool {
        let (bi, bj) = (i - i % 3, j - j % 3);
        for x in 0..9 {
            if self.get_xy(i, x) == Some(candidate)
                || self.get_xy(x, j) == Some(candidate)
                || self.get_xy(bi + x / 3, bj + x % 3) == Some(candidate)
            {
                return false;
            }
        }

        true
    }
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
