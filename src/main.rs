#![feature(never_type)]

use std::error::Error;
use std::fs;
use std::fmt::Display;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Context {
    total_duration: Duration,
    non_parse_duration: Duration,
}

trait OutputAndRest<'a, A> {
    fn rest(&self) -> &'a str;
    fn output(self) -> A;
}

impl <'a, T> OutputAndRest<'a, T> for (&'a str, T) {
    fn rest(&self) -> &'a str {
        self.0
    }

    fn output(self) -> T {
        self.1
    }
}

struct SimpleError(String);

impl<E: Error> From<E> for SimpleError {
    fn from(value: E) -> Self {
        SimpleError(value.to_string())
    }
}
macro_rules! solution {
    () => {
        solution!(crate::unparsed);
    };
    ($parse:path) => {
        solution!($parse, crate::unsolved);
    };
    ($parse:path, $solution:path) => {
        solution!($parse, $solution, crate::no_part_2);
    };
    ($parse:path, $part1:path, $part2:path) => {
        pub fn solve(ctx: &mut crate::Context) {
            let path = module_path!();
            let cutoff = path.rfind("::").unwrap();
            let filename = &path[cutoff + 2..];
            crate::solve(ctx, filename, $parse, $part1, $part2);
        }
    };
}

fn parse_report_errors<
    'a,
    Parsed,
    RestInfo: OutputAndRest<'a, Parsed>,
    Parser: FnOnce(&str) -> Result<RestInfo, SimpleError>
>(input: &'a str, p: Parser) -> Option<Parsed> {
    let result = p(input);
    return match result {
        Ok(rest_info) => {;
            if !rest_info.rest().is_empty() {
                eprintln!("Dangling input: '{}', ignoring", &rest_info.rest())
            }
            Some(rest_info.output())
        }
        Err(error) => {
            eprintln!("Could not parse input: {}", error.0);
            None
        }
    };
}

fn solve<
    Intermediate,
    ParseResult: for<'a> OutputAndRest<'a, Intermediate>,
    Result1: Display,
    Result2: Display,
    Parse: for<'a> FnOnce(&'a str) -> Result<ParseResult, SimpleError>,
    Part1: FnOnce(&Intermediate) -> Result1,
    Part2: FnOnce(&Intermediate) -> Result2>(
    context: &mut Context,
    filename: &str,
    parse: Parse,
    solve_part_1: Part1,
    solve_part_2: Part2,
) {
    let start = Instant::now();
    let path = format!("inputfiles/{}", filename);
    let contents = fs::read(&path);

    if let Ok(contents) = contents {
        let input = String::from_utf8(contents).expect("valid utf8");

        if let Some(parsed) = parse_report_errors(&input, parse) {
            let after_parse = Instant::now();
            let solution_part1 = solve_part_1(&parsed);
            let after_p1 = Instant::now();
            let solution_part2 = solve_part_2(&parsed);
            let after_p2 = Instant::now();

            println!("Solved {}, part1: {}, part2: {} ({:?} part 1, {:?} part 2)",
                     filename,
                     solution_part1,
                     solution_part2,
                     after_p1 - after_parse,
                     after_p2 - after_p1
            );
            context.total_duration += after_p2 - start;
            context.non_parse_duration += after_p2 - after_parse;
        }
    } else {
        eprintln!("Could not read input {}, due to {}", path, contents.err().unwrap())
    }
}

fn unparsed(str: String) -> Result<String, !> {
    Ok(str)
}

fn unsolved<T: ?Sized>(_input: &T) -> &'static str {
    "unsolved"
}

fn no_part_2<T: ?Sized>(_input: &T) -> &'static str {
    "No part 2"
}


mod day1;


fn main() {
    let day_pointers = vec![
        day1::solve
    ];

    let mut context = Context::default();
    for ptr in day_pointers {
        ptr(&mut context)
    }
    println!("AoC so far, including io: {:?} total, {:?} without overhead", context.total_duration, context.non_parse_duration)
}
