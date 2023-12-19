use std::collections::HashMap;
use std::hash::Hash;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u32};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::tuple;
use crate::day19::Decision::Forward;

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


#[derive(Debug, Clone)]
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
            ConditionOp::GT => *field > self.value
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    conditionals: Vec<Condition>,
    default: Decision,
}

#[derive(Debug, Clone)]
struct Input {
    ruleset: HashMap<String, Rule>,
    parts: Vec<Part>
}

impl Input {
    fn apply_rule<'a, 'input>(&'a self, label: &'a str, input: &'input Part) -> &'a Decision {
        let rule = &self.ruleset[label];

        for c in &rule.conditionals {
            if c.applies_to(input) {
                return &c.destination
            }
        }

        &rule.default
    }

    fn run_to_terminal<'a>(&'a self, input: &Part) -> bool {
        let mut rule_label: &'a str = "in";

        loop {
            match self.apply_rule(rule_label, input) {
                Decision::Accept => return true,
                Decision::Reject => return false,
                Forward(next) => rule_label = next,
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
        map(alpha1, |str: &str| Forward(str.to_owned()))
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
            target.insert(label.to_string(), Rule {
                conditionals,
                default,
            });
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

nom_solution!(parse, part1);