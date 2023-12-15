use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};
use std::hash::Hash;
use std::str::FromStr;

use nom::bytes::complete::{tag, take, take_while1};
use nom::character::complete::line_ending;
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::fold_many1;
use nom::sequence::{terminated, tuple};

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    map_nodes: HashMap<Tag, (Tag, Tag)>,
}

impl Input {
    fn new(directions: Vec<Direction>, map_nodes: HashMap<Tag, (Tag, Tag)>) -> Self {
        Input {
            directions,
            map_nodes,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Tag(u64);

impl Tag {
    fn ends_with(&self, ch: u8) -> bool {
        (self.0 & 0xff) as u8 == ch - b'A'
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tag(")?;
        <Self as Display>::fmt(self, f)?;
        f.write_str(")")
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c3 = b'A' + (((self.0 & 0xff) >> 0) as u8);
        let c2 = b'A' + (((self.0 & 0xff00) >> 8) as u8);
        let c1 = b'A' + (((self.0 % 0xff0000) >> 16) as u8);

        f.write_char(c1 as char)?;
        f.write_char(c2 as char)?;
        f.write_char(c3 as char)
    }
}

impl FromStr for Tag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        let valid = b'A'..=b'Z';

        if b.len() == 3 && valid.contains(&b[0]) && valid.contains(&b[1]) && valid.contains(&b[2]) {
            let num = ((b[0] - b'A') as u64 * 65536) + ((b[1] - b'A') as u64 * 256) + (b[2] - b'A') as u64;
            Ok(Tag(num))
        } else {
            dbg!(Err(format!("invalid input: {}", s)))
        }
    }
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    map(
        take_while1(|next| next == 'L' || next == 'R'),
        |str: &str| {
            str.bytes().map(|next| if next == b'L' { Direction::Left } else { Direction::Right }).collect()
        },
    )(input)
}

fn mapping_tag(input: &str) -> IResult<&str, Tag> {
    map_res(take(3usize), Tag::from_str)(input)
}

fn mapping_line<'a>(input: &str) -> IResult<&str, (Tag, (Tag, Tag))> {
    map(tuple((
        mapping_tag,
        tag(" = ("),
        mapping_tag,
        tag(", "),
        mapping_tag,
        tag(")")
    )), |(from, _, to1, _, to2, _)| {
        (from, (to1, to2))
    })(input)
}

fn parse(input: &str) -> IResult<&str, Input> {
    map(tuple((
        directions,
        line_ending,
        line_ending,
        fold_many1(terminated(mapping_line, line_ending),
                   HashMap::new,
                   |mut acc, (from, to)| {
                       acc.insert(from, to);
                       acc
                   })
    )), |(directions, _, _, map_nodes)| Input::new(directions, map_nodes))(input)
}

// assumption: The periodicity of the map is without a "lead in" of n < p steps (or formally, n = 0)
// this holds on all inputs I've seen, but needs to be acknowledged

fn part1(input: &Input) -> u64 {
    let start = Tag::from_str("AAA").unwrap();
    let goal = Tag::from_str("ZZZ").unwrap();

    steps_to_goal(input, start, |it| *it == goal)
}

fn steps_to_goal(input: &Input, mut current: Tag, condition: impl Fn(&Tag) -> bool) -> u64 {
    let mut steps = 0;

    while !condition(&current) {
        let direction = input.directions[steps % input.directions.len()];
        let fork = input.map_nodes[&current];

        current = if direction == Direction::Left {
            fork.0
        } else {
            fork.1
        };

        steps += 1;
    }

    steps as u64
}

fn part2(input: &Input) -> u64 {
    let starts = input.map_nodes.keys().filter(|n| n.ends_with(b'A')).cloned();
    let mut overall = 1;

    for current in starts {
        let steps = steps_to_goal(input, current, |it| it.ends_with(b'Z'));

        overall = num::integer::lcm(overall, steps);
    }

    overall
}

nom_solution!(parse, part1, part2);