use std::collections::BTreeMap;

use nom::character::complete::u64;
use nom::IResult;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coord2D(i64, i64);

#[derive(Debug)]
struct GridNumber(u64);

impl GridNumber {
    fn len(&self) -> usize {
        if self.0 >= 100 {
            3
        } else if self.0 >= 10 {
            2
        } else {
            1
        }
    }

    fn next_to(&self, base: &Coord2D, coord: &Coord2D) -> bool {
        let Coord2D(my_x, my_y) = base;
        let Coord2D(c_x, c_y) = *coord;
        c_y >= my_y - 1 &&
            c_y <= my_y + 1 &&
            c_x >= my_x - 1 &&
            c_x <= my_x + self.len() as i64
    }
}

#[derive(Default, Debug)]
struct Input {
    numbers: BTreeMap<Coord2D, GridNumber>,
    gear_locations: Vec<Coord2D>,
    part_locations: Vec<Coord2D>,
}

fn parse_line_into<'a, 'b>(target: &'b mut Input, y: usize, mut line: &'a str) -> IResult<&'a str, ()> {
    let y = y as i64;
    let mut x = 0;
    while !line.is_empty() {
        match line.as_bytes()[0] {
            b'0'..=b'9' => {
                let (rest, value) = u64(line)?;
                let number = GridNumber(value);
                let base = Coord2D(x, y);
                let len = number.len() as i64;
                target.numbers.insert(base, number);

                x += len;
                line = rest;
                continue;
            }
            b'.' => {}
            b'*' => {
                target.gear_locations.push(Coord2D(x, y));
            }
            _ => {
                target.part_locations.push(Coord2D(x, y));
            }
        }

        line = &line[1..];
        x += 1;
    }

    Ok(("", ()))
}

fn part1(input: &Input) -> u64 {
    let mut sum = 0;
    for part in input.part_locations.iter().chain(input.gear_locations.iter()) {
        let potential_nrs = adjecent_to(&input.numbers, *part);

        for (nr_location, nr) in potential_nrs {
            if nr.next_to(nr_location, part) {
                sum += nr.0
            }
        }
    }

    sum
}

fn adjecent_to(map: &BTreeMap<Coord2D, GridNumber>, base: Coord2D) -> impl Iterator<Item=(&Coord2D, &GridNumber)> {
    let Coord2D(x, y) = base;
    map.range(Coord2D(x - 3, y - 1)..=Coord2D(x + 1, y + 1))
}

fn part2(input: &Input) -> u64 {
    let mut sum = 0;
    for gear in &input.gear_locations {
        let potential_nrs = adjecent_to(&input.numbers, *gear);

        let numbers = potential_nrs.filter_map(|(nr_location, nr)| {
            if nr.next_to(&nr_location, gear) {
                Some(nr.0)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1]
        }
    }

    sum
}

fn parse(input: &str) -> IResult<&str, Input> {
    let mut target = Input::default();
    for (n, line) in input.lines().enumerate() {
        parse_line_into(&mut target, n, line)?;
    }

    Ok(("", target))
}

solution!(parse, part1, part2);