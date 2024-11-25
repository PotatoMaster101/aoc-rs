use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use num::{CheckedAdd, CheckedSub, Num, Signed};
use num::traits::{CheckedNeg, CheckedRem};
use crate::geo::direction::Direction;

/// A position in a 2D space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Display for Pos<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl<T: Copy + CheckedAdd> CheckedAdd for Pos<T> {
    #[inline]
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(Self { x: self.x.checked_add(&v.x)?, y: self.y.checked_add(&v.y)? })
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

impl<T: Copy + CheckedSub> CheckedSub for Pos<T> {
    #[inline]
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(Self { x: self.x.checked_sub(&v.x)?, y: self.y.checked_sub(&v.y)? })
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Pos<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Pos<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Pos<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Pos<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Rem<Output = T>> Rem for Pos<T> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self { x: self.x % rhs.x, y: self.y % rhs.y }
    }
}

impl<T: RemAssign> RemAssign for Pos<T> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<T: CheckedRem> CheckedRem for Pos<T> {
    #[inline]
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        Some(Self { x: self.x.checked_rem(&v.x)?, y: self.y.checked_rem(&v.y)? })
    }
}

impl<T: Neg<Output = T>> Neg for Pos<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl<T: CheckedNeg> CheckedNeg for Pos<T> {
    #[inline]
    fn checked_neg(&self) -> Option<Self> {
        Some(Self { x: self.x.checked_neg()?, y: self.y.checked_neg()? })
    }
}

impl<T: Neg<Output = T> + Num> From<Direction> for Pos<T> {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self { x: T::zero(), y: T::one() },
            Direction::Down => Self { x: T::zero(), y: -T::one() },
            Direction::Left => Self { x: -T::one(), y: T::zero() },
            Direction::Right => Self { x: T::one(), y: T::zero() },
            Direction::TopLeft => Self { x: -T::one(), y: T::one() },
            Direction::TopRight => Self { x: T::one(), y: T::one() },
            Direction::BottomLeft => Self { x: -T::one(), y: -T::one() },
            Direction::BottomRight => Self { x: T::one(), y: -T::one() },
        }
    }
}

impl<T> Pos<T> {
    /// Returns a new [`Pos<T>`].
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy + Num> Pos<T> {
    /// Computes modulo between this [`Pos<T>`] and another [`Pos<T>`].
    #[inline]
    pub fn modulo(&self, rhs: &Self) -> Self {
        Self { x: (self.x % rhs.x + rhs.x) % rhs.x, y: (self.y % rhs.y + rhs.y) % rhs.y }
    }

    /// Returns the destination [`Pos<T>`].
    #[inline]
    pub fn dest(&self, distance: T, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self { x: self.x, y: self.y + distance },
            Direction::Down => Self { x: self.x, y: self.y - distance },
            Direction::Left => Self { x: self.x - distance, y: self.y },
            Direction::Right => Self { x: self.x + distance, y: self.y },
            Direction::TopLeft => Self { x: self.x - distance, y: self.y + distance },
            Direction::TopRight => Self { x: self.x + distance, y: self.y + distance },
            Direction::BottomLeft => Self { x: self.x - distance, y: self.y - distance },
            Direction::BottomRight => Self { x: self.x + distance, y: self.y - distance },
        }
    }

    /// Returns the [`Pos<T>`] at origin.
    #[inline]
    pub fn origin() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }

    /// Returns the [`Pos<T>`] at unit X .
    #[inline]
    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero() }
    }

    /// Returns the [`Pos<T>`] at unit Y.
    #[inline]
    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one() }
    }
}

impl<T: Copy + Num + CheckedAdd + CheckedSub> Pos<T> {
    /// Returns the destination [`Pos<T>`], or [`None`] if arithmetic error.
    #[inline]
    pub fn checked_dest(&self, distance: T, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => Some(Self { x: self.x, y: self.y.checked_add(&distance)? }),
            Direction::Down => Some(Self { x: self.x, y: self.y.checked_sub(&distance)? }),
            Direction::Left => Some(Self { x: self.x.checked_sub(&distance)?, y: self.y }),
            Direction::Right => Some(Self { x: self.x.checked_add(&distance)?, y: self.y }),
            Direction::TopLeft => Some(Self { x: self.x.checked_sub(&distance)?, y: self.y.checked_add(&distance)? }),
            Direction::TopRight => Some(Self { x: self.x.checked_add(&distance)?, y: self.y.checked_add(&distance)? }),
            Direction::BottomLeft => Some(Self { x: self.x.checked_sub(&distance)?, y: self.y.checked_sub(&distance)? }),
            Direction::BottomRight => Some(Self { x: self.x.checked_add(&distance)?, y: self.y.checked_sub(&distance)? }),
        }
    }
}

impl<T: Copy + Signed> Pos<T> {
    /// Returns the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
    #[inline]
    pub fn manhattan(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Returns a [`Pos<T>`] with X and Y's abs value.
    #[inline]
    pub fn abs(&self) -> Self {
        Pos { x: self.x.abs(), y: self.y.abs() }
    }
}

impl<T: Copy> Pos<T> {
    /// Swaps X and Y values.
    #[inline]
    pub fn swap(&self) -> Self {
        Self { x: self.y, y: self.x }
    }

    /// Returns a [`Pos<T>`] with X and Y set to the same value.
    #[inline]
    pub fn with_same(xy: T) -> Self {
        Self { x: xy, y: xy }
    }
}

pub type PosIdx = Pos<usize>;

impl PosIdx {
    /// Returns the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
    #[inline]
    pub fn manhattan_unsigned(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
    fn test_checked_add() {
        let sut = Pos { x: 1, y: 2 }.checked_add(&Pos { x: 3, y: 4 }).unwrap();
        assert_eq!(sut, Pos { x: 4, y: 6 });

        let sut = Pos { x: 1, y: 2 }.checked_add(&Pos { x: usize::MAX, y: usize::MAX });
        assert!(sut.is_none());
    }

    #[test]
    fn test_from_direction() {
        assert_eq!(Pos::from(Direction::Up), Pos { x: 0, y: 1 });
        assert_eq!(Pos::from(Direction::Down), Pos { x: 0, y: -1 });
        assert_eq!(Pos::from(Direction::Left), Pos { x: -1, y: 0 });
        assert_eq!(Pos::from(Direction::Right), Pos { x: 1, y: 0 });
        assert_eq!(Pos::from(Direction::TopLeft), Pos { x: -1, y: 1 });
        assert_eq!(Pos::from(Direction::TopRight), Pos { x: 1, y: 1 });
        assert_eq!(Pos::from(Direction::BottomLeft), Pos { x: -1, y: -1 });
        assert_eq!(Pos::from(Direction::BottomRight), Pos { x: 1, y: -1 });
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
    fn test_checked_sub() {
        let sut = Pos { x: 1, y: 2 }.checked_sub(&Pos { x: 3, y: 4 }).unwrap();
        assert_eq!(sut, Pos { x: -2, y: -2 });

        let sut = Pos { x: 1, y: 2 }.checked_sub(&Pos { x: usize::MAX, y: usize::MAX });
        assert!(sut.is_none());
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
    fn test_rem() {
        let sut = Pos { x: 9, y: 9 } % Pos { x: 10, y: 10 };
        assert_eq!(sut, Pos { x: 9, y: 9 });

        let sut = Pos { x: 11, y: 13 } % Pos { x: 10, y: 11 };
        assert_eq!(sut, Pos { x: 1, y: 2 });

        let sut = Pos { x: -1, y: -1 } % Pos { x: 10, y: 10 };
        assert_eq!(sut, Pos { x: -1, y: -1 });

        let sut = Pos { x: 10, y: 10 } % Pos { x: -1, y: -1 };
        assert_eq!(sut, Pos { x: 0, y: 0 });
    }

    #[test]
    fn test_rem_assign() {
        let mut sut = Pos { x: 9, y: 9 };
        sut %= Pos { x: 10, y: 10 };
        assert_eq!(sut, Pos { x: 9, y: 9 });

        let mut sut = Pos { x: 11, y: 13 };
        sut %= Pos { x: 10, y: 11 };
        assert_eq!(sut, Pos { x: 1, y: 2 });
    }

    #[test]
    fn test_checked_rem() {
        let sut = Pos { x: 11, y: 13 }.checked_rem(&Pos { x: 10, y: 11 }).unwrap();
        assert_eq!(sut, Pos { x: 1, y: 2 });

        let sut = Pos { x:10, y: 10 }.checked_rem(&Pos { x: 0, y: 0 });
        assert!(sut.is_none());
    }

    #[test]
    fn test_neg() {
        let sut = -Pos { x: 1, y: 2 };
        assert_eq!(sut, Pos { x: -1, y: -2 });
    }

    #[test]
    fn test_checked_neg() {
        let sut = Pos { x: 1, y: 2 }.checked_neg().unwrap();
        assert_eq!(sut, Pos { x: -1, y: -2 });

        let sut = Pos { x: usize::MAX, y: usize::MAX }.checked_neg();
        assert!(sut.is_none());
    }

    #[test]
    fn test_new() {
        let sut = Pos::new(1, 2);
        assert_eq!(sut, Pos { x: 1, y: 2 });
    }

    #[test]
    fn test_modulo() {
        let sut = Pos { x: 10, y: 10 }.modulo(&Pos { x: 10, y: 10 });
        assert_eq!(sut, Pos { x: 0, y: 0 });

        let sut = Pos { x: 9, y: 9 }.modulo(&Pos { x: 10, y: 10 });
        assert_eq!(sut, Pos { x: 9, y: 9 });

        let sut = Pos { x: -1, y: -1 }.modulo(&Pos { x: 10, y: 15 });
        assert_eq!(sut, Pos { x: 9, y: 14 });
    }

    #[test]
    fn test_dest() {
        let sut = Pos { x: 0, y: 0 };
        assert_eq!(sut.dest(5, Direction::Up), Pos { x: 0, y: 5 });
        assert_eq!(sut.dest(5, Direction::Down), Pos { x: 0, y: -5 });
        assert_eq!(sut.dest(5, Direction::Left), Pos { x: -5, y: 0 });
        assert_eq!(sut.dest(5, Direction::Right), Pos { x: 5, y: 0 });
        assert_eq!(sut.dest(5, Direction::TopLeft), Pos { x: -5, y: 5 });
        assert_eq!(sut.dest(5, Direction::TopRight), Pos { x: 5, y: 5 });
        assert_eq!(sut.dest(5, Direction::BottomLeft), Pos { x: -5, y: -5 });
        assert_eq!(sut.dest(5, Direction::BottomRight), Pos { x: 5, y: -5 });
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
        assert_eq!(p.manhattan(&Pos { x: 3, y: 4 }), 4);
        assert_eq!(p.manhattan(&Pos { x: 45, y: -9 }), 55);

        let p = Pos { x: -1, y: -2 };
        assert_eq!(p.manhattan(&Pos { x: -45, y: 9 }), 55);
    }

    #[test]
    fn test_abs() {
        let sut = Pos { x: 1, y: 2 };
        assert_eq!(sut.abs(), Pos { x: 1, y: 2 });

        let sut = Pos { x: -1, y: -2 };
        assert_eq!(sut.abs(), Pos { x: 1, y: 2 });
    }

    #[test]
    fn test_checked_dest() {
        let sut: Pos<usize> = Pos { x: 0, y: 0 };
        assert_eq!(sut.checked_dest(5, Direction::Up), Some(Pos { x: 0, y: 5 }));
        assert_eq!(sut.checked_dest(5, Direction::Down), None);
        assert_eq!(sut.checked_dest(5, Direction::Left), None);
        assert_eq!(sut.checked_dest(5, Direction::Right), Some(Pos { x: 5, y: 0 }));
        assert_eq!(sut.checked_dest(5, Direction::TopLeft), None);
        assert_eq!(sut.checked_dest(5, Direction::TopRight), Some(Pos { x: 5, y: 5 }));
        assert_eq!(sut.checked_dest(5, Direction::BottomLeft), None);
        assert_eq!(sut.checked_dest(5, Direction::BottomRight), None);
    }

    #[test]
    fn test_swap() {
        let sut = Pos { x: 1, y: 2 };
        assert_eq!(sut.swap(), Pos { x: 2, y: 1 });

        let sut = Pos { x: -2, y: 1 };
        assert_eq!(sut.swap(), Pos { x: 1, y: -2 });
    }

    #[test]
    fn test_with_same() {
        let sut: Pos<i32> = Pos::with_same(3);
        assert_eq!(sut, Pos { x: 3, y: 3 });

        let sut: Pos<i32> = Pos::with_same(-53);
        assert_eq!(sut, Pos { x: -53, y: -53 });
    }

    #[test]
    fn test_manhattan_unsigned() {
        let p = PosIdx { x: 1, y: 2 };
        assert_eq!(p.manhattan_unsigned(&Pos { x: 1, y: 2 }), 0);
        assert_eq!(p.manhattan_unsigned(&Pos { x: 3, y: 4 }), 4);
        assert_eq!(p.manhattan_unsigned(&Pos { x: 0, y: 0 }), 3);
    }
}
