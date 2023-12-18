use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{line_ending, one_of, space1, u32};
use nom::character::is_hex_digit;
use nom::combinator::{map, map_res, value};
use nom::{AsChar, IResult};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::util::{Direction, Index2D};
use crate::util::Direction::{EAST, NORTH, SOUTH, WEST};

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    length: u32,
    color_code: u32,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(NORTH, tag("U")),
        value(EAST, tag("R")),
        value(SOUTH, tag("D")),
        value(WEST, tag("L"))
    ))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, DigInstruction> {
    map(tuple((
        parse_direction,
        space1,
        u32,
        tag(" (#"),
        map_res(take_while1(char::is_hex_digit),|hex: &str| u32::from_str_radix(hex, 16)),
        tag(")")
    )), |(direction, _, length, _, color_code, _)| DigInstruction { direction, length, color_code} )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<DigInstruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}


fn part1(input: &Vec<DigInstruction>) -> i32 {
    dbg!(input);
    10
}

nom_solution!(parse, part1);