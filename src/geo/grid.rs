#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec;
#[cfg(feature = "std")]
use std::vec::Vec;
use core::fmt::Display;
use core::num::TryFromIntError;
use core::ops::{Index, IndexMut};
use core::str::FromStr;
use crate::geo::area::Area;
use crate::geo::pos::{Pos, PosIdx};

/// An error returned when a `Grid<T>` has invalid data dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridDimensionError;

/// An error returned when parsing a `Grid<T>` fails.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridParseError;

/// Represents a 2D rectangular grid.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

pub type CharGrid = Grid<u8>;

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, chunk) in self.data.chunks(self.width).enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            for (j, item) in chunk.iter().enumerate() {
                if j > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", item)?;
            }
        }
        Ok(())
    }
}

impl FromStr for CharGrid {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(GridParseError);
        }

        let lines: Vec<_> = s.lines().map(str::as_bytes).collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut data = Vec::with_capacity(width * height);
        data.extend(lines.iter().flat_map(|&line| line.iter()));

        if width * height != data.len() {
            return Err(GridParseError);
        }
        Ok(Self { width, height, data })
    }
}

impl<T> Index<PosIdx> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: PosIdx) -> &Self::Output {
        &self.data[self.width * index.y + index.x]
    }
}

impl<T> IndexMut<PosIdx> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: PosIdx) -> &mut Self::Output {
        &mut self.data[self.width * index.y + index.x]
    }
}

impl<T: Clone> Grid<T> {
    /// Returns a new `Grid<T>` with a value.
    pub fn new(width: usize, height: usize, value: T) -> Result<Self, GridDimensionError> {
        if width == 0 || height == 0 {
            return Err(GridDimensionError);
        }
        Ok(Self { width, height, data: vec![value; width * height] })
    }

    /// Returns a new `Grid<T>` with initial data.
    pub fn with_data(width: usize, data: &Vec<T>) -> Result<Self, GridDimensionError> {
        if width == 0 || data.is_empty() || data.len() % width != 0 {
            return Err(GridDimensionError);
        }
        Ok(Self { width, height: data.len() / width, data: (*data).clone() })
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    pub fn find(&self, value: T) -> Option<PosIdx> {
        self.data.iter().position(|&v| v == value).map(move |idx| Pos {
            x: idx % self.width,
            y: idx / self.width,
        })
    }
}

impl<T> Grid<T> {
    /// Returns an `Area<usize>` which bounds this `Grid<T>`.
    #[inline]
    pub fn area(&self) -> Area<usize> {
        Area { max_x: self.width - 1, max_y: self.height - 1, min_x: 0, min_y: 0 }
    }

    /// Returns an `Area<isize>` which bounds this `Grid<T>`.
    #[inline]
    pub fn signed_area(&self) -> Result<Area<isize>, TryFromIntError> {
        Ok(Area { max_x: isize::try_from(self.width - 1)?, max_y: isize::try_from(self.height - 1)?, min_x: 0, min_y: 0 })
    }

    /// Checks whether this `Grid<T>` has a `PosIdx`.
    #[inline]
    pub fn has(&self, pos: PosIdx) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}

#[cfg(test)]
mod test {
    use std::{format, vec};
    use super::*;

    #[test]
    fn test_display() {
        let sut = Grid { height: 3, width: 3, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9] };
        assert_eq!(format!("{}", sut), "1 2 3\n4 5 6\n7 8 9");
    }

    #[test]
    fn test_from_str() {
        let sut: CharGrid = Grid::from_str("abc\ndef").unwrap();
        assert_eq!(sut, Grid { height: 2, width: 3, data: vec![b'a', b'b', b'c', b'd', b'e', b'f'] });

        let sut: CharGrid = Grid::from_str("123").unwrap();
        assert_eq!(sut, Grid { height: 1, width: 3, data: vec![b'1', b'2', b'3'] });

        let sut: Result<CharGrid, GridParseError> = Grid::from_str("");
        assert!(sut.is_err());

        let sut: Result<CharGrid, GridParseError> = Grid::from_str("123\n45");
        assert!(sut.is_err());
    }

    #[test]
    fn test_index() {
        let sut = Grid { height: 3, width: 3, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9] };
        assert_eq!(sut[Pos { x: 0, y: 0 }], 1);
        assert_eq!(sut[Pos { x: 1, y: 1 }], 5);
        assert_eq!(sut[Pos { x: 1, y: 2 }], 8);
        assert_eq!(sut[Pos { x: 2, y: 2 }], 9);
    }

    #[test]
    fn test_index_mut() {
        let mut sut = Grid { height: 3, width: 3, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9] };
        sut[Pos { x: 0, y: 0 }] = 255;
        assert_eq!(sut[Pos { x: 0, y: 0 }], 255);

        sut[Pos { x: 2, y: 2 }] = 100;
        assert_eq!(sut[Pos { x: 2, y: 2 }], 100);
    }

    #[test]
    fn test_new() {
        let sut = Grid::new(3, 3, 100).unwrap();
        assert_eq!(sut, Grid { height: 3, width: 3, data: vec![100, 100, 100, 100, 100, 100, 100, 100, 100] });

        let sut = Grid::new(0, 0, 100);
        assert!(sut.is_err());
    }

    #[test]
    fn test_with_data() {
        let sut = Grid::with_data(3, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(sut, Grid { height: 3, width: 3, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9] });

        let sut = Grid::with_data(0, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert!(sut.is_err());

        let sut = Grid::with_data(3, &vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(sut.is_err());

        let sut: Result<Grid<i32>, GridDimensionError> = Grid::with_data(3, &vec![]);
        assert!(sut.is_err());
    }

    #[test]
    fn test_find() {
        let sut = Grid { height: 3, width: 3, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9] };
        assert_eq!(sut.find(1), Some(Pos { x: 0, y: 0 }));
        assert_eq!(sut.find(3), Some(Pos { x: 2, y: 0 }));
        assert_eq!(sut.find(9), Some(Pos { x: 2, y: 2 }));
        assert_eq!(sut.find(10), None);
    }

    #[test]
    fn test_area() {
        let sut = Grid { height: 10, width: 10, data: vec![0; 100] };
        assert_eq!(sut.area(), Area { max_x: 9, max_y: 9, min_x: 0, min_y: 0 });

        let sut = Grid { height: 1, width: 1, data: vec![0] };
        assert_eq!(sut.area(), Area { max_x: 0, max_y: 0, min_x: 0, min_y: 0 });
    }

    #[test]
    fn test_signed_area() {
        let sut = Grid { height: 10, width: 10, data: vec![0; 100] };
        assert_eq!(sut.signed_area().unwrap(), Area { max_x: 9, max_y: 9, min_x: 0, min_y: 0 });

        let sut = Grid { height: usize::MAX, width: usize::MAX, data: vec![0] };
        assert!(sut.signed_area().is_err());
    }

    #[test]
    fn test_has() {
        let sut = Grid { height: 10, width: 10, data: vec![0; 100] };
        assert!(sut.has(Pos { x: 0, y: 0 }));
        assert!(sut.has(Pos { x: 5, y: 5 }));
        assert!(sut.has(Pos { x: 9, y: 9 }));
        assert!(!sut.has(Pos { x: 10, y: 9 }));
        assert!(!sut.has(Pos { x: 9, y: 10 }));
    }
}
