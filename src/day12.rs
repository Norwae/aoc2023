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

#[derive(Debug, Clone)]
struct Problem {
    states: Vec<SpringState>,
    broken_series: Vec<u32>,
}

impl Problem {
    fn series_plausible(&self) -> bool {
        let mut in_series = false;
        let mut series = Vec::new();

        for state in &self.states {
            match state {
                SpringState::UNKNOWN => {
                    let n = series.len();
                    if in_series {
                        return n <= self.broken_series.len() &&
                            series[0..n - 1] == self.broken_series[0..n - 1] &&
                            series[n - 1] <= self.broken_series[n - 1];
                    } else {
                        return n <= self.broken_series.len() &&
                            series == self.broken_series[0..n];
                    }
                }
                SpringState::DAMAGED => {
                    if !in_series {
                        series.push(0);
                    }
                    in_series = true;
                    *series.last_mut().unwrap() += 1
                }
                SpringState::OPERATIONAL => {
                    if in_series {
                        in_series = false;
                        if series.len() > self.broken_series.len() ||
                            series != self.broken_series[0..series.len()] {
                            return false;
                        }
                    }
                }
            }
        }

        series == self.broken_series
    }

    fn arrangements(self, target: &mut usize) -> Self {
        let mut me = self;
        let first_unknown = me.states.iter().position(|s| *s == SpringState::UNKNOWN);
        if let Some(unknown) = first_unknown {
            me.states[unknown] = SpringState::DAMAGED;

            if me.series_plausible() {
                me = me.arrangements(target)
            }

            me.states[unknown] = SpringState::OPERATIONAL;

            if me.series_plausible() {
                me = me.arrangements(target)
            }

            me.states[unknown] = SpringState::UNKNOWN
        } else {
            if me.series_plausible() {
                *target += 1
            }
        }
        me
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
    )), |(states, _, broken_series)| Problem { states, broken_series })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
    separated_list1(line_ending, parse_problem)(input)
}

fn part_1(input: &Vec<Problem>) -> usize {
    let mut count = 0usize;
    for problem in input {
        problem.clone().arrangements(&mut count);
    }
    count
}


solution!(parse, part_1, part_2);