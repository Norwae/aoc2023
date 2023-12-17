use crate::util::{Direction, Flat2DArray, Index2D, TwoDimensional};

use pathfinding::directed::astar::astar;
use crate::day17::ForcedDirection::{EITHER, HORIZONTAL, VERTICAL};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum ForcedDirection {
    EITHER,
    // Special case, start only
    HORIZONTAL,
    VERTICAL,
}

const HUGE: i32 = i32::MAX / 64;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Position {
    index: Index2D,
    direction: ForcedDirection,
}

fn parse(input: &str) -> Flat2DArray<i32> {
    let mut buffer = Vec::with_capacity(input.len());
    let mut line_length = usize::MAX;
    for line in input.lines() {
        for byte in line.bytes() {
            buffer.push((byte - b'0') as i32)
        }
        line_length = line.len();
    }

    Flat2DArray::from_data(HUGE, buffer, line_length)
}


fn successor_part_1<'a>(arr: &'a Flat2DArray<i32>) -> impl Fn(&Position) -> [(Position, i32); 6] + 'a {
    |pos| {
        let Position { index: Index2D(column, row), direction } = *pos;

        match direction {
            EITHER =>
                [
                    (Position { index: Index2D(1, 0),direction: VERTICAL}, arr[Index2D(1, 0)]),
                    (Position { index: Index2D(2, 0),direction: VERTICAL}, arr[Index2D(1, 0)] + arr[Index2D(2, 0)]),
                    (Position { index: Index2D(3, 0),direction: VERTICAL}, arr[Index2D(1, 0)] + arr[Index2D(2, 0)] + arr[Index2D(3, 0)]),
                    (Position { index: Index2D(0, 1),direction: VERTICAL}, arr[Index2D(0, 1)]),
                    (Position { index: Index2D(0, 2),direction: VERTICAL}, arr[Index2D(0, 1)] + arr[Index2D(0, 2)]),
                    (Position { index: Index2D(0, 3),direction: VERTICAL}, arr[Index2D(0, 1)] + arr[Index2D(0, 2)] + arr[Index2D(0, 3)]),
                ],
            HORIZONTAL => {
                let p1 = Index2D(column + 1, row);
                let p2 = Index2D(column + 2, row);
                let p3 = Index2D(column + 3, row);
                let m1 = Index2D(column - 1, row);
                let m2 = Index2D(column - 2, row);
                let m3 = Index2D(column - 3, row);

                [
                    (Position { index: p1,direction: VERTICAL}, arr[p1]),
                    (Position { index: p2,direction: VERTICAL}, arr[p1] + arr[p2]),
                    (Position { index: p3,direction: VERTICAL}, arr[p1] + arr[p2] + arr[p3]),
                    (Position { index: m1,direction: VERTICAL}, arr[m1]),
                    (Position { index: m2,direction: VERTICAL}, arr[m1] + arr[m2]),
                    (Position { index: m3,direction: VERTICAL}, arr[m1] + arr[m2] + arr[m3]),
                ]
            }
                ,
            VERTICAL => {
                let p1 = Index2D(column, row + 1);
                let p2 = Index2D(column, row + 2);
                let p3 = Index2D(column, row + 3);
                let m1 = Index2D(column, row - 1);
                let m2 = Index2D(column, row - 2);
                let m3 = Index2D(column, row - 3);

                [
                    (Position { index: p1,direction: HORIZONTAL}, arr[p1]),
                    (Position { index: p2,direction: HORIZONTAL}, arr[p1] + arr[p2]),
                    (Position { index: p3,direction: HORIZONTAL}, arr[p1] + arr[p2] + arr[p3]),
                    (Position { index: m1,direction: HORIZONTAL}, arr[m1]),
                    (Position { index: m2,direction: HORIZONTAL}, arr[m1] + arr[m2]),
                    (Position { index: m3,direction: HORIZONTAL}, arr[m1] + arr[m2] + arr[m3]),
                ]
            }
        }
    }
}



fn successor_part_2<'a>(arr: &'a Flat2DArray<i32>) -> impl Fn(&Position) -> [(Position, i32); 6] + 'a {
    |pos| {
        let Position { index: Index2D(column, row), direction } = *pos;

        match direction {
            EITHER =>
                [
                    (Position { index: Index2D(1, 0),direction: VERTICAL}, arr[Index2D(1, 0)]),
                    (Position { index: Index2D(2, 0),direction: VERTICAL}, arr[Index2D(1, 0)] + arr[Index2D(2, 0)]),
                    (Position { index: Index2D(3, 0),direction: VERTICAL}, arr[Index2D(1, 0)] + arr[Index2D(2, 0)] + arr[Index2D(3, 0)]),
                    (Position { index: Index2D(0, 1),direction: VERTICAL}, arr[Index2D(0, 1)]),
                    (Position { index: Index2D(0, 2),direction: VERTICAL}, arr[Index2D(0, 1)] + arr[Index2D(0, 2)]),
                    (Position { index: Index2D(0, 3),direction: VERTICAL}, arr[Index2D(0, 1)] + arr[Index2D(0, 2)] + arr[Index2D(0, 3)]),
                ],
            HORIZONTAL => {
                let p1 = Index2D(column + 1, row);
                let p2 = Index2D(column + 2, row);
                let p3 = Index2D(column + 3, row);
                let m1 = Index2D(column - 1, row);
                let m2 = Index2D(column - 2, row);
                let m3 = Index2D(column - 3, row);

                [
                    (Position { index: p1,direction: VERTICAL}, arr[p1]),
                    (Position { index: p2,direction: VERTICAL}, arr[p1] + arr[p2]),
                    (Position { index: p3,direction: VERTICAL}, arr[p1] + arr[p2] + arr[p3]),
                    (Position { index: m1,direction: VERTICAL}, arr[m1]),
                    (Position { index: m2,direction: VERTICAL}, arr[m1] + arr[m2]),
                    (Position { index: m3,direction: VERTICAL}, arr[m1] + arr[m2] + arr[m3]),
                ]
            }
            ,
            VERTICAL => {
                let p1 = Index2D(column, row + 1);
                let p2 = Index2D(column, row + 2);
                let p3 = Index2D(column, row + 3);
                let m1 = Index2D(column, row - 1);
                let m2 = Index2D(column, row - 2);
                let m3 = Index2D(column, row - 3);

                [
                    (Position { index: p1,direction: HORIZONTAL}, arr[p1]),
                    (Position { index: p2,direction: HORIZONTAL}, arr[p1] + arr[p2]),
                    (Position { index: p3,direction: HORIZONTAL}, arr[p1] + arr[p2] + arr[p3]),
                    (Position { index: m1,direction: HORIZONTAL}, arr[m1]),
                    (Position { index: m2,direction: HORIZONTAL}, arr[m1] + arr[m2]),
                    (Position { index: m3,direction: HORIZONTAL}, arr[m1] + arr[m2] + arr[m3]),
                ]
            }
        }
    }
}

fn part_1(input: &Flat2DArray<i32>) -> i32 {
    let rows = input.rows() as i32;
    let columns = input.columns() as i32;
    astar(&Position {
        index: Index2D(0, 0),
        direction: EITHER,
    }, successor_part_1(&input),
          |Position { index, .. }| rows - index.1 + columns - index.0,
          |Position { index, .. }| index.0 == columns - 1 && index.1 == rows - 1,
    ).unwrap().1
}

simple_solution!(parse, part_1);