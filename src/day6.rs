use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1, u64};
use nom::combinator::{map, map_res, peek};
use nom::IResult;
use nom::multi::{fold_many1, many1};
use nom::sequence::{preceded, tuple};

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

#[derive(Debug)]
struct Input {
    part1: Vec<Race>,
    part2: Race
}

fn parse(input: &str) -> IResult<&str, Input> {
    map(
        tuple((
        peek(parse_part_1),
        parse_part_2
    )), |(part1, part2)|{
            Input { part1, part2 }
        })(input)
}

fn parse_part_1(input: &str) -> IResult<&str, Vec<Race>> {
    map(
        tuple((
            tag("Time:"),
            many1(preceded(space1, u64)),
            line_ending,
            tag("Distance:"),
            many1(preceded(space1, u64)
        ))),
        |(_,times, _, _, records)|{
            times.into_iter().zip(records.into_iter()).map(|(time, record)|{
                Race { time, record}
            }).collect()
        })(input)
}

fn number_with_spaces(input: &str) -> IResult<&str, u64> {
    map_res(fold_many1(
        preceded(space1, digit1),
        String::new,
        |mut buffer, part|{
            buffer.push_str(part);
            buffer
        }
    ), |x|x.parse())(input)
}

fn parse_part_2(input: &str) -> IResult<&str, Race> {
    map(
        tuple((
            tag("Time:"),
            number_with_spaces,
            line_ending,
            tag("Distance:"),
            number_with_spaces
            )),
        |(_,time, _, _, record)|{
            Race { time, record }
        })(input)
}

fn solve_race(race: &Race) -> u64 {
    let mut lower = 1;
    let mut upper = race.time / 2;

    while lower + 1 < upper {
        let mid = (lower + upper) / 2;

        if (race.time - mid) * mid <= race.record {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    race.time - 2 * lower - 1
}

fn part1(input: &Input) -> u64 {
    input.part1.iter().map(solve_race).product()
}

fn part2(input: &Input) -> u64 {
    solve_race(&input.part2)

}

solution!(parse, part1, part2);