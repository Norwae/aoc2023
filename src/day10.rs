use std::collections::HashSet;
use std::ops::{Add, Index, IndexMut};
use geo::{Contains, Coord, LineString, Polygon};

use nom::IResult;

pub use crate::util::Direction::{self, EAST, NORTH, SOUTH, WEST};
use crate::util::{Flat2DArray, Index2D};


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

fn solve_2(input: &Input) -> usize {

    let layout = &input.data;
    let (mut from, mut index) = first_step(input.start, &input.data);
    let mut outline = HashSet::new();
    outline.insert(input.start);
    let mut path = vec![input.start.into()];

    while index != input.start {
        let at_index = layout[index];
        let to = at_index.exit_opening(from).expect("pipe is not broken");

        from = to.opposite();
        index = index + to;
        outline.insert(index);
        path.push(index.into())
    }

    let mut count = 0;
    let poly = Polygon::new(LineString(path), Vec::new());

    for y in 0..layout.rows() {
        for x in 0..layout.columns() {
            let index = Index2D(x as i32, y as i32);
            let coord: Coord = index.into();

            if !outline.contains(&index) && poly.contains(&coord) {
                count += 1
            }

        }
    }

    count
}


solution!(parse, solve_1, solve_2);