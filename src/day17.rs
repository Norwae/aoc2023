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
                    (Position { index: Index2D(0, 1),direction: HORIZONTAL}, arr[Index2D(0, 1)]),
                    (Position { index: Index2D(0, 2),direction: HORIZONTAL}, arr[Index2D(0, 1)] + arr[Index2D(0, 2)]),
                    (Position { index: Index2D(0, 3),direction: HORIZONTAL}, arr[Index2D(0, 1)] + arr[Index2D(0, 2)] + arr[Index2D(0, 3)]),
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



fn successor_part_2<'a>(arr: &'a Flat2DArray<i32>) -> impl Fn(&Position) -> [(Position, i32); 14] + 'a {
    |pos| {
        let Position { index: Index2D(column, row), direction } = *pos;

        match direction {
            EITHER => {
                let e4 = arr[Index2D(1, 0)] + arr[Index2D(2, 0)] + arr[Index2D(3, 0)] + arr[Index2D(4, 0)];
                let e5 = e4 + arr[Index2D(5, 0)];
                let e6 = e5 + arr[Index2D(6, 0)];
                let e7 = e6 + arr[Index2D(7, 0)];
                let e8 = e7 + arr[Index2D(8, 0)];
                let e9 = e8 + arr[Index2D(9, 0)];
                let e10 = e9 + arr[Index2D(10, 0)];
                let s4 = arr[Index2D(0, 1)] + arr[Index2D(0, 2)] + arr[Index2D(0, 3)] + arr[Index2D(0, 4)];
                let s5 = e4 + arr[Index2D(0, 5)];
                let s6 = e5 + arr[Index2D(0, 6)];
                let s7 = e6 + arr[Index2D(0, 7)];
                let s8 = e7 + arr[Index2D(0, 8)];
                let s9 = e8 + arr[Index2D(0, 9)];
                let s10 = e9 + arr[Index2D(0, 10)];
                [
                    (Position { index: Index2D(4, 0), direction: VERTICAL }, e4),
                    (Position { index: Index2D(5, 0), direction: VERTICAL }, e5),
                    (Position { index: Index2D(6, 0), direction: VERTICAL }, e6),
                    (Position { index: Index2D(7, 0), direction: VERTICAL }, e7),
                    (Position { index: Index2D(8, 0), direction: VERTICAL }, e8),
                    (Position { index: Index2D(9, 0), direction: VERTICAL }, e9),
                    (Position { index: Index2D(10, 0), direction: VERTICAL }, e10),
                    (Position { index: Index2D(0, 4), direction: HORIZONTAL }, s4),
                    (Position { index: Index2D(0, 5), direction: HORIZONTAL }, s5),
                    (Position { index: Index2D(0, 6), direction: HORIZONTAL }, s6),
                    (Position { index: Index2D(0, 7), direction: HORIZONTAL }, s7),
                    (Position { index: Index2D(0, 8), direction: HORIZONTAL }, s8),
                    (Position { index: Index2D(0, 9), direction: HORIZONTAL }, s9),
                    (Position { index: Index2D(0, 10), direction: HORIZONTAL }, s10),
                ]
            }
            HORIZONTAL => {
                let p4 = Index2D(column + 4, row);
                let p5 = Index2D(column + 5, row);
                let p6 = Index2D(column + 6, row);
                let p7 = Index2D(column + 7, row);
                let p8 = Index2D(column + 8, row);
                let p9 = Index2D(column + 9, row);
                let p10 = Index2D(column + 10, row);
                let m4 = Index2D(column - 4, row);
                let m5 = Index2D(column - 5, row);
                let m6 = Index2D(column - 6, row);
                let m7 = Index2D(column - 7, row);
                let m8 = Index2D(column - 8, row);
                let m9 = Index2D(column - 9, row);
                let m10 = Index2D(column - 10, row);
                let cp4 = arr[Index2D(column + 1, row)] + arr[Index2D(column + 2, row)] + arr[Index2D(column + 3, row)] + arr[p4];
                let cp5 = cp4 + arr[p5];
                let cp6 = cp5 + arr[p6];
                let cp7 = cp6 + arr[p7];
                let cp8 = cp7 + arr[p8];
                let cp9 = cp8 + arr[p9];
                let cp10 = cp9 + arr[p10];
                let cm4 = arr[Index2D(column - 1, row)] + arr[Index2D(column - 2, row)] + arr[Index2D(column - 3, row)] + arr[m4];
                let cm5 = cm4 + arr[m5];
                let cm6 = cm5 + arr[m6];
                let cm7 = cm6 + arr[m7];
                let cm8 = cm7 + arr[m8];
                let cm9 = cm8 + arr[m9];
                let cm10 = cm9 + arr[m10];

                [
                    (Position { index: p4, direction: VERTICAL }, cp4),
                    (Position { index: p5, direction: VERTICAL }, cp5),
                    (Position { index: p6, direction: VERTICAL }, cp6),
                    (Position { index: p7, direction: VERTICAL }, cp7),
                    (Position { index: p8, direction: VERTICAL }, cp8),
                    (Position { index: p9, direction: VERTICAL }, cp9),
                    (Position { index: p10, direction: VERTICAL }, cp10),
                    (Position { index: m4, direction: VERTICAL }, cm4),
                    (Position { index: m5, direction: VERTICAL }, cm5),
                    (Position { index: m6, direction: VERTICAL }, cm6),
                    (Position { index: m7, direction: VERTICAL }, cm7),
                    (Position { index: m8, direction: VERTICAL }, cm8),
                    (Position { index: m9, direction: VERTICAL }, cm9),
                    (Position { index: m10, direction: VERTICAL }, cm10),
                ]
            }
            VERTICAL => {
                let p4 = Index2D(column, row + 4);
                let p5 = Index2D(column, row + 5);
                let p6 = Index2D(column, row + 6);
                let p7 = Index2D(column, row + 7);
                let p8 = Index2D(column, row + 8);
                let p9 = Index2D(column, row + 9);
                let p10 = Index2D(column, row + 10);
                let m4 = Index2D(column, row - 4);
                let m5 = Index2D(column, row - 5);
                let m6 = Index2D(column, row - 6);
                let m7 = Index2D(column, row - 7);
                let m8 = Index2D(column, row - 8);
                let m9 = Index2D(column, row - 9);
                let m10 = Index2D(column, row - 10);
                let cp4 = arr[Index2D(column, row + 1)] + arr[Index2D(column, row + 2)] + arr[Index2D(column, row + 3)] + arr[p4];
                let cp5 = cp4 + arr[p5];
                let cp6 = cp5 + arr[p6];
                let cp7 = cp6 + arr[p7];
                let cp8 = cp7 + arr[p8];
                let cp9 = cp8 + arr[p9];
                let cp10 = cp9 + arr[p10];
                let cm4 = arr[Index2D(column, row - 1)] + arr[Index2D(column, row - 2)] + arr[Index2D(column, row - 3)] + arr[m4];
                let cm5 = cm4 + arr[m5];
                let cm6 = cm5 + arr[m6];
                let cm7 = cm6 + arr[m7];
                let cm8 = cm7 + arr[m8];
                let cm9 = cm8 + arr[m9];
                let cm10 = cm9 + arr[m10];

                [
                    (Position { index: p4, direction: HORIZONTAL }, cp4),
                    (Position { index: p5, direction: HORIZONTAL }, cp5),
                    (Position { index: p6, direction: HORIZONTAL }, cp6),
                    (Position { index: p7, direction: HORIZONTAL }, cp7),
                    (Position { index: p8, direction: HORIZONTAL }, cp8),
                    (Position { index: p9, direction: HORIZONTAL }, cp9),
                    (Position { index: p10, direction: HORIZONTAL }, cp10),
                    (Position { index: m4, direction: HORIZONTAL }, cm4),
                    (Position { index: m5, direction: HORIZONTAL }, cm5),
                    (Position { index: m6, direction: HORIZONTAL }, cm6),
                    (Position { index: m7, direction: HORIZONTAL }, cm7),
                    (Position { index: m8, direction: HORIZONTAL }, cm8),
                    (Position { index: m9, direction: HORIZONTAL }, cm9),
                    (Position { index: m10, direction: HORIZONTAL }, cm10),
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


fn part_2(input: &Flat2DArray<i32>) -> i32 {
    let rows = input.rows() as i32;
    let columns = input.columns() as i32;
    astar(&Position {
        index: Index2D(0, 0),
        direction: EITHER,
    }, successor_part_2(&input),
          |Position { index, .. }| rows - index.1 + columns - index.0,
          |Position { index, .. }| index.0 == columns - 1 && index.1 == rows - 1,
    ).unwrap().1
}

simple_solution!(parse, part_1,  part_2);