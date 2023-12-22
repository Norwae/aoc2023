use std::collections::{HashSet, VecDeque};
use crate::util::{Direction, Flat2DArray, Index2D};
use crate::util::Direction::{EAST, NORTH, SOUTH, WEST};

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
    let queue = VecDeque::from([(input.start, 64)]);
    consume_queue_double_steps(&input.rocks, queue, |x|x)
}

fn part2(input: &Input) -> usize {
    let central_diamond = consume_queue_double_steps(&input.rocks, VecDeque::from([
        (input.start + NORTH, 64),
        (input.start + EAST, 64),
        (input.start + WEST, 64),
        (input.start + SOUTH, 64),
    ]), |x|x);

    let frame_shift = |Index2D(mut y, mut x): Index2D| {
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
    };

    let ul = Index2D(0, 0);
    let ul_diamond = consume_queue_double_steps(&input.rocks, VecDeque::from([
        (ul + NORTH, 64),
        (ul + EAST, 64),
        (ul + WEST, 64),
        (ul + SOUTH, 64),
    ]), frame_shift);

    let br = Index2D(130, 130);
    let br_diamond = consume_queue_double_steps(&input.rocks, VecDeque::from([
        (br + NORTH, 64),
        (br + EAST, 64),
        (br + WEST, 64),
        (br + SOUTH, 64),
    ]), frame_shift);
    let bl = Index2D(130, 0);
    let bl_diamond = consume_queue_double_steps(&input.rocks, VecDeque::from([
        (bl + NORTH, 64),
        (bl + EAST, 64),
        (bl + WEST, 64),
        (bl + SOUTH, 64),
    ]), frame_shift);

    let ur = Index2D(0, 130);
    let ur_diamond = consume_queue_double_steps(&input.rocks, VecDeque::from([
        (ur + NORTH, 64),
        (ur + EAST, 64),
        (ur + WEST, 64),
        (ur + SOUTH, 64),
    ]), frame_shift);

    // okay, we start off with 65 steps, getting a central diamond
    let steps = 26501365;
    let steps = steps - 65;
    let full_traversals = steps / 131;
}

fn consume_queue_double_steps(rocks: &HashSet<Index2D>, mut queue: VecDeque<(Index2D, i32)>, shift: impl Fn(Index2D) -> Index2D) -> usize {
    let mut record = Flat2DArray::<bool>::new(131, 131);
    let mut count = 0usize;

    while let Some((position, steps)) = queue.pop_front() {
        for d1 in Direction::ALL {
            let position = shift(position + d1);
            let steps = steps - 1;
            if !rocks.contains(&position) {
                for d2 in Direction::ALL {
                    let position = shift(position + d2);
                    let steps = steps - 1;

                    if !rocks.contains(&position)
                        && !record[position]{
                        count += 1;
                        record[position] = true;
                        if steps > 0 {
                            queue.push_back((position, steps))
                        }
                    }
                }
            }
        }
    }

    count
}

simple_solution!(parse, part1, part2);