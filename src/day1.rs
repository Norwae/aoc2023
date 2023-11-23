use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use crate::SimpleError;


fn food_list(input: &str) -> IResult<&str, usize> {
    map(
        many1(terminated(map_res(digit1, str::parse), tag("\n"))),
        |list: Vec<usize>| list.into_iter().sum(),
    )(input)
}

fn parse(input: &str) ->Result<(&str, Vec<usize>), SimpleError> {
    Ok(separated_list1(tag("\n"), food_list)(&input)?)
}

fn part1(input: &Vec<usize>) -> usize {
    input.iter().cloned().max().unwrap_or(0usize)
}

fn part2(input: &Vec<usize>) -> usize {
    let mut buffer = [0usize; 4];

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

pub fn solve(ctx: &mut crate::Context) {
    crate::solve(ctx, "day1", parse, part1, part2)
}