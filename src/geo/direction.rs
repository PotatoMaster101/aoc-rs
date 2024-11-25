use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use num::{CheckedAdd, CheckedSub, Num};
use crate::geo::pos::Pos;

/// Represents the directions in a 2D grid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Display for Direction {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Direction::Up => write!(f, "up (north)"),
            Direction::Down => write!(f, "down (south)"),
            Direction::Left => write!(f, "left (west)"),
            Direction::Right => write!(f, "right (east)"),
            Direction::TopLeft => write!(f, "top left (north west)"),
            Direction::TopRight => write!(f, "top right (north east)"),
            Direction::BottomLeft => write!(f, "bottom left (south west)"),
            Direction::BottomRight => write!(f, "bottom right (south east)"),
        }
    }
}

impl From<u8> for Direction {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            b'^' => Direction::Up,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => Direction::Down,
        }
    }
}

impl Direction {
    /// Returns all the directions.
    #[inline]
    pub fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::TopLeft,
            Direction::TopRight,
            Direction::BottomLeft,
            Direction::BottomRight
        ]
    }

    /// Returns all the cross directions.
    #[inline]
    pub fn cross() -> [Direction; 4] {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }

    /// Returns all the diagonal directions.
    #[inline]
    pub fn diagonal() -> [Direction; 4] {
        [Direction::TopLeft, Direction::TopRight, Direction::BottomLeft, Direction::BottomRight]
    }

    /// Returns the back [`Direction`] relative to the current [`Direction`].
    #[inline]
    pub fn back(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::TopLeft => Direction::BottomRight,
            Direction::TopRight => Direction::BottomLeft,
            Direction::BottomLeft => Direction::TopRight,
            Direction::BottomRight => Direction::TopLeft,
        }
    }

    /// Returns the left [`Direction`] relative to the current [`Direction`].
    #[inline]
    pub fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::TopLeft => Direction::BottomLeft,
            Direction::TopRight => Direction::TopLeft,
            Direction::BottomLeft => Direction::BottomRight,
            Direction::BottomRight => Direction::TopRight,
        }
    }

    /// Returns the right [`Direction`] relative to the current [`Direction`].
    #[inline]
    pub fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::TopLeft => Direction::TopRight,
            Direction::TopRight => Direction::BottomRight,
            Direction::BottomLeft => Direction::TopLeft,
            Direction::BottomRight => Direction::BottomLeft,
        }
    }
}

/// Represents a [`Pos<T>`] with a direction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DirectionalPos<T> {
    pub pos: Pos<T>,
    pub direction: Direction,
}

impl<T: PartialOrd> PartialOrd for DirectionalPos<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl<T: Ord> Ord for DirectionalPos<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl<T: Display> Display for DirectionalPos<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.pos, self.direction)
    }
}

impl<T: Copy + Num> DirectionalPos<T> {
    /// Returns the [`DirectionalPos<T>`] next to this [`DirectionalPos<T>`].
    #[inline]
    pub fn next(&self, distance: T) -> Self {
        Self { pos: self.pos.dest(distance, self.direction), direction: self.direction }
    }

    /// Returns the `DirectionPos<T>` with a new direction.
    #[inline]
    pub fn update_direction(&self, direction: Direction) -> Self {
        Self { pos: self.pos, direction }
    }
}

impl<T: Copy + Num + CheckedAdd + CheckedSub> DirectionalPos<T> {
    /// Returns the [`DirectionalPos<T>`] next to this [`DirectionalPos<T>`]. [`None`] when arithmetic overflow.
    #[inline]
    pub fn checked_next(&self, distance: T) -> Option<Self> {
        Some(Self { pos: self.pos.checked_dest(distance, self.direction)?, direction: self.direction })
    }
}

impl<T> DirectionalPos<T> {
    /// Returns a new [`DirectionalPos<T>`].
    #[inline]
    pub fn new(pos: Pos<T>, direction: Direction) -> Self {
        Self { pos, direction }
    }
}

#[cfg(test)]
mod test {
    use crate::geo::pos::PosIdx;
    use super::*;

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::Up), "up (north)");
        assert_eq!(format!("{}", Direction::Down), "down (south)");
        assert_eq!(format!("{}", Direction::Left), "left (west)");
        assert_eq!(format!("{}", Direction::Right), "right (east)");
        assert_eq!(format!("{}", Direction::TopLeft), "top left (north west)");
        assert_eq!(format!("{}", Direction::TopRight), "top right (north east)");
        assert_eq!(format!("{}", Direction::BottomLeft), "bottom left (south west)");
        assert_eq!(format!("{}", Direction::BottomRight), "bottom right (south east)");
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Direction::Up, Direction::from(b'^'));
        assert_eq!(Direction::Left, Direction::from(b'<'));
        assert_eq!(Direction::Right, Direction::from(b'>'));
        assert_eq!(Direction::Down, Direction::from(b'v'));
    }

    #[test]
    fn test_all() {
        let sut = Direction::all();
        assert!(sut.contains(&Direction::Up));
        assert!(sut.contains(&Direction::Down));
        assert!(sut.contains(&Direction::Left));
        assert!(sut.contains(&Direction::Right));
        assert!(sut.contains(&Direction::TopLeft));
        assert!(sut.contains(&Direction::TopRight));
        assert!(sut.contains(&Direction::BottomLeft));
        assert!(sut.contains(&Direction::BottomRight));
    }

    #[test]
    fn test_cross() {
        let sut = Direction::cross();
        assert!(sut.contains(&Direction::Up));
        assert!(sut.contains(&Direction::Down));
        assert!(sut.contains(&Direction::Left));
        assert!(sut.contains(&Direction::Right));
    }

    #[test]
    fn test_diagonal() {
        let sut = Direction::diagonal();
        assert!(sut.contains(&Direction::TopLeft));
        assert!(sut.contains(&Direction::TopRight));
        assert!(sut.contains(&Direction::BottomLeft));
        assert!(sut.contains(&Direction::BottomRight));
    }

    #[test]
    fn test_back() {
        assert_eq!(Direction::Up.back(), Direction::Down);
        assert_eq!(Direction::Down.back(), Direction::Up);
        assert_eq!(Direction::Left.back(), Direction::Right);
        assert_eq!(Direction::Right.back(), Direction::Left);
        assert_eq!(Direction::TopLeft.back(), Direction::BottomRight);
        assert_eq!(Direction::TopRight.back(), Direction::BottomLeft);
        assert_eq!(Direction::BottomLeft.back(), Direction::TopRight);
        assert_eq!(Direction::BottomRight.back(), Direction::TopLeft);
    }

    #[test]
    fn test_left() {
        assert_eq!(Direction::Up.left(), Direction::Left);
        assert_eq!(Direction::Down.left(), Direction::Right);
        assert_eq!(Direction::Left.left(), Direction::Down);
        assert_eq!(Direction::Right.left(), Direction::Up);
        assert_eq!(Direction::TopLeft.left(), Direction::BottomLeft);
        assert_eq!(Direction::TopRight.left(), Direction::TopLeft);
        assert_eq!(Direction::BottomLeft.left(), Direction::BottomRight);
        assert_eq!(Direction::BottomRight.left(), Direction::TopRight);
    }

    #[test]
    fn test_right() {
        assert_eq!(Direction::Up.right(), Direction::Right);
        assert_eq!(Direction::Down.right(), Direction::Left);
        assert_eq!(Direction::Left.right(), Direction::Up);
        assert_eq!(Direction::Right.right(), Direction::Down);
        assert_eq!(Direction::TopLeft.right(), Direction::TopRight);
        assert_eq!(Direction::TopRight.right(), Direction::BottomRight);
        assert_eq!(Direction::BottomLeft.right(), Direction::TopLeft);
        assert_eq!(Direction::BottomRight.right(), Direction::BottomLeft);
    }

    #[test]
    fn test_pos_display() {
        let sut = DirectionalPos { pos: Pos { x: 10, y: 30 }, direction: Direction::Up };
        assert_eq!(format!("{}", sut), "(10, 30): up (north)");
    }

    #[test]
    fn test_pos_ord() {
        let sut = DirectionalPos { pos: Pos { x: 1, y: 2 }, direction: Direction::Up };
        assert!(sut > DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::Up });
        assert!(sut > DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::Down });
        assert!(sut > DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::Left });
        assert!(sut > DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::Right });
        assert!(sut < DirectionalPos { pos: Pos { x: 10, y: 0 }, direction: Direction::Up });
        assert!(sut < DirectionalPos { pos: Pos { x: 10, y: 0 }, direction: Direction::Down });
        assert!(sut < DirectionalPos { pos: Pos { x: 10, y: 0 }, direction: Direction::Left });
        assert!(sut < DirectionalPos { pos: Pos { x: 10, y: 0 }, direction: Direction::Right });
    }

    #[test]
    fn test_next() {
        let p = DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::Down };
        let sut = p.next(3);
        assert_eq!(sut.pos, Pos { x: 0, y: -3 });
        assert_eq!(sut.direction, Direction::Down);
    }

    #[test]
    fn test_checked_next() {
        let p = DirectionalPos { pos: PosIdx { x: 0, y: 0 }, direction: Direction::Up };
        let sut = p.checked_next(3).unwrap();
        assert_eq!(sut.pos, Pos { x: 0, y: 3 });

        let p = DirectionalPos { pos: PosIdx { x: 0, y: 0 }, direction: Direction::Down };
        let sut = p.checked_next(3);
        assert!(sut.is_none());
    }

    #[test]
    fn new_pos_direction() {
        let p = DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::TopLeft };
        let sut = p.update_direction(Direction::Up);
        assert_eq!(sut.pos, Pos { x: 0, y: 0 });
        assert_eq!(sut.direction, Direction::Up);
    }

    #[test]
    fn test_pos_new() {
        let sut = DirectionalPos { pos: Pos { x: 10, y: 30 }, direction: Direction::TopLeft };
        assert_eq!(sut.pos, Pos { x: 10, y: 30 });
        assert_eq!(sut.direction, Direction::TopLeft);
    }
}
