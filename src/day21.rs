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

simple_solution!(parse, part1);