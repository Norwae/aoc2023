#![feature(never_type)]

use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Context {
    total_duration: Duration,
    non_parse_duration: Duration
}

trait OutputAndRest {
    type Output : ?Sized;

    fn output(&self) -> &Self::Output;
    fn rest(&self) -> &str;
}

impl OutputAndRest for String {
    type Output = str;
    fn output(&self) -> &Self::Output {
        &self
    }

    fn rest(&self) -> &str {
        ""
    }
}

impl <T> OutputAndRest for (&str, T) {
    type Output = T;

    fn output(&self) -> &Self::Output {
        &self.1
    }

    fn rest(&self) -> &str {
        &self.0
    }
}

fn solve<
    Intermediate: ?Sized,
    Err : Error,
    ParseResult: OutputAndRest<Output=Intermediate>,
    Result1 : Display,
    Result2 : Display,
    Parse: FnOnce(String) -> Result<ParseResult, Err>,
    Part1: FnOnce(&Intermediate) -> Result1,
    Part2: FnOnce(&Intermediate) -> Result2>(
    context: &mut Context,
    filename: &str,
    parse: Parse,
    solve_part_1: Part1,
    solve_part_2: Part2
){
    let start = Instant::now();
    let path = format!("inputfiles/{}", filename);
    let contents = fs::read(&path);

    if let Ok(contents) = contents {
        let input = String::from_utf8(contents).expect("valid utf8");
        let prepared = parse(input);

        if let Ok(parsed) = prepared {
            if !parsed.rest().is_empty() {
                eprintln!("Input was not completely parsed, rest is '{}'", parsed.rest())
            }

            let after_parse = Instant::now();
            let solution_part1 = solve_part_1(parsed.output());
            let after_p1 = Instant::now();
            let solution_part2 = solve_part_2(parsed.output());
            let after_p2 = Instant::now();

            println!("Solved {}, part1: {}, part2: {} ({:?} part 1, {:?} part 2)",
                     filename,
                     solution_part1,
                     solution_part2,
                     after_p1 - after_parse,
                     after_p2 - after_p1
            );
            context.total_duration += (after_p2 - start);
            context.non_parse_duration += (after_p2 - after_parse)
        } else  {
            eprintln!("Could not parse input: {}", prepared.err().unwrap())
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
