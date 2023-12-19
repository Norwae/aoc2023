extern crate core;

use std::fs;
use std::fmt::Display;
use std::time::{Duration, Instant};
use nom::IResult;

#[derive(Debug, Default)]
pub struct Context {
    total_duration: Duration,
    non_parse_duration: Duration,
    longest: Option<Duration>,
}


#[allow(dead_code)]
fn part_2_absent<T>(_: &T) -> &'static str {
    "???"
}

#[allow(dead_code)]
fn not_solved<T: std::fmt::Debug>(input: &T) -> String { format!("Parse result: {:?}", input) }

fn filename_for_module(module: &str) -> &str {
    let cutoff = module.rfind("::").map(|found| found + 2).unwrap_or(0);
    &module[cutoff..]
}

macro_rules! simple_solution {
    ($parse:path) => {
        simple_solution!($parse, crate::not_solved);
    };
    ($parse:path, $( $parts:path),*) => {
        solution!({|input|Some($parse(&input)) }, $($parts),*);
    };
}

macro_rules! nom_solution {
    ($parse:path) => {
        nom_solution!($parse, crate::not_solved);
    };
    ($parse:path, $( $parts:path),*) => {
        solution!({|input|crate::nom_parse(input, $parse)}, $($parts),*);
    }
}


macro_rules! unparsed_solution {
    ($( $parts:path),*) => {
        solution!({|input|Some(input)}, $($parts),*);
    }
}

macro_rules! solution {
    () => {
        pub fn solve(_: &mut crate::Context) {}
    };
    ($parse:tt) => {
        solution!($parse, crate::not_solved);
    };
    ($parse:tt, $solution:path) => {
        solution!($parse, $solution, crate::part_2_absent);
    };
    ($parse:tt, $part1:path, $part2:path) => {
        pub fn solve(ctx: &mut crate::Context) {
            let path = module_path!();
            crate::solve(ctx, crate::filename_for_module(&path), $parse, $part1, $part2);
        }
    };
}

fn nom_parse<
    Parsed,
    Parser: FnMut(&str) -> IResult<&str, Parsed>
>(input: String, mut p: Parser) -> Option<Parsed> {
    let result = p(&input);
    return match result {
        Ok((rest, parsed)) => {
            if !rest.trim().is_empty() {
                eprintln!("Dangling input: '{}', ignoring", rest)
            }
            Some(parsed)
        }
        Err(error) => {
            eprintln!("Could not parse input: {}", error);
            None
        }
    };
}


fn solve<
    Intermediate,
    Result1: Display,
    Result2: Display,
    Parse: FnOnce(String) -> Option<Intermediate>,
    Part1: FnOnce(&Intermediate) -> Result1,
    Part2: FnOnce(&Intermediate) -> Result2>(
    context: &mut Context,
    filename: &str,
    parse: Parse,
    solve_part_1: Part1,
    solve_part_2: Part2,
) {
    let path = format!("inputfiles/{}", filename);
    let contents = fs::read_to_string(&path);
    let start = Instant::now();

    if let Ok(contents) = contents {
        if let Some(parsed) = parse(contents) {
            let after_parse = Instant::now();
            let solution_part1 = solve_part_1(&parsed);
            let after_p1 = Instant::now();
            let solution_part2 = solve_part_2(&parsed);
            let after_p2 = Instant::now();

            println!("Solved {:5} - part1: {:16}, part2: {:16} --- ({:10?} parse, {:10?} part 1, {:10?} part 2)",
                     filename,
                     solution_part1,
                     solution_part2,
                     after_parse - start,
                     after_p1 - after_parse,
                     after_p2 - after_p1
            );
            let this_task = after_p2 - start;

            context.longest = Some(context.longest.unwrap_or(this_task).max(this_task));
            context.total_duration += this_task;
            context.non_parse_duration += after_p2 - after_parse;
        }
    } else {
        eprintln!("Could not read input {}, due to {}", path, contents.err().unwrap())
    }
}

mod util;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;


fn main() {
    let day_pointers = [
        day1::solve, day2::solve, day3::solve, day4::solve, day5::solve,
        day6::solve, day7::solve, day8::solve, day9::solve, day10::solve,
        day11::solve, day12::solve, day13::solve, day14::solve, day15::solve,
        day16::solve, day17::solve, day18::solve, day19::solve, day20::solve,
        day21::solve, day22::solve, day23::solve, day24::solve, day25::solve,
    ];

    let mut context = Context::default();
    for ptr in day_pointers {
        ptr(&mut context)
    }
    println!("AoC so far, excluding io: {:?} total, {:?} without overhead. Longest day runtime was {:?} ", context.total_duration, context.non_parse_duration, context.longest.unwrap())
}
