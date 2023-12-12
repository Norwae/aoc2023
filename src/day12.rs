use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u32 as parse_u32};
use nom::combinator::{map, value};
use nom::error::dbg_dmp;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use crate::day12::SpringState::{DAMAGED, OPERATIONAL, UNKNOWN};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SpringState {
    UNKNOWN,
    DAMAGED,
    OPERATIONAL,
}

impl SpringState {
    fn can_be_operational(self) -> bool {
        self == OPERATIONAL || self == UNKNOWN
    }
}

#[derive(Debug, Clone)]
struct Problem {
    states: Vec<SpringState>,
    broken_series: Vec<usize>,
    broken_required: usize,
}

struct Mask {
    gaps: Vec<usize>,
    width: usize,
}


fn generate_gaps(to_distribute: usize, gaps: &mut Vec<usize>, offset: usize, target: &mut Vec<Vec<usize>>) {
    for assigned in 0..=to_distribute {
        gaps[offset] = 1 + assigned;

        if offset < gaps.len() - 1 {
            generate_gaps(to_distribute - assigned, gaps, offset + 1, target)
        } else {
            target.push(gaps.clone())
        }
    }
}

impl Problem {
    fn mask_matches(&self, mask: &Mask) -> usize {
        for offset in 0..=(self.states.len() - mask.width) {

        }
        1
    }

    fn generate_masks(&self) -> Vec<Mask> {
        let total_length = self.states.len();
        let mut gaps = vec![1usize; self.broken_series.len() - 1];
        let to_allocate = total_length - gaps.len() - self.broken_series.iter().sum::<usize>();
        let mut gap_receiver = Vec::new();

        generate_gaps(to_allocate, &mut gaps, 0, &mut gap_receiver);
        let mut masks = Vec::new();

        for gaps in gap_receiver {
            let gaps = gaps.clone();
            let width = gaps.iter().sum();
            masks.push(Mask { gaps, width });
        }

        masks
    }

    fn possible_variants(&self) -> usize {
        let mut masks = self.generate_masks();

        masks.into_iter().map(|mask| {
            self.mask_matches(&mask)
        }).sum()
    }
}

fn parse_state(input: &str) -> IResult<&str, SpringState> {
    alt((
        value(UNKNOWN, tag("?")),
        value(DAMAGED, tag("#")),
        value(OPERATIONAL, tag("."))
    ))(input)
}

fn parse_series(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), map(parse_u32, |u| u as usize))(input)
}

fn parse_problem(input: &str) -> IResult<&str, Problem> {
    map(tuple((
        many1(parse_state),
        space1,
        parse_series
    )), |(states, _, broken_series)| {
        let broken_required = broken_series.iter().sum();
        Problem { states, broken_series, broken_required }
    })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
    separated_list1(line_ending, parse_problem)(input)
}


fn part_1(input: &Vec<Problem>) -> usize {
    input[0].possible_variants()
}


solution!(parse, part_1);