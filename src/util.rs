use std::ops::{Add, Index, IndexMut};
use crate::util::Direction::{EAST, NORTH, SOUTH, WEST};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}
impl Direction {
    pub(crate) const ALL: [Direction; 4] = [EAST, SOUTH, WEST, NORTH];

    pub(crate) fn opposite(self) -> Self {
        match self {
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST,
            NORTH => SOUTH,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Index2D(pub i32, pub i32);

impl Add<Direction> for  Index2D {
    type Output = Index2D;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            EAST => Self(self.0 + 1, self.1),
            SOUTH => Self(self.0, self.1 + 1),
            WEST => Self(self.0 - 1, self.1),
            NORTH => Self(self.0, self.1 - 1)
        }
    }
}

#[derive(Debug)]
pub struct Flat2DArray<T> {
    contents: Vec<T>,
    columns: usize,
    out_of_bounds_element: T,
}

impl<T> Flat2DArray<T> {
    pub(crate) fn from_data(out_of_bounds_element: T, contents: Vec<T>, columns: usize) -> Self {
        assert_eq!(contents.len() % columns, 0);

        Self { contents, columns, out_of_bounds_element }
    }

    fn range_check(&self, x: i32, y: i32) -> bool {
        let cols = self.columns as i32;
        let len = self.contents.len() as i32;
        (0..cols).contains(&x) && (0..len / cols).contains(&y)
    }

    fn linearize_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.columns + x as usize
    }
}

impl<T> Index<Index2D> for Flat2DArray<T> {
    type Output = T;

    fn index(&self, index: Index2D) -> &Self::Output {
        let Index2D(x, y) = index;
        if self.range_check(x, y) {
            &self.contents[self.linearize_index(x, y)]
        } else {
            &self.out_of_bounds_element
        }
    }
}

impl<T> IndexMut<Index2D> for Flat2DArray<T> {
    fn index_mut(&mut self, index: Index2D) -> &mut Self::Output {
        let Index2D(x, y) = index;

        if !self.range_check(x, y) {
            panic!("Out of range index in mutable operation: {:?}", index)
        }

        let linear = self.linearize_index(x, y);
        &mut self.contents[linear]
    }
}