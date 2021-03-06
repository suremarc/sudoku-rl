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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid([Option<Digit>; 81]);

impl Grid {
    pub fn new_from_rows(mat: [[u8; 9]; 9]) -> Self {
        let mut new: Self = Default::default();
        for (row, data) in mat.iter().enumerate() {
            for (col, &val) in data.iter().enumerate() {
                new[(row, col)] = Digit::from_u8(val);
            }
        }

        new
    }

    pub fn new_from_array(arr: [u8; 81]) -> Self {
        let mut new: Self = Default::default();
        for (i, &v) in arr.iter().enumerate() {
            new[i] = Digit::from_u8(v);
        }

        new
    }

    pub fn full(&self) -> bool {
        !self.0.iter().any(Option::is_none)
    }

    pub fn brute_force(&mut self) -> bool {
        let idx = self.0.iter().position(Option::is_none);
        if idx == None {
            return self.solved();
        }

        let idx = idx.unwrap();
        let (i, j) = (idx / 9, idx % 9);

        for candidate in (1..=9).map(|d| Digit::from_u8(d).unwrap()) {
            if self.safe(i, j, candidate) {
                self[(i, j)] = Some(candidate);
                if self.brute_force() {
                    return true;
                }

                self[(i, j)] = None;
            }
        }

        false
    }

    // FIXME: figure out how to refactor this to make it more compact
    pub fn solved(&self) -> bool {
        let rows = (0..9_usize).map(|x| {
            [
                (x, 0),
                (x, 1),
                (x, 2),
                (x, 3),
                (x, 4),
                (x, 5),
                (x, 6),
                (x, 7),
                (x, 8),
            ]
        });
        let columns = (0..9_usize).map(|x| {
            [
                (0, x),
                (1, x),
                (2, x),
                (3, x),
                (4, x),
                (5, x),
                (6, x),
                (7, x),
                (8, x),
            ]
        });
        let boxes = (0..9_usize).map(|x| {
            let (i, j) = (3 * (x / 3), 3 * (x % 3));
            [
                (i, j),
                (i + 1, j),
                (i + 2, j),
                (i, j + 1),
                (i + 1, j + 1),
                (i + 2, j + 1),
                (i, j + 2),
                (i + 1, j + 2),
                (i + 2, j + 2),
            ]
        });

        for set in rows.chain(columns).chain(boxes) {
            let mut found = [false; 10];
            const ALL_FOUND: [bool; 10] =
                [false, true, true, true, true, true, true, true, true, true];
            for v in set {
                found[self[v].map_or(0, |d| d as usize)] = true;
            }

            if found != ALL_FOUND {
                return false;
            }
        }

        true
    }

    fn safe(&self, i: usize, j: usize, candidate: Digit) -> bool {
        let (bi, bj) = (i - i % 3, j - j % 3);
        for x in 0..9 {
            if self[(i, x)] == Some(candidate)
                || self[(x, j)] == Some(candidate)
                || self[(bi + x / 3, bj + x % 3)] == Some(candidate)
            {
                return false;
            }
        }

        true
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self([None; 81])
    }
}

impl std::ops::Index<usize> for Grid {
    type Output = Option<Digit>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = Option<Digit>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[9 * index.0 + index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[9 * index.0 + index.1]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " - - - - - - - - - - - - -")?;

        for r in 0..9 {
            if r % 3 == 0 && r > 0 {
                writeln!(f, " | - - - + - - - + - - - |")?
            }

            for c in 0..9 {
                if c % 3 == 0 {
                    write!(f, " |")?
                }

                write!(
                    f,
                    " {}",
                    self[(r, c)].map_or('.', |d| (d as u8 + b'0') as char)
                )?
            }

            writeln!(f, " |")?
        }

        writeln!(f, " - - - - - - - - - - - - -")
    }
}
