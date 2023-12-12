use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u32 as parse_u32};
use nom::combinator::{map, value};
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

fn match_mask_with_partial(state: &[SpringState], gaps: &Vec<usize>, damaged: &Vec<usize>, assigned_count: usize) -> bool {
    let mut gaps = gaps.clone();
    let mut offset = 0;
    gaps.push(state.len() - assigned_count);

    for (damaged, gap) in damaged.iter().zip(gaps.iter()) {
        let damaged = *damaged;
        let gap = *gap;

        for _ in 0..damaged {
            if state[offset] == OPERATIONAL {
                return false
            }
            offset += 1
        }

        for _ in 0..gap {
            if state[offset] == DAMAGED {
                return false
            }
            offset += 1
        }
    }

    true
}

impl Problem {
    fn mask_matches(&self, mask: &Mask) -> usize {
        let assigned_count = mask.width + self.broken_required;
        let mut sum = 0;
        for offset in 0..=(self.states.len() - assigned_count) {
            match self.states[offset] {
                OPERATIONAL => (), // not a match here, so we can advance right now
                DAMAGED => { // we anchor here, can go no further, so break the loop after
                    if match_mask_with_partial(&self.states[offset..], &mask.gaps, &self.broken_series, assigned_count) {
                        sum += 1
                    }
                    break;
                }
                UNKNOWN => { // we can start here... but we don't need to anchor
                    if match_mask_with_partial(&self.states[offset..], &mask.gaps, &self.broken_series, assigned_count) {
                        sum += 1
                    }
                }
            }
        }
        sum
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
        println!("{}", masks.len());

        masks.into_iter().map(|mask| {
            self.mask_matches(&mask)
        }).sum()
    }

    fn unfold(&self) -> Self {
        let mut states = self.states.clone();
        let mut broken_series = self.broken_series.clone();
        let mut extend_states = Vec::new();
        extend_states.push(UNKNOWN);
        extend_states.extend_from_slice(states.as_slice());

        for _ in 0..4 {
            states.extend_from_slice(extend_states.as_slice());
            broken_series.extend_from_slice(self.broken_series.as_slice())
        }
        let broken_required = self.broken_required * 5;

        Self {
            states, broken_series, broken_required
        }
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
    input.iter().map(Problem::possible_variants).sum()
}

fn part_2(input: &Vec<Problem>) -> usize {
    input.iter().map(Problem::unfold).map(|it|it.possible_variants()).sum()
}

solution!(parse, part_1, part_2);