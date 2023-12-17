use crate::util::{Direction, Flat2DArray, Index2D, TwoDimensional};
use Tile::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Outside,
    Empty,
    TiltCCW,
    TiltCW,
    SplitVertical,
    SplitHorizontal,
}

fn parse(input: &str) -> Flat2DArray<Tile> {
    let mut buffer = Vec::with_capacity(input.len());
    let mut line_length = usize::MAX;
    for line in input.lines() {
        for byte in line.bytes() {
            buffer.push(match byte {
                b'\\' => TiltCCW,
                b'/' => TiltCW,
                b'|' => SplitVertical,
                b'-' => SplitHorizontal,
                _ => Empty,
            });
            line_length = line.len()
        }
    }

    Flat2DArray::from_data(Outside, buffer, line_length)
}

fn trace(
    start: Index2D,
    start_direction: Direction,
    tiles: &Flat2DArray<Tile>,
    mut mark_visit_to_scratch: impl FnMut(Index2D, Direction, usize) -> bool,
) {
    let mut cursor_buffer = Vec::new();
    cursor_buffer.push((start, start_direction, 0usize));

    while let Some((mut position, mut direction, mut steps)) = cursor_buffer.pop() {
        while mark_visit_to_scratch(position, direction, steps) {
            let tile = tiles[position];
            direction = match tile {
                Empty => direction,
                TiltCW => match direction {
                    Direction::EAST => Direction::NORTH,
                    Direction::SOUTH => Direction::WEST,
                    Direction::WEST => Direction::SOUTH,
                    Direction::NORTH => Direction::EAST,
                }
                TiltCCW => match direction {
                    Direction::EAST => Direction::SOUTH,
                    Direction::SOUTH => Direction::EAST,
                    Direction::WEST => Direction::NORTH,
                    Direction::NORTH => Direction::WEST
                }
                SplitVertical => {
                    if direction == Direction::EAST || direction == Direction::WEST {
                        cursor_buffer.push((position, Direction::NORTH, steps));
                        Direction::SOUTH
                    } else {
                        direction
                    }
                }
                SplitHorizontal => {
                    if direction == Direction::NORTH || direction == Direction::SOUTH {
                        cursor_buffer.push((position, Direction::EAST, steps));
                        Direction::WEST
                    } else {
                        direction
                    }
                }
                Outside => unreachable!("already excluded via visited flag")
            };
            position = position + direction;
            steps += 1
        }
    }
}

fn part1(input: &Flat2DArray<Tile>) -> usize {
    let mut tracker = input.mapped_by(|tile| [false, false, false, false]);


    trace(
        Index2D(0, 0),
        Direction::EAST,
        input,
        |idx, d, _| {
            if !tracker.bounds_check(idx) {
                return false;
            }

            let flag = &mut tracker[idx][d as usize];
            if !*flag {
                *flag = true;
                true
            } else {
                false
            }
        },
    );


    tracker.iter().filter(|e| e.iter().any(|it| *it)).count()
}

fn part2(input: &Flat2DArray<Tile>) -> usize {
    let empty_tracker = input.mapped_by(|tile| [(false, 0usize), (false, 0), (false, 0), (false, 0)]);
    let best_score = 0;
    let best_trace = empty_tracker.clone();
    let rows = input.rows() as i32;
    let columns = input.columns() as i32;
    let starts = (0..rows).into_iter().map(|r| {
        (Index2D(0, r), Direction::EAST)
    }).chain(
        (0..columns).into_iter().map(|c| (Index2D(c, 0), Direction::SOUTH))
    ).chain(
        (0..rows).into_iter().map(|r| {
            (Index2D(columns - 1, r), Direction::WEST)
        })
    ).chain(
        (0..columns).into_iter().map(|c| (Index2D(c, rows - 1), Direction::NORTH))
    );

    starts.map(|(start, direction)|{
        let mut tracker = empty_tracker.clone();

        trace(
            start,
            direction,
            input,
            |idx, d, _| {
                if !tracker.bounds_check(idx) {
                    return false;
                }

                let flag = &mut tracker[idx][d as usize].0;
                if !*flag {
                    *flag = true;
                    true
                } else {
                    false
                }
            },
        );

        tracker.iter().filter(|e| e.iter().any(|it| it.0)).count()
    }).max().unwrap()
}

simple_solution!(parse, part1, part2);