use std::ops::Index;
use nom::IResult;
use crate::util::{Flat2DArray, Index2D, TwoDimensional};

fn parse_maps(input: &str) -> IResult<&str, Vec<Flat2DArray<bool>>> {
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

    Ok(("", target))
}

fn find_reflection<Out: Eq + Copy, Container : Index<Index2D, Output=Out> + TwoDimensional>(data: &Container) -> Vec<i32> {
    let rows = data.rows() as i32;
    let columns = data.columns() as i32;
    let mut reflection_columns = (1..columns).collect();
    for row in 0..rows {
        find_reflections_in_row(data, row, &mut reflection_columns)
    }

    reflection_columns
}

fn find_reflections_in_row<Out: Eq + Copy, Container: Index<Index2D, Output=Out> + TwoDimensional>(data: &Container, row: i32, candidates: &mut Vec<i32>) {
    let mut next_candidates = Vec::new();
    let columns = data.columns() as i32;
    for cut in candidates.iter() {
        let cut = *cut;
        let left_range = 0..cut;
        let right_range = cut..columns;
        let left = left_range.map(|x|Index2D(x, row)).rev();
        let right = right_range.map(|x|Index2D(x, row));

        let mut zip = left.zip(right).map(|(l, r)|(data[l], data[r]));

        if zip.all(|(l, r)| l == r) {
            next_candidates.push(cut);
        }
    };

    *candidates = next_candidates;
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

solution!(parse_maps, solve_part1);