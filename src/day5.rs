use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::{line_ending, space1, i64};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};

#[derive(Debug)]
struct RangeMapping {
    source: i64,
    target: i64,
    length: i64,
}

impl RangeMapping {
    fn apply(&self, input: i64) -> Option<i64> {
        let offset = input - self.source;
        if (0..self.length).contains(&offset) {
            Some(self.target + offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct MappingTable {
    ranges: Vec<RangeMapping>,
}

impl MappingTable {
    fn apply(&self, input: i64) -> i64 {
        self.ranges.iter().flat_map(|it| it.apply(input).into_iter()).next().unwrap_or(input)
    }
}

#[derive(Debug)]
struct Input {
    ranges: Vec<MappingTable>,
    seeds: Vec<i64>,
}

fn range(input: &str) -> IResult<&str, RangeMapping> {
    map(tuple((
        i64, space1, i64, space1, i64
    )), |(target, _, source, _, length)| RangeMapping { target, source, length })(input)
}

fn mapping_table(input: &str) -> IResult<&str, MappingTable> {
    map(tuple((
        take_until(":"),
        take(1usize),
        line_ending,
        separated_list1(line_ending, range)
    )), |(_, _, _, ranges)| MappingTable { ranges })(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
    map(tuple((
        tag("seeds:"),
        many1(preceded(space1, i64))
    )), |(_, v)| v)(input)
}

fn parse(input: &str) -> IResult<&str, Input> {
    map(tuple((
        seeds,
        line_ending,
        line_ending,
        separated_list1(line_ending, mapping_table))), |(seeds, _, _, ranges)| Input { seeds, ranges })(input)
}

fn part1(input: &Input) -> i64 {
    input.seeds.iter().map(|value| {
        let mut value = *value;
        for table in &input.ranges {
            value = table.apply(value)
        }
        value
    }).min().expect("At least one seed")
}

fn part2(input: &Input) -> i64 {
    let mut seed_ranges = Vec::new();
    for i in (0..input.seeds.len()).step_by(2) {
        let start = input.seeds[i];
        let end = start + input.seeds[i + 1];
        seed_ranges.push(start..end)
    }

    seed_ranges.into_iter()
        .flat_map(|it|it)
        .map(|value| {
            let mut value = value;
            for table in &input.ranges {
                value = table.apply(value)
            }
            value
        }).min().expect("At least one seed")
}

solution!(parse, part1, part2);