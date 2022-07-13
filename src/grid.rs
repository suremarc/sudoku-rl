use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone)]
// Grid is a representation of a Sudoku grid.
// Two digits are packed in each byte, so that the whole grid fits in a single cache line.
pub struct Grid([u8; 41]);

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
}
