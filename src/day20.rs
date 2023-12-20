use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::day20::Module::{Conjunction, FlipFlop};

#[derive(Debug, Clone)]
enum Module {
    Broadcaster,
    FlipFlop {
        memory: bool
    },
    Conjunction {
        latest_inputs: Vec<bool>,
        input_received: Vec<bool>,
    },
}

#[derive(Debug)]
struct PlannedWiring<'a> {
    label: &'a str,
    module: Module,
    outputs: Vec<&'a str>,
}

#[derive(Debug)]
struct WiredModule {
    module: Module,
    wires: Vec<Wire>,
}

#[derive(Debug)]
struct Wire {
    to: usize,
    inbound_index: usize,
}

#[derive(Debug)]
struct Input {
    broadcaster_index: usize,
    modules: Vec<WiredModule>,
}

fn parse_broadcaster(input: &str) -> IResult<&str, (Module, &str)> {
    value((Module::Broadcaster, "broadcaster"), tag("broadcaster"))(input)
}

fn parse_flip_flop(input: &str) -> IResult<&str, (Module, &str)> {
    map(tuple((
        tag("%"),
        alpha1
    )), |(_, label)| (FlipFlop { memory: false }, label))(input)
}

fn parse_conjunction(input: &str) -> IResult<&str, (Module, &str)> {
    map(tuple((
        tag("&"),
        alpha1
    )), |(_, label)| (Conjunction { latest_inputs: Vec::new(), input_received: Vec::new() }, label))(input)
}

fn parse_module(input: &str) -> IResult<&str, (Module, &str)> {
    alt((
        parse_broadcaster,
        parse_flip_flop,
        parse_conjunction
    ))(input)
}

fn parse_planned_wiring(input: &str) -> IResult<&str, PlannedWiring> {
    map(tuple((
        parse_module,
        tag(" -> "),
        separated_list1(tag(", "), alpha1)
    )), |((module, label), _, outputs)| {
        PlannedWiring { label, module, outputs }
    })(input)
}

fn assign_module_indices<'a>(input: &Vec<PlannedWiring<'a>>) -> HashMap<&'a str, usize> {
    let mut result = HashMap::new();
    for planned in input {
        let mut len = result.len();
        result.entry(planned.label).or_insert(len);

        for output in &planned.outputs {
            len = result.len();
            result.entry(*output).or_insert(len);
        }
    }

    result
}

fn parse_and_reformat(input: &str) -> IResult<&str, Input> {
    map(separated_list1(line_ending, parse_planned_wiring), |wirings| {
        let index_assignment = assign_module_indices(&wirings);
        let mut inbound_count = vec![0; wirings.len()];
        let broadcaster_index = index_assignment["broadcaster"];
        let mut modules: Vec<_> = wirings.into_iter().map(|wiring| {
            let wires = wiring.outputs.into_iter().map(|target_label| {
                let to = index_assignment[target_label];
                let index_count_ptr = &mut inbound_count[to];
                let inbound_index = *index_count_ptr;
                *index_count_ptr += 1;
                Wire {
                    to,
                    inbound_index,
                }
            }).collect();
            WiredModule {
                module: wiring.module,
                wires,
            }
        }).collect();

        for (i, module) in modules.iter_mut().enumerate() {
            if let Conjunction { input_received, latest_inputs } = &mut module.module {
                *input_received = vec![false; inbound_count[i]];
                *latest_inputs = vec![false; inbound_count[i]];
            }
        }

        Input {
            broadcaster_index,
            modules,
        }
    })(input)
}

nom_solution!(parse_and_reformat);