use geo::{Area, Coord, LineString, Polygon};
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{i32, line_ending, space1};
use nom::combinator::{map, map_res, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

use crate::util::{Direction, Index2D};
use crate::util::Direction::{EAST, NORTH, SOUTH, WEST};

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    length: i32
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(NORTH, tag("U")),
        value(EAST, tag("R")),
        value(SOUTH, tag("D")),
        value(WEST, tag("L"))
    ))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, (DigInstruction, DigInstruction)> {
    map(tuple((
        parse_direction,
        space1,
        i32,
        tag(" (#"),
        parse_pseudocolor,
        tag(")")
    )), |(direction, _, length, _, instruction2, _)| (DigInstruction { direction, length }, instruction2))(input)
}

fn parse_pseudocolor(input: &str) -> IResult<&str, DigInstruction> {
    map_res(tuple((
        take(5usize),
        alt((
            value(EAST, tag("0")),
            value(SOUTH, tag("1")),
            value(WEST, tag("2")),
            value(NORTH, tag("3"))
        ))
    )), |(digits, direction)| {
        i32::from_str_radix(digits, 16).map(|length| DigInstruction { length, direction })
    })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(DigInstruction, DigInstruction)>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn area<'a>(it: impl Iterator<Item=&'a DigInstruction>) -> f64 {
    const DELTAS: [Coord; 4] = [Coord { x: 1., y: 0.}, Coord { x: 0., y: 1.}, Coord { x: -1., y: 0.}, Coord { x: 0., y: -1.}];
    let mut cursor = Coord { x: 0., y: 0. };
    let mut nodes = vec![cursor];
    let mut correction = 1f64;

    for DigInstruction { direction, length, .. } in it {
        let direction = *direction;
        let length = *length as f64;

        if direction == SOUTH || direction == WEST {
            correction += length
        }
        cursor = cursor + DELTAS[direction as usize] * length;
        nodes.push(cursor.into())
    }

    let poly = Polygon::new(LineString(nodes), Vec::new());


    poly.signed_area() + correction as f64
}


fn part1(input: &Vec<(DigInstruction, DigInstruction)>) -> f64 {
    area(input.iter().map(|(i, _)|i))
}

fn part2(input: &Vec<(DigInstruction, DigInstruction)>) -> f64 {
    area(input.iter().map(|(_, i)|i))
}

nom_solution!(parse, part1, part2);