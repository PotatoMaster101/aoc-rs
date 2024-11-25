use core::fmt::{Display, Formatter};
use num::Num;
use crate::geo::direction::Direction;
use crate::geo::pos::Pos;

/// Represents a `Pos<T>` with a direction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DirectionalPos<T> {
    pub pos: Pos<T>,
    pub direction: Direction,
}

impl<T: Display> Display for DirectionalPos<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.pos, self.direction)
    }
}

impl<T: Clone + Copy + Num> DirectionalPos<T> {
    /// Returns the `DirectionalPos<T>` next to this `DirectionalPos<T>`.
    #[inline]
    pub fn next(&self, distance: T) -> Self {
        Self { pos: self.next_pos(distance), direction: self.direction }
    }

    /// Returns the `Pos<T>` next to this `DirectionalPos<T>`.
    #[inline]
    pub fn next_pos(&self, distance: T) -> Pos<T> {
        self.pos.destination(distance, self.direction)
    }

    /// Returns the `DirectionPos<T>` with a new direction.
    #[inline]
    pub fn update_direction(&self, direction: Direction) -> Self {
        Self { pos: self.pos, direction }
    }
}

impl<T> DirectionalPos<T> {
    /// Returns a new `DirectionalPos<T>`.
    #[inline]
    pub fn new(pos: Pos<T>, direction: Direction) -> Self {
        Self { pos, direction }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let sut = DirectionalPos { pos: Pos { x: 10, y: 30 }, direction: Direction::Up };
        assert_eq!(format!("{}", sut), "(10, 30): up (north)");
    }

    #[test]
    fn test_next() {
        let p = DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::Down };
        let sut = p.next(3);
        assert_eq!(sut.pos, Pos { x: 0, y: -3 });
        assert_eq!(sut.direction, Direction::Down);
    }

    #[test]
    fn test_next_pos() {
        let p = DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::TopLeft };
        let sut = p.next_pos(9);
        assert_eq!(sut, Pos { x: -9, y: 9 });
    }

    #[test]
    fn new_direction() {
        let p = DirectionalPos { pos: Pos { x: 0, y: 0 }, direction: Direction::TopLeft };
        let sut = p.update_direction(Direction::Up);
        assert_eq!(sut.pos, Pos { x: 0, y: 0 });
        assert_eq!(sut.direction, Direction::Up);
    }

    #[test]
    fn test_new() {
        let sut = DirectionalPos { pos: Pos { x: 10, y: 30 }, direction: Direction::TopLeft };
        assert_eq!(sut.pos, Pos { x: 10, y: 30 });
        assert_eq!(sut.direction, Direction::TopLeft);
    }
}
