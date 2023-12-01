use nom::branch::alt;
use nom::character::complete::{line_ending, space1, u64 as parse_u64};
use nom::IResult;
use nom::sequence::terminated;

pub fn parse_u64_terminated(input: &str) -> IResult<&str, u64> {
    terminated(parse_u64, alt((space1, line_ending)))(input)
}