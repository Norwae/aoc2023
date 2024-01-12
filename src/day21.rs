use std::cell::{Ref, RefCell};
use std::collections::{HashSet, VecDeque};

use crate::util::{Direction, Flat2DArray, Index2D, TwoDimensional};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Reachability {
    Outside,
    Unreachable,
    Reachable(usize),
}

#[derive(Debug)]
struct Input {
    rocks: HashSet<Index2D>,
    step_counts: RefCell<Option<Flat2DArray<Reachability>>>,
}

impl Input {
    fn step_counts(&self) -> Ref<Flat2DArray<Reachability>> {
        let mut step_counts = self.step_counts.borrow_mut();
        step_counts.get_or_insert_with(|| {
            build_map_for_rocks(&self.rocks)
        });

        drop(step_counts);

        let step_counts = self.step_counts.borrow();

        Ref::map(step_counts, |sc| sc.as_ref().unwrap())
    }
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

    Input { rocks, step_counts: RefCell::new(None) }
}

fn part1(input: &Input) -> usize {
    let step_counts = input.step_counts();

    step_counts.as_slice().iter().filter(|n| if let Reachability::Reachable(n) = n {
        let n = *n;
        n <= 64 && n % 2 == 0
    } else {
        false
    }).count()
}

fn part2(input: &Input) -> usize {
    let step_counts = input.step_counts();
    let mut center_diamond_even = 0usize;
    let mut center_diamond_odd = 0usize;
    let mut corner_diamond_even = 0usize;
    let mut corner_diamond_odd = 0usize;

    for r in step_counts.as_slice() {
        if let Reachability::Reachable(n) = r {
            let n = *n;
            let target = if n % 2 == 0 {
                if n <= 64 {
                    &mut center_diamond_even
                } else {
                    &mut center_diamond_odd
                }
            } else {
                if n <= 64 {
                    &mut corner_diamond_even
                } else {
                    &mut corner_diamond_odd
                }
            };
            *target += 1;
        }
    }

    let full_steps = 26501300usize;

    let center_diamonds_odd = (full_steps + 1) * (full_steps + 1);
    let center_diamonds_even = full_steps * full_steps;
    let corner_diamonds = full_steps * full_steps + full_steps - 1;

    center_diamonds_odd * center_diamond_odd +
        center_diamonds_even + center_diamond_even +
        corner_diamonds * corner_diamond_even // equal value to odd
}

fn build_map_for_rocks(rocks: &HashSet<Index2D>) -> Flat2DArray<Reachability> {
    let mut queue = VecDeque::from([(Index2D(65, 65), 0)]);
    let mut reachability = Flat2DArray::from_data(Reachability::Outside, vec![Reachability::Unreachable; 131 * 131], 131);

    while let Some((idx, steps)) = queue.pop_front() {
        if reachability[idx] != Reachability::Unreachable {
            continue;
        }
        reachability[idx] = Reachability::Reachable(steps);

        for d in Direction::ALL {
            let neighbour = idx + d;
            if !rocks.contains(&neighbour) {
                queue.push_back((neighbour, steps + 1))
            }
        }
    }

    reachability
}


simple_solution!(parse, part1, part2);