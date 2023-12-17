use std::ops::{Add, Index, IndexMut};
use geo::{Coord, CoordNum};
use crate::util::Direction::{EAST, NORTH, SOUTH, WEST};

pub struct IteratorIndex2D(usize, usize, usize);

pub trait TwoDimensional {

    fn rows(&self) -> usize;
    fn columns(&self) -> usize;

    fn indices(&self) -> IteratorIndex2D {
        IteratorIndex2D(0, self.rows() * self.columns(), self.columns())
    }

    fn bounds_check(&self, idx: Index2D) -> bool {
        idx.0 >= 0 && idx.1 >= 0 && idx.0 < self.columns() as i32 && idx.1 < self.rows() as i32
    }
}

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Index2D(pub i32, pub i32);


impl <T: CoordNum + From<i32>> Into<Coord<T>> for Index2D {
    fn into(self) -> Coord<T> {
        Coord {
            x: self.0.into(),
            y: self.1.into()
        }
    }
}

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

#[derive(Debug, Clone)]
pub struct Flat2DArray<T> {
    contents: Vec<T>,
    columns: usize,
    out_of_bounds_element: T,
}


impl<T> Flat2DArray<T> {
    pub fn from_data(out_of_bounds_element: T, contents: Vec<T>, columns: usize) -> Self {
        assert_eq!(contents.len() % columns, 0);

        Self { contents, columns, out_of_bounds_element }
    }

    fn range_check(&self, x: i32, y: i32) -> bool {
        let cols = self.columns as i32;
        let len = self.contents.len() as i32;
        (0..cols).contains(&x) && (0..len / cols).contains(&y)
    }

    pub fn transpose(&self) -> Transposed<T> {
        Transposed(self)
    }

    pub fn mapped_by<ToType>(&self, mut f: impl FnMut(&T) -> ToType) -> Flat2DArray<ToType> {
        let out_of_bounds_element = f(&self.out_of_bounds_element);
        let contents = self.contents.iter().map(f).collect();
        let columns = self.columns;

        Flat2DArray { contents, columns, out_of_bounds_element }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T>{
        self.contents.iter()
    }

    fn linearize_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.columns + x as usize
    }
}

impl <T> TwoDimensional for Flat2DArray<T> {
    fn rows(&self) -> usize {
        self.contents.len() / self.columns
    }

    fn columns(&self) -> usize {
        self.columns
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

pub struct Transposed<'a, T>(&'a Flat2DArray<T>);

impl <T> Index<Index2D> for Transposed<'_, T> {
    type Output = T;

    fn index(&self, index: Index2D) -> &Self::Output {
        let Index2D(x, y) = index;

        &self.0[Index2D(y, x)]
    }
}

impl <T> TwoDimensional for Transposed<'_, T> {
    fn rows(&self) -> usize {
        self.0.columns
    }

    fn columns(&self) -> usize {
        self.0.rows()
    }
}

impl Iterator for IteratorIndex2D {
    type Item = Index2D;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(offset, limit, columns) = self;

        if offset >= limit {
            return None
        }

        let this_offset = *offset;
        *offset += 1;
        let row = this_offset / *columns;
        let column = this_offset % *columns;

        Some(Index2D(column as i32, row as i32))
    }
}