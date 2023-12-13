use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u32 as parse_u32};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SpringState {
    UNKNOWN,
    DAMAGED,
    OPERATIONAL,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Problem {
    states: Vec<SpringState>,
    broken_series: Vec<u32>,
}

struct SubProblem<'a> {
    states: &'a [SpringState],
    broken_series: &'a [u32],
    hashcode: u64
}

impl PartialEq for SubProblem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.states == other.states && self.broken_series == other.broken_series
    }
}

impl Eq for SubProblem<'_>{}

impl Hash for SubProblem<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hashcode)
    }
}


fn verify_all_compatible(slice: &[SpringState], expected: SpringState) -> bool {
    for state in slice {
        if *state != SpringState::UNKNOWN && *state != expected {
            return false
        }
    }

    return true
}
fn verify_damaged_section(data: &[SpringState], length: usize) -> bool {

    verify_all_compatible(&data[..length], SpringState::DAMAGED) &&
        verify_all_compatible(&data[length..=length], SpringState::OPERATIONAL)
}


impl SubProblem<'_> {
    fn new<'a>(states: &'a [SpringState], broken_series: &'a [u32]) -> SubProblem<'a> {
        let mut hashcode = broken_series.len() as u64;

        for state in states {
            let (h1, _) = hashcode.overflowing_mul(37u64);
            let (h2, _) = h1.overflowing_add(*state as u64);

            hashcode = h2;
        }

        SubProblem { states, broken_series, hashcode }
    }
}

impl <'a> SubProblem<'a> {
    fn arrangements_cached(self: SubProblem<'a>, cache: &mut HashMap<SubProblem<'a>, usize>) -> usize {
        if let Some(cached) = cache.get(&self) {
            return *cached
        }

        let mut sum = 0;

        if let Some(next_length) = self.broken_series.first() {
            let next_length = *next_length as usize;
            let mut slice = &self.states[..];

            while slice.len() >= next_length + 1 {
                match slice[0] {
                    SpringState::DAMAGED if verify_damaged_section(slice, next_length) => {
                        let sub_problem = SubProblem::new(&slice[next_length + 1..], &self.broken_series[1..]);
                        sum += sub_problem.arrangements_cached(cache);
                        break
                    },
                    SpringState::DAMAGED => break,
                    SpringState::UNKNOWN if verify_damaged_section(slice, next_length) => {
                        let sub_problem = SubProblem::new(&slice[next_length + 1..], &self.broken_series[1..]);
                        sum += sub_problem.arrangements_cached(cache);
                    }
                    _ => () // continue loop
                }
                slice = &slice[1..]
            }
        } else {
            sum = if verify_all_compatible(&self.states, SpringState::OPERATIONAL) {
                1
            } else {
                0
            }
        }

        cache.insert(self, sum);
        sum
    }
}

impl Problem {

    fn new(states: Vec<SpringState>, broken_series:Vec<u32>) -> Self {

        Self { states, broken_series }
    }
    fn arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        let mut clone = self.clone();
        if !clone.states.ends_with(&[SpringState::OPERATIONAL]) {
            clone.states.push(SpringState::OPERATIONAL)
        }
        let whole = SubProblem::new(&clone.states, &clone.broken_series);
        let options = whole.arrangements_cached(&mut cache);
        options
    }

    fn unfold(&self) -> Self {
        let mut states = self.states.clone();
        let mut broken_series = self.broken_series.clone();
        for _ in 0..4 {
            states.push(SpringState::UNKNOWN);
            states.extend_from_slice(&self.states);
            broken_series.extend_from_slice(&self.broken_series)
        }

        Self::new(states, broken_series)
    }
}

fn parse_state(input: &str) -> IResult<&str, SpringState> {
    alt((
        value(SpringState::UNKNOWN, tag("?")),
        value(SpringState::DAMAGED, tag("#")),
        value(SpringState::OPERATIONAL, tag("."))
    ))(input)
}

fn parse_series(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), parse_u32)(input)
}

fn parse_problem(input: &str) -> IResult<&str, Problem> {
    map(tuple((
        many1(parse_state),
        space1,
        parse_series
    )), |(states, _, broken_series)| Problem::new(states, broken_series))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
    separated_list1(line_ending, parse_problem)(input)
}

fn part_1(input: &Vec<Problem>) -> usize {
    let mut count = 0usize;
    for problem in input {
        count += problem.arrangements();
    }
    count
}

fn part_2(input: &Vec<Problem>) -> usize {

    let mut count = 0usize;
    for problem in input {
        count += problem.unfold().arrangements();
    }
    count
}


solution!(parse, part_1, part_2);