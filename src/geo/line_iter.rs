use num::Num;
use crate::geo::direction::Direction;
use crate::geo::pos::Pos;

/// Represents an iterator that iterates through all the `Pos<T>`s on a line.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LineIterator<T> {
    pub(crate) current: Pos<T>,
    pub(crate) direction: Direction,
    pub(crate) distance: usize,
}

impl<T: Copy + Num + PartialOrd> Iterator for LineIterator<T> {
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.distance == 0 {
            return None;
        }

        let result = self.current;
        self.current = self.current.destination(T::one(), self.direction);
        self.distance -= 1;
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use std::vec::Vec;
    use super::*;

    #[test]
    fn test_iter() {
        let iter = LineIterator { current: Pos::origin(), direction: Direction::Right, distance: 5 };
        let sut: Vec<_> = iter.collect();
        assert_eq!(sut.len(), 5);
        assert_eq!(sut[0], Pos { x: 0, y: 0 });
        assert_eq!(sut[1], Pos { x: 1, y: 0 });
        assert_eq!(sut[2], Pos { x: 2, y: 0 });
        assert_eq!(sut[3], Pos { x: 3, y: 0 });
        assert_eq!(sut[4], Pos { x: 4, y: 0 });
    }
}
