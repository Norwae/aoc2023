use std::cmp::Ordering;
use std::ops::Range;

use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::{i64, line_ending, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};

#[derive(Debug, PartialEq, Eq)]
struct RangeMapping {
    range: Range<i64>,
    offset: i64,
}

impl PartialOrd for RangeMapping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeMapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

fn search_for_container(value: i64) -> impl Fn(&RangeMapping) -> Ordering {
    move |range|
        if range.range.contains(&value) {
            Ordering::Equal
        } else if range.range.start < value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
}

impl RangeMapping {
    fn apply(&self, input: i64) -> Option<i64> {
        if self.range.contains(&input) {
            Some(input + self.offset)
        } else {
            None
        }
    }

    fn merge_into(mut self, others: &MappingTable, target_mappings: &mut Vec<RangeMapping>) {
        while !self.range.is_empty() {
            let target_range = (self.range.start + self.offset)..(self.range.end + self.offset);
            let intersecting = others.ranges.binary_search_by(search_for_container(target_range.start));

            let (consumed, offset) = match intersecting {
                Ok(found) => {
                    let matching = &others.ranges[found];
                    let match_end = target_range.end.min(matching.range.end);
                    let consumed = match_end - target_range.start;

                    (consumed, matching.offset)
                }
                Err(nearest_after) => {
                    let consumed = if nearest_after == others.ranges.len() {
                        let remaining = target_range.end - target_range.start;

                        remaining
                    } else {
                        let after = &others.ranges[nearest_after];
                        let defaulting_end = after.range.start;
                        let match_end = defaulting_end.min(target_range.end);
                        let consumed = match_end - target_range.start;

                        consumed
                    };

                    (consumed, 0)
                }
            };

            let mapping = RangeMapping {
                range: self.range.start..(self.range.start + consumed),
                offset: offset + self.offset,
            };
            target_mappings.push(mapping);

            self.range.start += consumed;
        }
    }
}

#[derive(Debug)]
struct MappingTable {
    ranges: Vec<RangeMapping>,
}

impl MappingTable {
    fn apply(&self, input: i64) -> i64 {
        if let Ok(mapped) = self.ranges.binary_search_by(search_for_container(input)) {
            self.ranges[mapped].apply(input).expect("Already verified")
        } else {
            input
        }
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
    )), |(target, _, source, _, length)| RangeMapping { range: source..(source + length), offset: target - source })(input)
}

fn mapping_table(input: &str) -> IResult<&str, MappingTable> {
    map(tuple((
        take_until(":"),
        take(1usize),
        line_ending,
        separated_list1(line_ending, range)
    )), |(_, _, _, mut ranges)| {
        ranges.sort();
        MappingTable { ranges }
    })(input)
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
    let mut input_ranges = Vec::with_capacity(input.seeds.len() / 2);
    for i in (0..input.seeds.len()).step_by(2) {
        input_ranges.push(RangeMapping {
            range: input.seeds[i]..(input.seeds[i] + input.seeds[i + 1]),
            offset: 0,
        });
    }

    for table in &input.ranges {
        let mut buffer = Vec::new();

        for range in input_ranges {
            range.merge_into(table, &mut buffer);
        }

        input_ranges = buffer;
    }

    input_ranges.into_iter().map(|r|{
        r.range.start + r.offset
    }).min().expect("At least one")
}

nom_solution!(parse, part1, part2);