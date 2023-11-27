use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, multispace1, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::terminated;

pub fn parse_u64(input: &str) -> IResult<&str, u64> {
    terminated(map_res(digit1, str::parse), alt((space1, line_ending)))(input)
}