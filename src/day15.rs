use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u32 as parse_u32};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{terminated, tuple};

#[derive(Debug, Clone)]
struct SlotEntry<'a>(&'a str, u32);

#[derive(Debug, Default)]
struct TrivialHashMap<'a> {
    slots: Vec<Vec<SlotEntry<'a>>>
}

impl <'a>  TrivialHashMap<'a> {
    fn new() -> Self {
        let slots = vec![Vec::new(); 256];

        Self { slots }
    }
}

impl <'a> TrivialHashMap<'a> {
    fn checksum(&self) -> usize {
        self.slots.iter().enumerate().flat_map(|(slot, array)|{
            array.iter().enumerate().map(move |(position, SlotEntry(_, value))| {
                (slot + 1) * (position + 1) * (*value as usize)
            })
        }).sum()
    }
}

fn no_parse(input: &str) -> IResult<&str, String> {
    Ok(("", input.to_string()))
}

fn hash(fragment: &str) -> u8{
    let mut hash = 0u8;
    for ascii in fragment.bytes() {
        if ascii == b'\n' || ascii == b'\r' {
            continue
        }

        hash = hash.overflowing_add(ascii).0;
        hash = hash.overflowing_mul(17).0;
    }

    hash
}

fn part1(input: &String) -> u64 {
    let mut sum = 0;

    for str in input.split(",") {
        sum += hash(str) as u64
    }

    sum
}

enum Operation<'a> {
    Replace(&'a str, u32),
    Remove(&'a str)
}

fn parse_operation<'a>(input: &'a str) -> IResult<&str, Operation<'a>> {
    let replace = map(tuple((
        alpha1,
        tag("="),
        parse_u32
    )), |(name, _, value)|Operation::Replace(name, value));
    let remove = map(terminated(alpha1, tag("-")), |str| Operation::Remove(str));
    alt((replace, remove))(input)
}

fn part2(input: &String) -> usize {
    let mut map = TrivialHashMap::new();
    for op in input.split(",") {
        let (_, op) = parse_operation(op).expect("valid op");
        match op {
            Operation::Replace(label, value) => {
                let hash = hash(label) as usize;
                let slot = &mut map.slots[hash];
                let prev = slot.iter_mut().find(|e|e.0 == label);
                let slot_entry = SlotEntry(label, value);
                if let Some(prev) = prev {
                    *prev = slot_entry;
                } else {
                    slot.push(slot_entry)
                }

            }
            Operation::Remove(label) => {
                let hash = hash(label) as usize;
                let slot = &mut map.slots[hash];
                slot.retain(|e|e.0 != label)
            }
        }
    }

    map.checksum()
}

solution!(no_parse, part1, part2);