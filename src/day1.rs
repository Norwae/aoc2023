use nom::IResult;

#[derive(Debug, Copy, Clone)]
enum Contents {
    Plain(u64),
    Spelled(u64),
}

const STRING_VALUE_PAIRS: [(&'static str, Contents); 19] = [
    ("0", Contents::Plain(0)),
    ("1", Contents::Plain(1)),
    ("2", Contents::Plain(2)),
    ("3", Contents::Plain(3)),
    ("4", Contents::Plain(4)),
    ("5", Contents::Plain(5)),
    ("6", Contents::Plain(6)),
    ("7", Contents::Plain(7)),
    ("8", Contents::Plain(8)),
    ("9", Contents::Plain(9)),
    ("one", Contents::Spelled(1)),
    ("two", Contents::Spelled(2)),
    ("three", Contents::Spelled(3)),
    ("four", Contents::Spelled(4)),
    ("five", Contents::Spelled(5)),
    ("six", Contents::Spelled(6)),
    ("seven", Contents::Spelled(7)),
    ("eight", Contents::Spelled(8)),
    ("nine", Contents::Spelled(9))
];


fn solve_generic<F: Fn(&Contents) -> Option<u64>>(input: &Vec<Vec<Contents>>, map: F) -> u64 {
    let mut sum = 0;
    for content in input.into_iter() {
        let parts = content.iter().filter_map(&map);
        let mut first = u64::MAX;
        let mut last = 0;

        for p in parts {
            if first == u64::MAX {
                first = p
            }
            last = p
        }

        sum += first * 10 + last
    }

    sum
}

fn part1(input: &Vec<Vec<Contents>>) -> u64 {
    solve_generic(input, |c| {
        match c {
            Contents::Plain(v) => Some(*v),
            Contents::Spelled(_) => None
        }
    })
}

fn part2(input: &Vec<Vec<Contents>>) -> u64 {
    solve_generic(input, |c| {
        match c {
            Contents::Plain(v) => Some(*v),
            Contents::Spelled(v) => Some(*v)
        }
    })
}

fn parse(mut input: &str) -> IResult<&str, Vec<Vec<Contents>>> {
    let mut result = vec![Vec::new()];


    while !input.is_empty() {
        if input.as_bytes()[0] == b'\n' {
            result.push(Vec::new())
        } else {
            for (prefix, value) in &STRING_VALUE_PAIRS {
                if input.starts_with(prefix) {
                    result.last_mut().unwrap().push(*value)
                }
            }
        }
        input = &input[1..]
    }


    Ok(("", result))
}


solution!(parse, part1, part2);