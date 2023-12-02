use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::{line_ending, space1, u64};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Copy, Clone, Default)]
struct BagState {
    red: u64,
    blue: u64,
    green: u64,
}

impl BagState {
    fn plausible_with(&self, other: &Self) -> bool {
        self.red <= other.red &&
            self.green <= other.green &&
            self.blue <= other.blue
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

fn color(input: &str) -> IResult<&str, Color> {
    alt(
        (
            map(tag("red"), |_| Color::Red),
            map(tag("green"), |_| Color::Green),
            map(tag("blue"), |_| Color::Blue)
        )
    )(input)
}

fn single_round<'a>(state: &'a mut BagState) -> impl FnMut(&str) -> IResult<&str, ()> + 'a { |input|
    map(tuple((u64, space1, color)), |(n,_, c)|{
        let ptr = match c {
            Color::Red => &mut state.red,
            Color::Green => &mut state.green,
            Color::Blue => &mut state.blue
        };
        let val = *ptr;
        let val = val.max(n);

        *ptr = val
    })(input)
}

fn line(input: &str) -> IResult<&str, BagState> {
    let mut merge_bag = BagState::default();


    let (rest, _) = tuple((
        take_until(": "),
        take(2usize),
        separated_list1(alt((tag(", "), tag("; "))), single_round(&mut merge_bag))
    ))(input)?;
    Ok((rest, merge_bag))
}

fn parse(input: &str) -> IResult<&str, Vec<BagState>> {
    separated_list1(line_ending, line)(input)
}

fn part1(input: &Vec<BagState>) -> i32 {
    let limits = BagState {
        red: 12, green: 13, blue: 14
    };

    input.iter().enumerate().fold(0, |sum, (idx, state)| {
        sum + if (state.plausible_with(&limits)) {
            1 + idx as i32
        } else {
            0
        }
    })
}

fn part2(input: &Vec<BagState>) -> u64 {
    input.iter().fold(0, |sum, state|{
        sum + state.power()
    })
}

solution!(parse, part1, part2);