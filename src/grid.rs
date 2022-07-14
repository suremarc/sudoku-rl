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

pub trait Grid
where
    Self: Sized,
{
    fn new_zeroed() -> Self;

    fn get(&self, idx: usize) -> Option<Digit>;
    fn set(&mut self, idx: usize, val: Option<Digit>);

    #[inline]
    fn get_xy(&self, i: usize, j: usize) -> Option<Digit> {
        self.get(i + 9 * j)
    }

    #[inline]
    fn set_xy(&mut self, i: usize, j: usize, val: Option<Digit>) {
        self.set(i + 9 * j, val)
    }

    fn new_from_rows(mat: [[u8; 9]; 9]) -> Self {
        let mut new = Self::new_zeroed();
        for (j, row) in mat.iter().enumerate() {
            for (i, val) in row.iter().enumerate() {
                new.set_xy(i, j, Digit::from_u8(*val))
            }
        }

        new
    }

    fn first_empty_cell(&self) -> Option<usize> {
        for idx in 0..81 {
            if self.get(idx).is_none() {
                return Some(idx);
            };
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

    fn full(&self) -> bool {
        self.first_empty_cell().is_none()
    }

    // FIXME: figure out how to refactor this to make it more compact
    fn solved(&self) -> bool {
        let mut found: [bool; 10] = [false; 10];
        let found_all = [false, true, true, true, true, true, true, true, true, true];
        // check each row
        for j in 0..9 {
            for i in 0..9 {
                found[self.get_xy(i, j).map_or(0, |d| d as usize)] = true;
            }

            if found != found_all {
                return false;
            }
            found = [false; 10];
        }

        // check each column
        for i in 0..9 {
            for j in 0..9 {
                found[self.get_xy(i, j).map_or(0, |d| d as usize)] = true;
            }

            if found != found_all {
                return false;
            }
            found = [false; 10];
        }

        // check each box
        for i in 0..9 {
            for j in 0..9 {
                found[self
                    .get_xy(3 * (i % 3) + j / 3, 3 * (i / 3) + j % 3)
                    .map_or(0, |d| d as usize)] = true;
            }

            if found != found_all {
                return false;
            }
            found = [false; 10];
        }

        true
    }

    fn brute_force_with_count(&mut self, num: &mut u64) -> bool {
        let idx = self.first_empty_cell();
        if idx == None {
            return self.solved();
        }

        let idx = idx.unwrap();
        let (i, j) = (idx % 9, idx / 9);

        for candidate in (1..=9).map(|d| Digit::from_u8(d).unwrap()) {
            if self.safe(i, j, candidate) {
                self.set_xy(i, j, Some(candidate));
                *num += 1;
                if self.brute_force_with_count(num) {
                    return true;
                }

                self.set_xy(i, j, None);
            }
        }

        false
    }

    fn brute_force(&mut self) -> (bool, u64) {
        let mut num: u64 = 0;
        let solved = self.brute_force_with_count(&mut num);

        (solved, num)
    }
}

macro_rules! impl_grid_display {
    ($name:ty) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, " - - - - - - - - - - - - -")?;

                for j in 0..9 {
                    if j % 3 == 0 && j > 0 {
                        writeln!(f, " | - - - + - - - + - - - |")?
                    }

                    for i in 0..9 {
                        if i % 3 == 0 {
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
    };
}

pub struct NaiveGrid([Option<Digit>; 81]);

impl Grid for NaiveGrid {
    fn new_zeroed() -> NaiveGrid {
        Self([None; 81])
    }

    #[inline]
    fn get(&self, idx: usize) -> Option<Digit> {
        self.0[idx]
    }

    #[inline]
    fn set(&mut self, idx: usize, val: Option<Digit>) {
        self.0[idx] = val
    }
}

impl_grid_display!(NaiveGrid);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// PackedGrid is a representation of a Sudoku grid.
// Two digits are packed in each byte, so that the whole grid fits in a single cache line.
pub struct PackedGrid([u8; 41]);

impl Grid for PackedGrid {
    fn new_zeroed() -> PackedGrid {
        Self([0; 41])
    }

    #[inline]
    fn get(&self, idx: usize) -> Option<Digit> {
        Digit::from_u8((self.0[idx / 2] >> ((idx % 2) * 4)) & 0b1111) // if the index is even, we don't need to shift
    }

    #[inline]
    fn set(&mut self, idx: usize, val: Option<Digit>) {
        let mask: u8 = 0b1111 << ((idx % 2) * 4);
        let val_u8 = val.map_or(0, |d| d as u8) << ((idx % 2) * 4);

        self.0[idx / 2] ^= (self.0[idx / 2] ^ val_u8) & mask
    }
}

impl_grid_display!(PackedGrid);
