use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::{fold_many1, separated_list1};

use crate::util::parse_u64_terminated;

fn food_list(input: &str) -> IResult<&str, u64> {
    fold_many1(
        parse_u64_terminated,
        ||0u64,
        |x, y| x + y
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(line_ending, food_list)(input)
}

fn part1(input: &Vec<u64>) -> u64 {
    let mut m = 0;
    for n in input {
        m = m.max(*n)
    }

    m
}

fn part2(input: &Vec<u64>) -> u64 {
    let mut buffer = [0; 4];

    for next in input {
        let next = *next;
        buffer[0] = next;
        let mut idx = 0;
        while idx < 3 && buffer[idx] > buffer[idx + 1] {
            buffer.swap(idx, idx + 1);
            idx += 1;
        }
    }
    buffer[1] + buffer[2] + buffer[3]
}

solution!(parse, part1, part2);