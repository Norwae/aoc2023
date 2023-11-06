use std::fmt::Display;
use std::fs;
use std::time::Instant;

fn solve<
    Intermediate,
    Result1 : Display,
    Result2 : Display,
    Parse: FnOnce(String) -> Intermediate,
    Part1: FnOnce(&Intermediate) -> Result1,
    Part2: FnOnce(&Intermediate) -> Result2>(
    filename: &str,
    parse: Parse,
    solve_part_1: Part1,
    solve_part_2: Part2
){
    let path = format!("inputfiles/{}", filename);
    let contents = fs::read(path).expect("Input file readable");
    let input = String::from_utf8(contents).expect("valid utf8");
    let prepared = parse(input);

    let start = Instant::now();
    let solution_part1 = solve_part_1(&prepared);
    let after_p1 = Instant::now();
    let solution_part2 = solve_part_2(&prepared);
    let after_p2 = Instant::now();

    println!("Solved {}, part1: {}, part2: {} ({:?} part 1, {:?} part 2)",
        filename,
        solution_part1,
        solution_part2,
        after_p1 - start,
        after_p2 - after_p1
    )
}

fn unparsed(str: String) -> String {
    str
}

fn unsolved<T>(_input: &T) -> &'static str {
    "unsolved"
}


mod day1;


fn main() {
    let day_pointers = vec![
        day1::solve
    ];

    let start = Instant::now();
    for ptr in day_pointers {
        ptr()
    }
    println!("AoC so far, including io: {:?}", Instant::now() - start)
}
