use std::collections::{HashSet, VecDeque};
use crate::util::{Direction, Index2D};

#[derive(Debug)]
struct Input {
    start: Index2D,
    limit: Index2D,
    rocks: HashSet<Index2D>,
}

fn parse(input: &str) -> Input {
    let mut start: Index2D = Index2D(-1, -1);
    let mut limit = Index2D(0, 0);
    let mut rocks = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'S' {
                start = Index2D(y as i32, x as i32);
            } else if byte == b'#' {
                rocks.insert(Index2D(y as i32, x as i32));
            }
        }

        limit.0 = line.len() as i32;
        limit.1 += 1;
    }

    Input { start, limit, rocks }
}

fn part1(input: &Input) -> usize {
    let mut queue = VecDeque::from([(input.start, 64)]);
    let mut ending_positions = HashSet::new();

    while let Some((position, steps)) = queue.pop_front() {
        for d1 in Direction::ALL {
            let position = position + d1;
            let steps = steps - 1;
            if !input.rocks.contains(&position) {
                for d2 in Direction::ALL {
                    let position = position + d2;
                    let steps = steps - 1;

                    if !input.rocks.contains(&position)
                        && ending_positions.insert(position)
                        && steps > 0
                    {
                        queue.push_back((position, steps))
                    }
                }
            }
        }
    }

    ending_positions.len()
}

simple_solution!(parse, part1);