use std::ops::Range;

use crate::util::Index2D;

#[derive(Debug)]
struct Input {
    galaxies: Vec<Index2D>,
    rows: usize,
    columns: usize,
}

fn parse(input: &str) -> Input {
    let mut result = Input {
        galaxies: vec![],
        rows: 0,
        columns: 0,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                result.galaxies.push(Index2D(x as i32, y as i32))
            }
        }

        result.rows = y + 1;
        result.columns = line.len();
    }

    result
}

fn part1(input: &Input) -> u64 {
    let row_weights = (0..input.rows).map(|i| {
        if input.galaxies.iter().any(|Index2D(_, row)| *row as usize == i) {
            1
        } else {
            2
        }
    }).collect::<Vec<_>>();

    let col_weights = (0..input.columns).map(|i| {
        if input.galaxies.iter().any(|Index2D(col, _)| *col as usize == i) {
            1
        } else {
            2
        }
    }).collect::<Vec<_>>();

    compute_sum(input, &row_weights, &col_weights)
}

fn part2(input: &Input) -> u64 {
    let row_weights = (0..input.rows).map(|i| {
        if input.galaxies.iter().any(|Index2D(_, row)| *row as usize == i) {
            1
        } else {
            1000000
        }
    }).collect::<Vec<_>>();

    let col_weights = (0..input.columns).map(|i| {
        if input.galaxies.iter().any(|Index2D(col, _)| *col as usize == i) {
            1
        } else {
            1000000
        }
    }).collect::<Vec<_>>();

    compute_sum(input, &row_weights, &col_weights)
}

fn compute_sum(input: &Input, row_weights: &Vec<u64>, col_weights: &Vec<u64>) -> u64 {
    let mut sum = 0;
    for (off, g1) in input.galaxies[0..input.galaxies.len() - 1].into_iter().enumerate() {
        for g2 in input.galaxies[1 + off..].into_iter() {
            let dist = galaxy_distance(&row_weights, &col_weights, *g1, *g2);
            sum += dist
        }
    }

    sum
}

fn galaxy_distance(row_weights: &Vec<u64>, col_weights: &Vec<u64>, from: Index2D, to: Index2D) -> u64 {
    let start_x = from.0.min(to.0);
    let end_x = from.0.max(to.0);
    let start_y = from.1.min(to.1);
    let end_y = from.1.max(to.1);

    let horizontal = (start_x as usize)..(end_x as usize);
    let vertical = (start_y as usize)..(end_y as usize);

    let vertical_distance = range_dist(row_weights, vertical);
    let horizontal_distance = range_dist(col_weights, horizontal);

    vertical_distance + horizontal_distance
}

fn range_dist(weights: &Vec<u64>, range: Range<usize>) -> u64 {
    range.map(|x| weights[x]).sum()
}

simple_solution!(parse, part1, part2);