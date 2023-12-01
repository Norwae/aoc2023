use nom::IResult;


const STRING_VALUE_PAIRS: [(&'static str, i64); 9] = [
    ("one", -1),
    ("two", -2),
    ("three", -3),
    ("four", -4),
    ("five", -5),
    ("six", -6),
    ("seven", -7),
    ("eight", -8),
    ("nine", -9)
];


fn solve_generic<F: Fn(&i64) -> Option<u64>>(input: &Vec<Vec<i64>>, map: F) -> u64 {
    let mut sum = 0;
    for content in input.into_iter() {
        let parts = content.into_iter().filter_map(&map);
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

fn part1(input: &Vec<Vec<i64>>) -> u64 {
    solve_generic(input, |c| {
        let c = *c;
        if c >= 0 {
            Some(c as u64)
        } else {
            None
        }
    })
}

fn part2(input: &Vec<Vec<i64>>) -> u64 {
    solve_generic(input, |c| {
        Some(c.abs() as u64)
    })
}

fn parse(mut input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let mut result = vec![Vec::with_capacity(8)];

    while !input.is_empty() {
        let first = input.as_bytes()[0];
        if first == b'\n' {
            result.push(Vec::with_capacity(8))
        } else if (b'1'..=b'9').contains(&first) {
            result.last_mut().unwrap().push((first - b'0') as i64);
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