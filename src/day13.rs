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

fn find_reflection<Out: Eq + Copy, Container : Index<Index2D, Output=Out> + TwoDimensional>(data: &Container) -> Option<i32> {
    (0..data.rows() as i32).into_iter().fold(Some(1), |start, row|{
        if let Some(start) = start {
            find_reflection_in_row(data, row, start)
        } else {
            None
        }
    })
}

fn find_reflection_in_row<Out: Eq + Copy, Container: Index<Index2D, Output=Out> + TwoDimensional>(data: &Container, row: i32, start_hint: i32) -> Option<i32> {
    let columns = data.columns() as i32;
    for cut in start_hint..columns {
        let left_range = 0..cut;
        let right_range = cut..columns;
        let left = left_range.map(|x|Index2D(x, row)).rev();
        let right = right_range.map(|x|Index2D(x, row));

        let mut zip = left.zip(right).map(|(l, r)|(data[l], data[r]));

        if zip.all(|(l, r)| l == r) {
            return Some(cut);
        }
    }

    None
}

fn solve_part1(input: &Vec<Flat2DArray<bool>>) -> i32 {
    let mut sum = 0;

    for input in input {
        if let Some(line) = find_reflection(input) {
            sum += line
        } else {
            let transposed = input.transpose();
            let line = find_reflection(&transposed).expect("vertical reflection required");
            sum += 100 * line
        }
    }

    sum
}

solution!(parse_maps, solve_part1);