use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};

use crate::util::parse_u64;

fn food_list(input: &str) -> IResult<&str, u64> {
    map(
        many1(parse_u64),
        |list| list.into_iter().sum(),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    Ok(separated_list1(tag("\n"), food_list)(&input)?)
}

fn part1(input: &Vec<u64>) -> u64 {
    input.iter().cloned().max().unwrap_or(0)
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
    buffer[1..].iter().cloned().sum()
}

solution!(parse, part1, part2);