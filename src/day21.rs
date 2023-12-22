use std::collections::{HashSet, VecDeque};
use crate::util::{Direction, Flat2DArray, Index2D};
use crate::util::Direction::{EAST, NORTH, SOUTH, WEST};

#[derive(Debug)]
struct Input {
    rocks: HashSet<Index2D>,
}

fn parse(input: &str) -> Input {
    let mut rocks = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'S' {} else if byte == b'#' {
                rocks.insert(Index2D(y as i32, x as i32));
            }
        }
    }

    Input { rocks }
}

fn part1(input: &Input) -> usize {
    let queue = VecDeque::from([(Index2D(65, 65), 64, 0)]);
    consume_queue_double_steps(&input.rocks, queue, |x| x).1
}

fn part2(input: &Input) -> usize {
    // 131 should do it too, but I want it to starve of input to avoid any weird corner cases
    let queue = VecDeque::from([(Index2D(65, 65), 9999, 0)]);
    let flood_filled = consume_queue_double_steps(&input.rocks, queue, |Index2D(mut y, mut x): Index2D| {
        if x < 0 {
            x += 131;
        }
        if y < 0 {
            y += 131
        }

        if x >= 131 {
            x -= 131
        }

        if y >= 131 {
            y -= 131
        }

        Index2D(y, x)
    }).0;

    42
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum VisitStatus {
    Distance(usize),
    Unvisited,
}

fn consume_queue_double_steps(rocks: &HashSet<Index2D>, mut queue: VecDeque<(Index2D, i32, usize)>, shift: impl Fn(Index2D) -> Index2D) -> (Flat2DArray<VisitStatus>, usize) {
    let mut record = Flat2DArray::from_data(VisitStatus::Unvisited, vec![VisitStatus::Unvisited; 131 * 131], 131);
    let mut count = 0usize;

    while let Some((position, ttl, steps_taken)) = queue.pop_front() {
        for d1 in Direction::ALL {
            let position = shift(position + d1);
            if !rocks.contains(&position) && record[position] == VisitStatus::Unvisited {
                record[position] = VisitStatus::Distance(steps_taken + 1);

                for d2 in Direction::ALL {
                    let position = shift(position + d2);


                    if !rocks.contains(&position)
                        && record[position] == VisitStatus::Unvisited {
                        count += 1;
                        record[position] = VisitStatus::Distance(steps_taken + 2);
                        if ttl > 2 {
                            queue.push_back((position, ttl - 2, steps_taken + 2))
                        }
                    }
                }
            }
        }
    }

    (record, count)
}

simple_solution!(parse, part1, part2);