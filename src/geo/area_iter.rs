use num::Num;
use crate::geo::area::Area;
use crate::geo::pos::Pos;

/// Represents an iterator that iterates through all the [`Pos<T>`]s inside a [`Pos<T>`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AreaIterator<T> {
    pub(crate) area: Area<T>,
    pub(crate) current_x: T,
    pub(crate) current_y: T,
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

#[cfg(test)]
mod test {
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
}
