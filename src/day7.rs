use std::cmp::Ordering;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u64 as parse_u64};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::day7::ScoreClass::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, Pair, ThreeOfAKind, TwoPair};

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

impl CardType {
    fn part2_special_cmp(&self, other: &CardType) -> Ordering {
        if *self == CardType::Jack {
            if *other == CardType::Jack {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if *other == CardType::Jack {
            Ordering::Greater
        } else {
            self.cmp(other)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [CardType; 5],
    bet: u64,
}


impl Hand {

    fn score_class_part2(&self) -> ScoreClass {
        let mut occurrance = [0; 13];
        for card in self.cards {
            occurrance[card as usize] += 1;
        }
        let mut seen_two = 0;
        let mut seen_three = false;
        let jokers = occurrance[CardType::Jack as usize];
        occurrance[CardType::Jack as usize] = 0;

        for o in occurrance {
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

    fn score_class_part1(&self) -> ScoreClass {
        let mut occurrance = [0; 13];
        for card in self.cards {
            occurrance[card as usize] += 1;
        }
        let mut seen_three = false;
        let mut seen_two = false;

        for o in occurrance {
            match o {
                5 => return FiveOfAKind,
                4 => return FourOfAKind,
                3 => seen_three = true,
                2 if seen_two => return TwoPair,
                2 => seen_two = true,
                _ => {}
            }
        }

        if seen_three {
            if seen_two {
                FullHouse
            } else {
                ThreeOfAKind
            }
        } else if seen_two {
            Pair
        } else {
            HighCard
        }
    }

    fn cmp_generic<Classifier: Fn(&Hand) -> ScoreClass, Comparator: Fn(&CardType, &CardType) -> Ordering>(
        &self, other: &Self, classifier: Classifier, comparator: Comparator
    ) -> Ordering {

        match classifier(self).cmp(&classifier(other)) {
            Ordering::Equal => {
                for i in 0..5 {
                    let card_compare = comparator(&self.cards[i], &other.cards[i]);

                    if card_compare != Ordering::Equal {
                        return card_compare;
                    }
                }

                return Ordering::Equal;
            }
            ord => ord
        }
    }
    fn cmp_part1(&self, other: &Self) -> Ordering {
        self.cmp_generic(other, Self::score_class_part1, CardType::cmp)
    }
    fn cmp_part2(&self, other: &Self) -> Ordering {
        self.cmp_generic(other, Self::score_class_part2, CardType::part2_special_cmp)
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
    input.sort_by(Hand::cmp_part1);
    let mut sum = 0;

    for (rank, hand) in input.iter().enumerate() {
        sum += (rank + 1) as u64 * hand.bet
    }

    sum
}
fn part2(input: &Vec<Hand>) -> u64 {
    let mut input = input.clone();
    input.sort_by(Hand::cmp_part2);
    let mut sum = 0;

    for (rank, hand) in input.iter().enumerate() {
        sum += (rank + 1) as u64 * hand.bet
    }

    sum
}

solution!(parse, part1, part2);