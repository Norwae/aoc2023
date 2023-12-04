use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::{line_ending, space1, u64 as parse_u64};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};

#[derive(Debug, Default)]
struct Card {
    my_numbers: Vec<u64>,
    chosen_numbers: Vec<u64>
}

impl Card {

    fn match_count(&self) -> usize {
        let Self{my_numbers, chosen_numbers} = self;

        my_numbers.iter().filter(|n| chosen_numbers.contains(n)).count()
    }

    fn value(&self) -> u64 {
        let count = self.match_count();
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
    map(tuple((
        take_until(": "),
        take(1usize),
        many1(u64_with_sep),
        tag(" |"),
        many1(u64_with_sep)
    )), |(_, _, my_numbers, _, chosen_numbers)| {
        Card { my_numbers, chosen_numbers }
    })(input)
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
        let additional_cards = additional_cards.match_count();

        let lower = i + 1;
        let higher = (i + additional_cards).min(input.len() - 1);

        for j in lower..=higher {
            input[j].1 += copies
        }
    }

    input.into_iter().map(|(_, v)| v).sum()
}

solution!(parse, part1, part2);