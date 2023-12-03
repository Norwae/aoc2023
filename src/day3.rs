use nom::character::complete::u64;
use nom::IResult;

#[derive(Debug, Copy, Clone)]
struct Coord2D(i64, i64);

#[derive(Debug)]
struct GridNumber {
    base: Coord2D,
    value: u64,
    len: i64,
}

impl GridNumber {
    fn next_to(&self, coord: &Coord2D) -> bool {
        let Coord2D(my_x, my_y) = self.base;
        let Coord2D(c_x, c_y) = *coord;
        c_y >= my_y - 1 &&
            c_y <= my_y + 1 &&
            c_x >= my_x - 1 &&
            c_x <= my_x + self.len
    }
}

#[derive(Default, Debug)]
struct Input {
    numbers: Vec<GridNumber>,
    part_locations: Vec<Coord2D>,
}

fn parse_line_into<'a, 'b>(target: &'b mut Input, y: usize, mut line: &'a str) -> IResult<&'a str, ()> {
    let y = y as i64;
    let mut x = 0;
    while !line.is_empty() {
        match line.as_bytes()[0] {
            b'0'..=b'9' => {
                let (rest, value) = u64(line)?;
                let base = Coord2D(x, y);
                let len = (line.len() - rest.len()) as i64;
                target.numbers.push(GridNumber {
                    base,
                    value,
                    len,
                });

                x += len;
                line = rest;
                continue;
            }
            b'.' => {}
            _ => {
                target.part_locations.push(Coord2D(x, y))
            }
        }

        line = &line[1..];
        x += 1;
    }

    Ok(("", ()))
}

fn part1(input: &Input) -> u64 {
    let mut sum = 0;
    for nr in &input.numbers {
        if input.part_locations.iter().any(|part| {
            nr.next_to(part)
        }) {
            sum += nr.value
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

solution!(parse, part1);