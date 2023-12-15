use std::mem::swap;
use std::ops::Index;

use crate::util::{Flat2DArray, Index2D, TwoDimensional};

trait ReflectionLineCandidate {
    fn index(&self) -> i32;
    fn see_line(&mut self, mismatches: usize) -> bool;
}

impl ReflectionLineCandidate for i32 {
    fn index(&self) -> i32 {
        *self
    }

    fn see_line(&mut self, mismatches: usize) -> bool {
        mismatches > 0
    }
}

impl ReflectionLineCandidate for (i32, bool) {
    fn index(&self) -> i32 {
        self.0
    }

    fn see_line(&mut self, mismatches: usize) -> bool {
        if mismatches == 0 {
            false
        } else if mismatches == 1 && !self.1 {
            self.1 = true;
            false
        } else {
            true
        }
    }
}

fn parse_maps(input: &str) -> Vec<Flat2DArray<bool>> {
    let mut target = Vec::new();
    let mut this_width = usize::MAX;
    let mut buffer = Vec::new();

    for line in input.lines() {
        if line == "" {
            let map = Flat2DArray::from_data(false, buffer.clone(), this_width);
            target.push(map);
            buffer.clear();
            this_width = usize::MAX;
            continue;
        }

        let bytes = line.as_bytes();
        if this_width == usize::MAX {
            this_width = bytes.len();
        }

        buffer.extend(bytes.iter().map(|x| *x == b'#'))
    }

    if !buffer.is_empty() {
        let map = Flat2DArray::from_data(false, buffer.clone(), this_width);
        target.push(map);
    }

    target
}

fn find_reflection<Out: Eq + Copy, Container: Index<Index2D, Output=Out> + TwoDimensional>(data: &Container) -> Vec<i32> {
    let rows = data.rows() as i32;
    let columns = data.columns() as i32;
    let mut reflection_columns = (1..columns).collect();
    for row in 0..rows {
        find_reflections_in_row(data, row, &mut reflection_columns)
    }

    reflection_columns
}

fn find_near_reflection<Out: Eq + Copy, Container: Index<Index2D, Output=Out> + TwoDimensional>(data: &Container) -> Vec<i32> {
    let rows = data.rows() as i32;
    let columns = data.columns() as i32;
    let mut reflection_columns = (1..columns).map(|n| (n, false)).collect();
    for row in 0..rows {
        find_reflections_in_row(data, row, &mut reflection_columns)
    }

    reflection_columns.into_iter().filter_map(|(n, defect)| if defect {
        Some(n)
    } else {
        None
    }).collect()
}

fn find_reflections_in_row<
    Out: Eq + Copy,
    Container: Index<Index2D, Output=Out> + TwoDimensional,
    Candidate: ReflectionLineCandidate
>(data: &Container, row: i32, candidates: &mut Vec<Candidate>) {
    let mut my_candidates = Vec::new();
    swap(&mut my_candidates, candidates);
    let columns = data.columns() as i32;
    for mut cut in my_candidates {
        let cut_idx = cut.index();
        let left_range = 0..cut_idx;
        let right_range = cut_idx..columns;
        let left = left_range.map(|x| Index2D(x, row)).rev();
        let right = right_range.map(|x| Index2D(x, row));

        let mismatches =
            left
                .zip(right)
                .map(|(l, r)| (data[l], data[r]))
                .filter(|(l, r)| l != r)
                .count();
        if !cut.see_line(mismatches) {
            candidates.push(cut)
        }
    }
}

fn solve_part1(input: &Vec<Flat2DArray<bool>>) -> i32 {
    let mut sum = 0;

    for input in input {
        for vertical in find_reflection(input) {
            sum += vertical
        }
        let transposed = input.transpose();
        for horizontal in find_reflection(&transposed) {
            sum += 100 * horizontal
        }
    }

    sum
}


fn solve_part2(input: &Vec<Flat2DArray<bool>>) -> i32 {
    let mut sum = 0;

    for input in input {
        for vertical in find_near_reflection(input) {
            sum += vertical
        }
        let transposed = input.transpose();
        for horizontal in find_near_reflection(&transposed) {
            sum += 100 * horizontal
        }
    }

    sum
}
simple_solution!(parse_maps, solve_part1, solve_part2);