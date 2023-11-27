use nom::branch::alt;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::terminated;

pub fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}
pub fn parse_u64_terminated(input: &str) -> IResult<&str, u64> {
    terminated(parse_u64, alt((space1, line_ending)))(input)
}