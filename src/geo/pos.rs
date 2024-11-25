use core::fmt::{Display, Formatter};
use core::num::TryFromIntError;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::str::FromStr;
use num::{Num, Signed};
use crate::geo::direction::Direction;
use crate::geo::line_iter::LineIterator;

/// An error returned when parsing a `Pos<T>` fails.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosParseError;

/// A position in a 2D space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

pub type PosIdx = Pos<usize>;
pub type SignedPosIdx = Pos<isize>;

impl<T: Display> Display for Pos<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: FromStr> FromStr for Pos<T> {
    type Err = PosParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(PosParseError)?;

        let x = x.trim().parse::<T>().map_err(|_| PosParseError)?;
        let y = y.trim().parse::<T>().map_err(|_| PosParseError)?;
        Ok(Pos { x, y })
    }
}

impl<T: Add<Output = T>> Add for Pos<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: AddAssign> AddAssign for Pos<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Pos<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: SubAssign> SubAssign for Pos<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Clone + Copy + Mul<Output = T>> Mul<T> for Pos<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Clone + Copy + MulAssign> MulAssign<T> for Pos<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Clone + Copy + Div<Output = T>> Div<T> for Pos<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T: Clone + Copy + DivAssign> DivAssign<T> for Pos<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Pos<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl<T> Pos<T> {
    /// Returns a new `Pos<T>`.
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy + Num + PartialOrd> Pos<T> {
    /// Returns a new `LineIterator<T>` from this position.
    #[inline]
    pub fn line_iter(&self, distance: usize, direction: Direction) -> LineIterator<T> {
        LineIterator { current: *self, distance, direction }
    }
}

impl<T: Clone + Copy + Num> Pos<T> {
    /// Returns a `Pos<T>` above the current `Pos<T>` (`y + distance`).
    #[inline]
    pub fn up(&self, distance: T) -> Self {
        Self { x: self.x, y: self.y + distance }
    }

    /// Returns a `Pos<T>` below the current `Pos<T>` (`y - distance`).
    #[inline]
    pub fn down(&self, distance: T) -> Self {
        Self { x: self.x, y: self.y - distance }
    }

    /// Returns a `Pos<T>` to the left of the current `Pos<T>` (`x - distance`).
    #[inline]
    pub fn left(&self, distance: T) -> Self {
        Self { x: self.x - distance, y: self.y }
    }

    /// Returns a `Pos<T>` to the right of the current `Pos<T>` (`x + distance`).
    #[inline]
    pub fn right(&self, distance: T) -> Self {
        Self { x: self.x + distance, y: self.y }
    }

    /// Returns a `Pos<T>` to the top left of the current `Pos<T>` (`x - distance` and `y + distance`).
    #[inline]
    pub fn top_left(&self, distance: T) -> Self {
        Self { x: self.x - distance, y: self.y + distance }
    }

    /// Returns a `Pos<T>` to the top right of the current `Pos<T>` (`x + distance` and `y + distance`).
    #[inline]
    pub fn top_right(&self, distance: T) -> Self {
        Self { x: self.x + distance, y: self.y + distance }
    }

    /// Returns a `Pos<T>` to the bottom left of the current `Pos<T>` (`x - distance` and `y - distance`).
    #[inline]
    pub fn bottom_left(&self, distance: T) -> Self {
        Self { x: self.x - distance, y: self.y - distance }
    }

    /// Returns a `Pos<T>` to the bottom right of the current `Pos<T>` (`x + distance` and `y - distance`).
    #[inline]
    pub fn bottom_right(&self, distance: T) -> Self {
        Self { x: self.x + distance, y: self.y - distance }
    }

    /// Returns a list of neighbouring `Pos<T>`s.
    #[inline]
    pub fn neighbours(&self, distance: T) -> [Self; 4] {
        [self.up(distance), self.down(distance), self.left(distance), self.right(distance)]
    }

    /// Returns a list of diagonal neighbouring `Pos<T>`s.
    #[inline]
    pub fn neighbours_diag(&self, distance: T) -> [Self; 4] {
        [self.top_left(distance), self.top_right(distance), self.bottom_left(distance), self.bottom_right(distance)]
    }

    /// Returns the destination `Pos<T>`.
    #[inline]
    pub fn destination(&self, distance: T, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.up(distance),
            Direction::Down => self.down(distance),
            Direction::Left => self.left(distance),
            Direction::Right => self.right(distance),
            Direction::TopLeft => self.top_left(distance),
            Direction::TopRight => self.top_right(distance),
            Direction::BottomLeft => self.bottom_left(distance),
            Direction::BottomRight => self.bottom_right(distance),
        }
    }

    /// Returns the `Pos<T>` at origin.
    #[inline]
    pub fn origin() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }

    /// Returns the `Pos<T>` at unit X .
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero() }
    }

    /// Returns the `Pos<T>` at unit Y.
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one() }
    }
}

impl<T: Copy + Signed> Pos<T> {
    /// Returns the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
    #[inline]
    pub fn manhattan(&self, other: Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T: Copy> Pos<T> {
    /// Swaps X and Y values.
    #[inline]
    pub fn swap(&self) -> Self {
        Self { x: self.y, y: self.x }
    }
}

impl TryFrom<SignedPosIdx> for PosIdx {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: SignedPosIdx) -> Result<Self, Self::Error> {
        Ok(Pos { x: usize::try_from(value.x)?, y: usize::try_from(value.y)? })
    }
}

impl TryFrom<PosIdx> for SignedPosIdx {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: PosIdx) -> Result<Self, Self::Error> {
        Ok(Pos { x: isize::try_from(value.x)?, y: isize::try_from(value.y)? })
    }
}

#[cfg(test)]
mod test {
    use std::format;
    use super::*;

    #[test]
    fn test_display() {
        let sut = Pos { x: 1, y: -2 };
        assert_eq!(format!("{}", sut), "(1, -2)");
    }

    #[test]
    fn test_from_str() {
        let sut: Result<Pos<i32>, PosParseError> = Pos::from_str("(1, -2)");
        assert_eq!(sut.unwrap(), Pos { x: 1, y: -2 });

        let sut: Result<Pos<i32>, PosParseError> = Pos::from_str("(1, x)");
        assert!(sut.is_err());
    }

    #[test]
    fn test_add() {
        let sut = Pos { x: 1, y: 2 } + Pos { x: 3, y: 4 };
        assert_eq!(sut, Pos { x: 4, y: 6 });

        let sut = Pos { x: -1, y: -2 } + Pos { x: -3, y: -4 };
        assert_eq!(sut, Pos { x: -4, y: -6 });
    }

    #[test]
    fn test_add_assign() {
        let mut sut = Pos { x: 1, y: 2 };
        sut += Pos { x: 3, y: 4 };
        assert_eq!(sut, Pos { x: 4, y: 6 });

        let mut sut = Pos { x: -1, y: -2 };
        sut += Pos { x: -3, y: -4 };
        assert_eq!(sut, Pos { x: -4, y: -6 });
    }

    #[test]
    fn test_sub() {
        let sut = Pos { x: 1, y: 2 } - Pos { x: 3, y: 4 };
        assert_eq!(sut, Pos { x: -2, y: -2 });

        let sut = Pos { x: -1, y: -2 } - Pos { x: -3, y: -4 };
        assert_eq!(sut, Pos { x: 2, y: 2 });
    }

    #[test]
    fn test_sub_assign() {
        let mut sut = Pos { x: 1, y: 2 };
        sut -= Pos { x: 3, y: 4 };
        assert_eq!(sut, Pos { x: -2, y: -2 });

        let mut sut = Pos { x: -1, y: -2 };
        sut -= Pos { x: -3, y: -4 };
        assert_eq!(sut, Pos { x: 2, y: 2 });
    }

    #[test]
    fn test_mul() {
        let sut = Pos { x: 1, y: 2 } * -3;
        assert_eq!(sut, Pos { x: -3, y: -6 });

        let sut = Pos { x: -1, y: -2 } * 4;
        assert_eq!(sut, Pos { x: -4, y: -8 });
    }

    #[test]
    fn test_mul_assign() {
        let mut sut = Pos { x: 1, y: 2 };
        sut *= -3;
        assert_eq!(sut, Pos { x: -3, y: -6 });

        let mut sut = Pos { x: -1, y: -2 };
        sut *= 4;
        assert_eq!(sut, Pos { x: -4, y: -8 });
    }

    #[test]
    fn test_div() {
        let sut = Pos { x: 3, y: 9 } / 3;
        assert_eq!(sut, Pos { x: 1, y: 3 });

        let sut = Pos { x: -4, y: 8 } / -4;
        assert_eq!(sut, Pos { x: 1, y: -2 });
    }

    #[test]
    fn test_div_assign() {
        let mut sut = Pos { x: 3, y: 9 };
        sut /= 3;
        assert_eq!(sut, Pos { x: 1, y: 3 });

        let mut sut = Pos { x: -4, y: 8 };
        sut /= -4;
        assert_eq!(sut, Pos { x: 1, y: -2 });
    }

    #[test]
    fn test_neg() {
        let sut = -Pos { x: 1, y: 2 };
        assert_eq!(sut, Pos { x: -1, y: -2 });
    }

    #[test]
    fn test_new() {
        let sut = Pos::new(1, 2);
        assert_eq!(sut, Pos { x: 1, y: 2 });
    }

    #[test]
    fn test_line_iter() {
        let p = Pos { x: 1, y: 2 };
        let sut = p.line_iter(4, Direction::Right);
        assert_eq!(sut.current, Pos { x: 1, y: 2 });
        assert_eq!(sut.direction, Direction::Right);
        assert_eq!(sut.distance, 4);
    }

    #[test]
    fn test_up() {
        let p = Pos { x: 1, y: 2 };
        let sut = p.up(3);
        assert_eq!(sut, Pos { x: 1, y: 5 });
    }

    #[test]
    fn test_down() {
        let p = Pos { x: 1, y: 2 };
        let sut = p.down(3);
        assert_eq!(sut, Pos { x: 1, y: -1 });
    }

    #[test]
    fn test_left() {
        let p = Pos { x: 1, y: 2 };
        let sut = p.left(3);
        assert_eq!(sut, Pos { x: -2, y: 2 });
    }

    #[test]
    fn test_right() {
        let p = Pos { x: 1, y: 2 };
        let sut = p.right(3);
        assert_eq!(sut, Pos { x: 4, y: 2 });
    }

    #[test]
    fn test_top_left() {
        let p = Pos { x: 0, y: 0 };
        let sut = p.top_left(3);
        assert_eq!(sut, Pos { x: -3, y: 3 });
    }

    #[test]
    fn test_top_right() {
        let p = Pos { x: 0, y: 0 };
        let sut = p.top_right(3);
        assert_eq!(sut, Pos { x: 3, y: 3 });
    }

    #[test]
    fn test_bottom_left() {
        let p = Pos { x: 0, y: 0 };
        let sut = p.bottom_left(3);
        assert_eq!(sut, Pos { x: -3, y: -3 });
    }

    #[test]
    fn test_bottom_right() {
        let p = Pos { x: 0, y: 0 };
        let sut = p.bottom_right(3);
        assert_eq!(sut, Pos { x: 3, y: -3 });
    }

    #[test]
    fn test_neighbours() {
        let p = Pos { x: 1, y: 2 };
        let sut = p.neighbours(2);
        assert!(sut.contains(&Pos { x: 1, y: 4 }));
        assert!(sut.contains(&Pos { x: 1, y: 0 }));
        assert!(sut.contains(&Pos { x: 3, y: 2 }));
        assert!(sut.contains(&Pos { x: -1, y: 2 }));
    }

    #[test]
    fn test_neighbours_diag() {
        let p = Pos { x: 0, y: 0 };
        let sut = p.neighbours_diag(2);
        assert!(sut.contains(&Pos { x: -2, y: 2 }));
        assert!(sut.contains(&Pos { x: 2, y: 2 }));
        assert!(sut.contains(&Pos { x: -2, y: -2 }));
        assert!(sut.contains(&Pos { x: 2, y: -2 }));
    }

    #[test]
    fn test_destination() {
        let sut = Pos { x: 0, y: 0 };
        assert_eq!(sut.destination(5, Direction::Up), Pos { x: 0, y: 5 });
        assert_eq!(sut.destination(5, Direction::Down), Pos { x: 0, y: -5 });
        assert_eq!(sut.destination(5, Direction::Left), Pos { x: -5, y: 0 });
        assert_eq!(sut.destination(5, Direction::Right), Pos { x: 5, y: 0 });
        assert_eq!(sut.destination(5, Direction::TopLeft), Pos { x: -5, y: 5 });
        assert_eq!(sut.destination(5, Direction::TopRight), Pos { x: 5, y: 5 });
        assert_eq!(sut.destination(5, Direction::BottomLeft), Pos { x: -5, y: -5 });
        assert_eq!(sut.destination(5, Direction::BottomRight), Pos { x: 5, y: -5 });
    }

    #[test]
    fn test_origin() {
        let sut: Pos<i32> = Pos::origin();
        assert_eq!(sut, Pos { x: 0, y: 0 });
    }

    #[test]
    fn test_unit_x() {
        let sut: Pos<i32> = Pos::unit_x();
        assert_eq!(sut, Pos { x: 1, y: 0 });
    }

    #[test]
    fn test_unit_y() {
        let sut: Pos<i32> = Pos::unit_y();
        assert_eq!(sut, Pos { x: 0, y: 1 });
    }

    #[test]
    fn test_manhattan() {
        let p = Pos { x: 1, y: 2 };
        assert_eq!(p.manhattan(Pos { x: 3, y: 4 }), 4);
        assert_eq!(p.manhattan(Pos { x: 45, y: -9 }), 55);

        let p = Pos { x: -1, y: -2 };
        assert_eq!(p.manhattan(Pos { x: -45, y: 9 }), 55);
    }

    #[test]
    fn test_swap() {
        let sut = Pos { x: 1, y: 2 };
        assert_eq!(sut.swap(), Pos { x: 2, y: 1 });

        let sut = Pos { x: -2, y: 1 };
        assert_eq!(sut.swap(), Pos { x: 1, y: -2 });
    }

    #[test]
    fn test_try_from_signed_idx() {
        let p = SignedPosIdx { x: 1, y: 2 };
        let sut = PosIdx::try_from(p).unwrap();
        assert_eq!(sut, PosIdx { x: 1, y: 2 });

        let p = SignedPosIdx { x: -1, y: -1 };
        let sut = PosIdx::try_from(p);
        assert!(sut.is_err());
    }

    #[test]
    fn test_try_from_unsigned_idx() {
        let p = PosIdx { x: 1, y: 2 };
        let sut = SignedPosIdx::try_from(p).unwrap();
        assert_eq!(sut, SignedPosIdx { x: 1, y: 2 });

        let p = PosIdx { x: usize::MAX, y: usize::MAX };
        let sut = SignedPosIdx::try_from(p);
        assert!(sut.is_err());
    }
}
