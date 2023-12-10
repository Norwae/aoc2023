use std::ops::{Add, Index, IndexMut};

use nom::IResult;

use crate::day10::Direction::{EAST, NORTH, SOUTH, WEST};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

impl Direction {
    const ALL: [Direction; 4] = [EAST, SOUTH, WEST, NORTH];

    fn opposite(self) -> Self {
        match self {
            EAST => WEST,
            SOUTH => NORTH,
            WEST => EAST,
            NORTH => SOUTH,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Index2D(i32, i32);

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
struct Flat2DArray<T> {
    contents: Vec<T>,
    columns: usize,
    out_of_bounds_element: T,
}

impl<T> Flat2DArray<T> {
    fn from_data(out_of_bounds_element: T, contents: Vec<T>, columns: usize) -> Self {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PipeSegment {
    Horizontal,
    Vertical,
    WestToSouth,
    SouthToEast,
    EastToNorth,
    NorthToWest,
    Starter,
    Ground,
}

impl PipeSegment {
    fn is_open(self, d: Direction) -> bool {
        static OPEN: [bool; 32] = [
            // horizontal, open to EAST and WEST
            true, false, true, false,
            // vertical
            false, true, false, true,
            // west-south
            false, true, true, false,
            // south-east
            true, true, false, false,
            // east-north
            true, false, false, true,
            // north-west
            false, false, true, true,
            // starter - all
            true, true, true, true,
            // ground
            false, false, false, false
        ];
        let index = self as usize * 4 + d as usize;

        OPEN[index]
    }

    fn exit_opening(&self, input: Direction) -> Option<Direction> {
        match self {
            PipeSegment::Horizontal if input == WEST => Some(EAST),
            PipeSegment::Horizontal if input == EAST => Some(WEST),
            PipeSegment::Vertical if input == NORTH => Some(SOUTH),
            PipeSegment::Vertical if input == SOUTH => Some(NORTH),
            PipeSegment::WestToSouth if input == WEST => Some(SOUTH),
            PipeSegment::WestToSouth if input == SOUTH => Some(WEST),
            PipeSegment::SouthToEast if input == SOUTH => Some(EAST),
            PipeSegment::SouthToEast if input == EAST => Some(SOUTH),
            PipeSegment::EastToNorth if input == NORTH => Some(EAST),
            PipeSegment::EastToNorth if input == EAST => Some(NORTH),
            PipeSegment::NorthToWest if input == NORTH => Some(WEST),
            PipeSegment::NorthToWest if input == WEST => Some(NORTH),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Input {
    data: Flat2DArray<PipeSegment>,
    start: Index2D,
}

fn parse(input: &str) -> IResult<&str, Input> {
    let mut columns = usize::MAX;
    let mut start = Index2D(-1, -1);
    let mut data = Vec::new();

    for (y, line) in input.lines().enumerate() {
        if columns == usize::MAX {
            columns = line.len();
        }

        for (x, byte) in line.bytes().enumerate() {
            let next = match byte {
                b'|' => PipeSegment::Vertical,
                b'-' => PipeSegment::Horizontal,
                b'7' => PipeSegment::WestToSouth,
                b'F' => PipeSegment::SouthToEast,
                b'L' => PipeSegment::EastToNorth,
                b'J' => PipeSegment::NorthToWest,
                b'S' => {
                    start = Index2D(x as i32, y as i32);
                    PipeSegment::Starter
                }
                _ => {
                    PipeSegment::Ground
                }
            };

            data.push(next)
        }
    }

    Ok(("", Input {
        data: Flat2DArray::from_data(PipeSegment::Ground, data, columns),
        start,
    }))
}

fn first_step(start: Index2D, layout: &Flat2DArray<PipeSegment>) -> (Direction, Index2D) {

    for first_step in Direction::ALL {
        let stepped = start + first_step;

        if layout[stepped].is_open(first_step.opposite()) {
            return (first_step.opposite(), stepped)
        }
    }

    unreachable!()
}

fn solve_1(input: &Input) -> usize {
    let layout = &input.data;
    let (mut from, mut index) = first_step(input.start, &input.data);
    let mut count = 0;

    while index != input.start {
        let to = layout[index].exit_opening(from).expect("pipe is not broken");

        from = to.opposite();
        index = index + to;
        count += 1
    }

    (count + 1) / 2
}


solution!(parse, solve_1);