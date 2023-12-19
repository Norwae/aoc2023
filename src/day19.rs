use std::collections::{HashMap, VecDeque};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u32};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::tuple;

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Copy, Clone)]
enum ConditionOp {
    LT,
    GT,
}

#[derive(Debug, Copy, Clone)]
enum ConditionRef {
    X,
    M,
    A,
    S,
}


#[derive(Debug, Clone, Eq, PartialEq)]
enum Decision {
    Accept,
    Reject,
    Forward(String),
}

#[derive(Debug, Clone)]
struct Condition {
    field: ConditionRef,
    op: ConditionOp,
    value: u32,
    destination: Decision,
}

impl Condition {
    fn applies_to(&self, part: &Part) -> bool {
        let field = match self.field {
            ConditionRef::X => &part.x,
            ConditionRef::M => &part.m,
            ConditionRef::A => &part.a,
            ConditionRef::S => &part.s
        };

        match self.op {
            ConditionOp::LT => *field < self.value,
            ConditionOp::GT => *field > self.value,
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    conditionals: Vec<Condition>,
    default: Decision,
}

#[derive(Debug, Clone)]
struct InputRange {
    from_x: u32,
    to_x: u32,
    from_m: u32,
    to_m: u32,
    from_a: u32,
    to_a: u32,
    from_s: u32,
    to_s: u32,
}

impl Default for InputRange {
    fn default() -> Self {
        InputRange {
            from_x: 1,
            to_x: 4001,
            from_m: 1,
            to_m: 4001,
            from_a: 1,
            to_a: 4001,
            from_s: 1,
            to_s: 4001,
        }
    }
}


impl InputRange {
    fn len(&self) -> usize {
        let mut product = 0usize;
        if !self.is_empty() {
            product = (self.to_x - self.from_x) as usize;
            product *= (self.to_m - self.from_m) as usize;
            product *= (self.to_a - self.from_a) as usize;
            product *= (self.to_s - self.from_s) as usize;
        }

        product
    }
    fn is_empty(&self) -> bool {
        self.from_x >= self.to_x || self.from_m >= self.to_m || self.from_a >= self.to_a || self.from_s >= self.to_s
    }

    fn split_for_condition(self, c: &Condition) -> (InputRange, InputRange) {
        let mut _then = self.clone();
        let mut _else = self.clone();

        if let ConditionOp::LT = c.op {
            match c.field {
                ConditionRef::X => {
                    _then.to_x = c.value;
                    _else.from_x = c.value;
                }
                ConditionRef::M => {
                    _then.to_m = c.value;
                    _else.from_m = c.value;
                }
                ConditionRef::A => {
                    _then.to_a = c.value;
                    _else.from_a = c.value;
                }
                ConditionRef::S => {
                    _then.to_s = c.value;
                    _else.from_s = c.value;
                }
            }
        } else {
            let value = c.value + 1;
            match c.field {
                ConditionRef::X => {
                    _then.from_x = value;
                    _else.to_x = value;
                }
                ConditionRef::M => {
                    _then.from_m = value;
                    _else.to_m = value;
                }
                ConditionRef::A => {
                    _then.from_a = value;
                    _else.to_a = value;
                }
                ConditionRef::S => {
                    _then.from_s = value;
                    _else.to_s = value;
                }
            }
        }

        (_then, _else)
    }
}

#[derive(Debug, Clone)]
struct Input {
    ruleset: HashMap<String, Rule>,
    parts: Vec<Part>,
}

impl Rule {
    fn apply(&self, input: &Part) -> &Decision {
        for c in &self.conditionals {
            if c.applies_to(input) {
                return &c.destination;
            }
        }

        &self.default
    }
}

impl Input {
    fn run_to_terminal(&self, input: &Part) -> bool {
        let mut rule = &self.ruleset["in"];

        loop {
            match rule.apply(input) {
                Decision::Accept => return true,
                Decision::Reject => return false,
                Decision::Forward(next) => rule = &self.ruleset[next],
            }
        }
    }

    fn apply_input_range_to<'a>(&'a self, range: InputRange, label: &'a str, mut acceptor: impl FnMut(InputRange)) {
        let mut queue = VecDeque::from([(label, range)]);

        'main: while let Some((label, mut range)) = queue.pop_front() {
            let rule = &self.ruleset[label];
            for condition in &rule.conditionals {
                let (_then, _else) = range.split_for_condition(condition);
                if !_then.is_empty() {
                    match &condition.destination {
                        Decision::Accept => acceptor(_then),
                        Decision::Reject => (),
                        Decision::Forward(label) => {
                            queue.push_back((label, _then))
                        }
                    }
                }

                if _else.is_empty() {
                    continue 'main;
                }
                range = _else
            }


            match &rule.default {
                Decision::Accept => acceptor(range),
                Decision::Reject => (),
                Decision::Forward(label) => {
                    queue.push_back((label, range))
                }
            }
        }
    }
}

fn parse_condition_ref(input: &str) -> IResult<&str, ConditionRef> {
    alt((
        value(ConditionRef::X, tag("x")),
        value(ConditionRef::M, tag("m")),
        value(ConditionRef::A, tag("a")),
        value(ConditionRef::S, tag("s")),
    ))(input)
}

fn parse_condition_op(input: &str) -> IResult<&str, ConditionOp> {
    alt((
        value(ConditionOp::LT, tag("<")),
        value(ConditionOp::GT, tag(">"))
    ))(input)
}

fn parse_decision(input: &str) -> IResult<&str, Decision> {
    alt((
        value(Decision::Accept, tag("A")),
        value(Decision::Reject, tag("R")),
        map(alpha1, |str: &str| Decision::Forward(str.to_owned()))
    ))(input)
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    map(tuple((
        parse_condition_ref,
        parse_condition_op,
        u32,
        tag(":"),
        parse_decision
    )), |(field, op, value, _, destination)| {
        Condition { field, op, value, destination }
    })(input)
}

fn parse_rule_into<'a>(target: &'a mut HashMap<String, Rule>) -> impl FnMut(&str) -> IResult<&str, ()> + 'a {
    |input| {
        map(tuple((
            alpha1,
            tag("{"),
            separated_list0(tag(","), parse_condition),
            tag(","),
            parse_decision,
            tag("}")
        )), |(label, _, conditionals, _, default, _)| {
            let rule = Rule {
                conditionals,
                default,
            };
            target.insert(label.to_string(), rule);
        })(input)
    }
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    map(tuple((
        tag("{x="),
        u32,
        tag(",m="),
        u32,
        tag(",a="),
        u32,
        tag(",s="),
        u32,
        tag("}")
    )), |(_, x, _, m, _, a, _, s, _)| {
        Part { x, m, a, s }
    })(input)
}

fn parse(input: &str) -> IResult<&str, Input> {
    let mut ruleset = HashMap::new();
    let (rest, parsed) = tuple((
        separated_list1(line_ending, parse_rule_into(&mut ruleset)),
        line_ending,
        line_ending,
        separated_list1(line_ending, parse_part)
    ))(input)?;
    let parts = parsed.3;

    Ok((rest, Input { ruleset, parts }))
}

fn part1(input: &Input) -> u32 {
    let mut sum = 0;
    for part in &input.parts {
        if input.run_to_terminal(&part) {
            sum += part.x + part.m + part.a + part.s;
        }
    }

    sum
}

fn part2(input: &Input) -> usize {
    let mut sum = 0;
    input.apply_input_range_to(InputRange::default(), "in", |r| {
        sum += r.len();
    });

    sum
}

nom_solution!(parse, part1, part2);