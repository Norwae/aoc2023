use nom::character::complete::{i64, line_ending, space1};
use nom::IResult;
use nom::multi::separated_list1;

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, separated_list1(space1, i64))(input)
}

fn extrapolate(input: &Vec<i64>) -> i64 {
    if input.iter().all(|it|*it == 0) {
        0
    } else {
        let mut differences = Vec::with_capacity(input.len() - 1);
        for i in 0..input.len() - 1 {
            differences.push(input[i + 1] - input[i]);
        }

        let last = input.last().expect("non-empty list");

        last + extrapolate(&differences)
    }
}

fn extrapolate_back(input: &Vec<i64>) -> i64 {

    if input.iter().all(|it|*it == 0) {
        0
    } else {
        let mut differences = Vec::with_capacity(input.len() - 1);
        for i in 0..input.len() - 1 {
            differences.push(input[i + 1] - input[i]);
        }

        let first = input.first().expect("non-empty list");

        first - extrapolate_back(&differences)
    }
}

fn solve_part_1(input: &Vec<Vec<i64>>) -> i64{
    input.iter().map(extrapolate).sum()
}

fn solve_part_2(input: &Vec<Vec<i64>>) -> i64{
    input.iter().map(extrapolate_back).sum()
}


solution!(parse, solve_part_1, solve_part_2);