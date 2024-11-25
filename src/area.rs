use num::Num;
use crate::pos::Pos;

/// An error returned when `Area<T>`'s dimension is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AreaBoundaryError;

/// A 2D area.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Area<T> {
    pub max_x: T,
    pub max_y: T,
    pub min_x: T,
    pub min_y: T,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AreaIterator<T> {
    pub area: Area<T>,
    pub current_x: T,
    pub current_y: T,
}

impl<T: Copy + Num + PartialOrd> Iterator for AreaIterator<T> {
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y > self.area.max_y {
            return None;
        }

        let result = Pos { x: self.current_x, y: self.current_y };
        if self.current_x >= self.area.max_x {
            self.current_x = self.area.min_x;
            self.current_y = self.current_y + T::one();
        } else {
            self.current_x = self.current_x + T::one();
        }
        Some(result)
    }
}

impl<T: Copy + Num + PartialOrd> IntoIterator for Area<T> {
    type Item = Pos<T>;
    type IntoIter = AreaIterator<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { area: self, current_x: self.min_x, current_y: self.min_y }
    }
}

impl<T: PartialOrd> Area<T> {
    /// Returns a new `Area<T>`.
    pub fn new(max_x: T, max_y: T, min_x: T, min_y: T) -> Result<Self, AreaBoundaryError> {
        if max_x < min_x || max_y < min_y {
            return Err(AreaBoundaryError);
        }
        Ok(Self { max_x, max_y, min_x, min_y })
    }

    /// Checks whether a `Pos<T>` is in this `Area<T>`.
    #[inline]
    pub fn has(&self, p: &Pos<T>) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }

    /// Filters a list of `Pos<T>` to the ones inside this `Area<T>`.
    #[inline]
    pub fn filter_pos<'a>(&'a self, pos: impl IntoIterator<Item = Pos<T>> + 'a) -> impl Iterator<Item = Pos<T>> + 'a {
        pos.into_iter().filter(move |p| self.has(p))
    }
}

impl<T: Num + PartialOrd> Area<T> {
    /// Returns a new `Area<T>` with custom max bounds and 0 as min bounds.
    #[inline]
    pub fn with_max(max_x: T, max_y: T) -> Result<Self, AreaBoundaryError> {
        Self::new(max_x, max_y, T::zero(), T::zero())
    }
}

impl<T: Copy + Num> Area<T> {
    /// Returns the row count.
    #[inline]
    pub fn rows(&self) -> T {
        self.max_y - self.min_y + T::one()
    }

    /// Returns the column count.
    #[inline]
    pub fn cols(&self) -> T {
        self.max_x - self.min_x + T::one()
    }

    /// Returns the area size.
    #[inline]
    pub fn size(&self) -> T {
        self.rows() * self.cols()
    }
}

impl<T: Copy> Area<T> {
    /// Returns the top left `Pos<T>`.
    #[inline]
    pub fn top_left(&self) -> Pos<T> {
        Pos { x: self.min_x, y: self.max_y }
    }

    /// Returns the top right `Pos<T>`.
    #[inline]
    pub fn top_right(&self) -> Pos<T> {
        Pos { x: self.max_x, y: self.max_y }
    }

    /// Returns the bottom left `Pos<T>`.
    #[inline]
    pub fn bottom_left(&self) -> Pos<T> {
        Pos { x: self.min_x, y: self.min_y }
    }

    /// Returns the bottom right `Pos<T>`.
    #[inline]
    pub fn bottom_right(&self) -> Pos<T> {
        Pos { x: self.max_x, y: self.min_y }
    }
}

#[cfg(test)]
mod test {
    use std::vec::Vec;
    use super::*;

    #[test]
    fn test_iter() {
        let area = Area { max_x: 2, max_y: 3, min_x: 0, min_y: -1 };
        let sut: Vec<Pos<_>> = area.into_iter().collect();
        assert_eq!(sut.len(), 15);
        assert_eq!(sut[0], Pos { x: 0, y: -1 });
        assert_eq!(sut[1], Pos { x: 1, y: -1 });
        assert_eq!(sut[2], Pos { x: 2, y: -1 });
        assert_eq!(sut[3], Pos { x: 0, y: 0 });
        assert_eq!(sut[4], Pos { x: 1, y: 0 });
        assert_eq!(sut[5], Pos { x: 2, y: 0 });
        assert_eq!(sut[6], Pos { x: 0, y: 1 });
        assert_eq!(sut[7], Pos { x: 1, y: 1 });
        assert_eq!(sut[8], Pos { x: 2, y: 1 });
        assert_eq!(sut[9], Pos { x: 0, y: 2 });
        assert_eq!(sut[10], Pos { x: 1, y: 2 });
        assert_eq!(sut[11], Pos { x: 2, y: 2 });
        assert_eq!(sut[12], Pos { x: 0, y: 3 });
        assert_eq!(sut[13], Pos { x: 1, y: 3 });
        assert_eq!(sut[14], Pos { x: 2, y: 3 });

        let area = Area { max_x: 0, max_y: 0, min_x: 0, min_y: 0 };
        let sut: Vec<Pos<_>> = area.into_iter().collect();
        assert_eq!(sut.len(), 1);
        assert_eq!(sut[0], Pos { x: 0, y: 0 });
    }

    #[test]
    fn test_into_iter() {
        let area = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        let sut = area.into_iter();
        assert_eq!(sut, AreaIterator { area, current_x: 0, current_y: 0, });

        let area = Area { max_x: 5, max_y: 10, min_x: -5, min_y: -10 };
        let sut = area.into_iter();
        assert_eq!(sut, AreaIterator { area, current_x: -5, current_y: -10, });
    }

    #[test]
    fn test_new() {
        let sut = Area::new(10, 10, 0, 0);
        assert_eq!(sut.unwrap(), Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 });

        let sut = Area::new(0, 0, 0, 0);
        assert_eq!(sut.unwrap(), Area { max_x: 0, max_y: 0, min_x: 0, min_y: 0 });

        let sut = Area::new(-1, -1, 0, 0);
        assert!(sut.is_err());
    }

    #[test]
    fn test_has() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert!(sut.has(&Pos { x: 10, y: 10 }));
        assert!(sut.has(&Pos { x: 0, y: 0 }));
        assert!(sut.has(&Pos { x: 10, y: 0 }));
        assert!(sut.has(&Pos { x: 0, y: 10 }));
        assert!(!sut.has(&Pos { x: -1, y: 10 }));
    }

    #[test]
    fn test_filter_pos() {
        let area = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        let p = Pos { x: 0, y: 0 };
        let sut: Vec<_> = area.filter_pos(p.neighbours(1)).collect();
        assert_eq!(sut.len(), 2);
        assert!(sut.contains(&Pos { x: 0, y: 1 }));
        assert!(sut.contains(&Pos { x: 1, y: 0 }));
        assert!(!sut.contains(&Pos { x: 0, y: -1 }));
        assert!(!sut.contains(&Pos { x: -1, y: 0 }));

        let p = Pos { x: 5, y: 5 };
        let sut: Vec<_> = area.filter_pos(p.neighbours(3)).collect();
        assert_eq!(sut.len(), 4);
        assert!(sut.contains(&Pos { x: 8, y: 5 }));
        assert!(sut.contains(&Pos { x: 2, y: 5 }));
        assert!(sut.contains(&Pos { x: 5, y: 8 }));
        assert!(sut.contains(&Pos { x: 5, y: 2 }));
    }

    #[test]
    fn test_rows() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert_eq!(sut.rows(), 11);

        let sut = Area { max_x: 5, max_y: 10, min_x: -5, min_y: -10 };
        assert_eq!(sut.rows(), 21);
    }

    #[test]
    fn test_cols() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert_eq!(sut.cols(), 11);

        let sut = Area { max_x: 10, max_y: 5, min_x: -10, min_y: -5 };
        assert_eq!(sut.cols(), 21);
    }

    #[test]
    fn test_size() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert_eq!(sut.size(), 121);

        let sut = Area { max_x: 10, max_y: 10, min_x: -10, min_y: -10 };
        assert_eq!(sut.size(), 441);
    }

    #[test]
    fn test_top_left() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert_eq!(sut.top_left(), Pos { x: 0, y: 10 });
    }

    #[test]
    fn test_top_right() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert_eq!(sut.top_right(), Pos { x: 10, y: 10 });
    }

    #[test]
    fn test_bottom_left() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert_eq!(sut.bottom_left(), Pos { x: 0, y: 0 });
    }

    #[test]
    fn test_bottom_right() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert_eq!(sut.bottom_right(), Pos { x: 10, y: 0 });
    }

    #[test]
    fn test_with_max() {
        let sut = Area::with_max(10, 20);
        assert_eq!(sut.unwrap(), Area { max_x: 10, max_y: 20, min_x: 0, min_y: 0 });

        let sut = Area::with_max(0, 0);
        assert_eq!(sut.unwrap(), Area { max_x: 0, max_y: 0, min_x: 0, min_y: 0 });

        let sut = Area::with_max(-1, -1);
        assert!(sut.is_err());
    }
}
