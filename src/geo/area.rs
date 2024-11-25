use num::Num;
use crate::geo::area_iter::AreaIterator;
use crate::geo::pos::Pos;

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

impl<T: Copy + Num + PartialOrd> IntoIterator for Area<T> {
    type Item = Pos<T>;
    type IntoIter = AreaIterator<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { area: self, current_x: self.min_x, current_y: self.min_y }
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

    /// Returns whether a `Pos<T>` is on the boundary of this `Area<T>`.
    #[inline]
    pub fn on_boundary(&self, p: &Pos<T>) -> bool {
        p.x == self.max_x || p.x == self.min_x || p.y == self.max_y || p.y == self.min_y
    }
}

impl<T: Copy + PartialOrd> Area<T> {
    /// Returns an `Area<T>` constructed from top left and bottom right `Pos<T>`s.
    pub fn from_pos(top_left: &Pos<T>, bottom_right: &Pos<T>) -> Result<Self, AreaBoundaryError> {
        if bottom_right.y > top_left.y || bottom_right.x < top_left.x {
            return Err(AreaBoundaryError);
        }
        Ok(Self { max_x: bottom_right.x, min_x: top_left.x, max_y: top_left.y, min_y: bottom_right.y })
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
    use super::*;

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
        let sut: Vec<_> = area.filter_pos([
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: -1 },
            Pos { x: -1, y: 0 },
        ]).collect();
        assert_eq!(sut.len(), 2);
        assert!(sut.contains(&Pos { x: 0, y: 1 }));
        assert!(sut.contains(&Pos { x: 1, y: 0 }));

        let sut: Vec<_> = area.filter_pos([
            Pos { x: 8, y: 5 },
            Pos { x: 2, y: 5 },
            Pos { x: 5, y: 8 },
            Pos { x: 5, y: 2 },
        ]).collect();
        assert_eq!(sut.len(), 4);
        assert!(sut.contains(&Pos { x: 8, y: 5 }));
        assert!(sut.contains(&Pos { x: 2, y: 5 }));
        assert!(sut.contains(&Pos { x: 5, y: 8 }));
        assert!(sut.contains(&Pos { x: 5, y: 2 }));
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
    fn test_on_boundary() {
        let sut = Area { max_x: 10, max_y: 10, min_x: 0, min_y: 0 };
        assert!(sut.on_boundary(&Pos { x: 10, y: 10 }));
        assert!(sut.on_boundary(&Pos { x: 3, y: 0 }));
        assert!(sut.on_boundary(&Pos { x: 0, y: 4 }));
        assert!(sut.on_boundary(&Pos { x: 0, y: 0 }));
        assert!(!sut.on_boundary(&Pos { x: 1, y: 1 }));
        assert!(!sut.on_boundary(&Pos { x: 9, y: 9 }));
    }

    #[test]
    fn test_from_pos() {
        let sut = Area::from_pos(&Pos { x: 0, y: 10 }, &Pos { x: 10, y: 0 }).unwrap();
        assert_eq!(sut.min_x, 0);
        assert_eq!(sut.max_x, 10);
        assert_eq!(sut.min_y, 0);
        assert_eq!(sut.max_y, 10);

        let sut = Area::from_pos(&Pos { x: -1, y: 1 }, &Pos { x: 1, y: -1 }).unwrap();
        assert_eq!(sut.min_x, -1);
        assert_eq!(sut.max_x, 1);
        assert_eq!(sut.min_y, -1);
        assert_eq!(sut.max_y, 1);

        let sut = Area::from_pos(&Pos { x: 0, y: 0 }, &Pos { x: 0, y: 0 }).unwrap();
        assert_eq!(sut.min_x, 0);
        assert_eq!(sut.max_x, 0);
        assert_eq!(sut.min_y, 0);
        assert_eq!(sut.max_y, 0);

        let sut = Area::from_pos(&Pos { x: 0, y: 0 }, &Pos { x: 1, y: 1 });
        assert!(sut.is_err());
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
}
