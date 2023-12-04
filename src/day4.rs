use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::{line_ending, space1, u64 as parse_u64};
use nom::combinator::{flat_map, map};
use nom::IResult;
use nom::multi::{fold_many1, many1, separated_list1};
use nom::sequence::{preceded, tuple};

#[derive(Debug, Default)]
struct Card {
    match_count: u64
}

impl Card {
    fn value(&self) -> u64 {
        let count = self.match_count;
        if count > 0 {
            1 << (count - 1)
        } else {
            0
        }
    }
}

fn u64_with_sep(input: &str) -> IResult<&str, u64> {
    preceded(space1, parse_u64)(input)
}


fn one_card(input: &str) -> IResult<&str, Card> {
    map(flat_map(tuple((
        take_until(": "),
        take(1usize),
        many1(u64_with_sep),
        tag(" |")
    )), |(_, _, my_numbers, _)| {
        fold_many1(u64_with_sep, ||0u64, move |sum: u64,next: u64|{
            if my_numbers.contains(&next) {
                sum + 1
            } else {
                sum
            }
        })
    }), |match_count|Card {match_count})(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, one_card)(input)
}

fn part1(input: &Vec<Card>) -> u64 {
    input.iter().map(|c|c.value()).sum()
}

fn part2(input: &Vec<Card>) -> u64 {
    let mut input = input.iter().map(|card|(card, 1)).collect::<Vec<_>>();

    for i in 0..input.len() {
        let (additional_cards, copies) = input[i];
        let additional_cards = additional_cards.match_count as usize;

        let lower = i + 1;
        let higher = (i + additional_cards).min(input.len() - 1);

        for j in lower..=higher {
            input[j].1 += copies
        }
    }

    input.into_iter().map(|(_, v)| v).sum()
}

solution!(parse, part1, part2);