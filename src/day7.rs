use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u64 as parse_u64};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

use crate::day7::ScoreClass::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPair};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Valuation {
    class: ScoreClass,
    fingerprint: u64,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
enum ScoreClass {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum CardType {
    Joker,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    Jack,
    Queen,
    King,
    Ace,
}


#[derive(Debug, Clone)]
struct Hand {
    cards: [CardType; 5],
    bet: u64,
}


impl Hand {
    fn valuation(&self) -> Valuation {
        let class = self.score_class();
        let mut fingerprint = 0;
        for i in 0..5 {
            fingerprint = (fingerprint << 8) + self.cards[i] as u64
        }

        Valuation { class, fingerprint }
    }

    fn score_class(&self) -> ScoreClass {
        let mut occurrance = [0; 14];
        for card in self.cards {
            occurrance[card as usize] += 1;
        }
        let mut seen_two = 0;
        let mut seen_three = false;
        let jokers = occurrance[CardType::Joker as usize];

        for o in 1..14 {
            let o = occurrance[o];
            match o {
                5 => return FiveOfAKind,
                4 if jokers == 1 => return FiveOfAKind,
                4 => return FourOfAKind,
                3 => seen_three = true,
                2 => seen_two += 1,
                _ => {}
            }
        }
        if seen_three {
            match jokers {
                2 => FiveOfAKind,
                1 => FourOfAKind,
                _ if seen_two == 1 => FullHouse,
                _ => ThreeOfAKind
            }
        } else if seen_two > 0 {
            match jokers {
                3 => FiveOfAKind,
                2 => FourOfAKind,
                1 if seen_two == 2 => FullHouse,
                1 => ThreeOfAKind,
                _ if seen_two == 2 => TwoPair,
                _ => Pair
            }
        } else {
            match jokers {
                4.. => FiveOfAKind,
                3 => FourOfAKind,
                2 => ThreeOfAKind,
                1 => Pair,
                _ => HighCard
            }
        }
    }

}


fn parse_card(input: &str) -> IResult<&str, CardType> {
    alt((
        value(CardType::_2, tag("2")),
        value(CardType::_3, tag("3")),
        value(CardType::_4, tag("4")),
        value(CardType::_5, tag("5")),
        value(CardType::_6, tag("6")),
        value(CardType::_7, tag("7")),
        value(CardType::_8, tag("8")),
        value(CardType::_9, tag("9")),
        value(CardType::_10, tag("T")),
        value(CardType::Jack, tag("J")),
        value(CardType::Queen, tag("Q")),
        value(CardType::King, tag("K")),
        value(CardType::Ace, tag("A")),
    ))(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map(tuple(
        (parse_card,
         parse_card,
         parse_card,
         parse_card,
         parse_card,
         space1,
         parse_u64
        )), |(c1, c2, c3, c4, c5, _, bet)|
            Hand { cards: [c1, c2, c3, c4, c5], bet },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}



fn part1(input: &Vec<Hand>) -> u64 {
    let mut input = input.clone();

    input.sort_by_cached_key(|hand|hand.valuation());
    let mut sum = 0;

    for (rank, hand) in input.iter().enumerate() {
        sum += (rank + 1) as u64 * hand.bet
    }

    sum
}

fn part2(input: &Vec<Hand>) -> u64 {
    let mut input = input.iter().map(|hand| {
        let mut hand = hand.clone();
        for i in 0..5 {
            if hand.cards[i] == CardType::Jack {
                hand.cards[i] = CardType::Joker
            }
        }
        hand
    }).collect::<Vec<_>>();
    input.sort_by_cached_key(|hand|hand.valuation());
    let mut sum = 0;

    for (rank, hand) in input.iter().enumerate() {
        sum += (rank + 1) as u64 * hand.bet
    }

    sum
}

solution!(parse, part1, part2);