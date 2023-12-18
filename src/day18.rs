use geo::{Area, Coord, LineString, Polygon};
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{i32, line_ending, space1};
use nom::combinator::{map, map_res, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

#[derive(Debug)]
struct DigInstruction {
    delta: Coord,
}


fn parse_direction(input: &str) -> IResult<&str, Coord> {
    alt((
        value(Coord { x: 0., y: -1. }, tag("U")),
        value(Coord { x: 1., y: 0. }, tag("R")),
        value(Coord { x: 0., y: 1. }, tag("D")),
        value(Coord { x: -1., y: 0. }, tag("L"))
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
    )), |(direction, _, length, _, instruction2, _)| (DigInstruction { delta:  direction * length as f64 }, instruction2))(input)
}

fn parse_pseudocolor(input: &str) -> IResult<&str, DigInstruction> {
    map_res(tuple((
        take(5usize),
        alt((
            value(Coord { x: 1., y: 0.}, tag("0")),
            value(Coord { x: 0., y: 1.}, tag("1")),
            value(Coord { x: -1., y: 0.}, tag("2")),
            value(Coord { x: 0., y: -1.}, tag("3"))
        ))
    )), |(digits, direction)| {
        i32::from_str_radix(digits, 16).map(|length| DigInstruction { delta: direction * length as f64  })
    })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(DigInstruction, DigInstruction)>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn area<'a>(it: impl Iterator<Item=&'a DigInstruction>) -> f64 {
    let mut cursor = Coord { x: 0., y: 0. };
    let mut nodes = Vec::with_capacity(1 + it.size_hint().0);
    nodes.push(cursor);

    let mut correction = 1f64;

    for DigInstruction { delta } in it {
        if  delta.x < 0. {
            correction -= delta.x
        }

        if delta.y > 0. {
            correction += delta.y
        }

        cursor = cursor + *delta;
        nodes.push(cursor.into())
    }

    let poly = Polygon::new(LineString(nodes), Vec::new());

    poly.signed_area() + correction
}


fn part1(input: &Vec<(DigInstruction, DigInstruction)>) -> f64 {
    area(input.iter().map(|(i, _)| i))
}

fn part2(input: &Vec<(DigInstruction, DigInstruction)>) -> f64 {
    area(input.iter().map(|(_, i)| i))
}

nom_solution!(parse, part1, part2);