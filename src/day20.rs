use std::collections::{HashMap, VecDeque};
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
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
    },
}


#[derive(Debug, Clone)]
struct PlannedWiring<'a> {
    label: &'a str,
    module: Module,
    outputs: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct WiredModule {
    module: Module,
    wires: Vec<Wire>,
}

impl WiredModule {

    fn process_pulse(&mut self, pulse: Pulse, emitted_target: &mut VecDeque<Pulse>) {
        let Pulse { high, input_port, ..} = pulse;
        match &mut self.module {
            Module::Broadcaster => {
                for wire in self.wires.iter_mut() {
                    emitted_target.push_back(Pulse {
                        to_module: wire.to,
                        input_port: wire.inbound_index,
                        high,
                    })
                }
            }
            FlipFlop { memory } => {
                if !high {
                    *memory = !*memory;
                    for wire in self.wires.iter_mut() {
                        emitted_target.push_back(Pulse {
                            to_module: wire.to,
                            input_port: wire.inbound_index,
                            high: *memory,
                        })
                    }
                }
            }
            Conjunction { latest_inputs } => {
                latest_inputs[input_port] = high;
                let all_high = latest_inputs.iter().all(|v|*v);

                for wire in self.wires.iter_mut() {
                    emitted_target.push_back(Pulse {
                        to_module: wire.to,
                        input_port: wire.inbound_index,
                        high: !all_high,
                    })
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Wire {
    to: usize,
    inbound_index: usize,
}

#[derive(Debug, Clone)]
struct Input {
    broadcaster_index: usize,
    modules: Vec<WiredModule>,
}

impl Input {
    fn push_button(&mut self) -> (usize, usize) {
        let mut low = 0;
        let mut high = 0;
        let mut queue = VecDeque::from([Pulse { to_module: self.broadcaster_index, input_port: 0usize, high: false }]);

        while let Some(next) = queue.pop_back() {
            if next.high {
                high += 1;
            } else {
                low += 1;
            }
            let wired_module = &mut self.modules[next.to_module];
            wired_module.process_pulse(next, &mut queue);
        }

        (low, high)
    }
}

#[derive(Debug)]
struct Pulse {
    to_module: usize,
    input_port: usize,
    high: bool
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
    )), |(_, label)| (Conjunction { latest_inputs: Vec::new() }, label))(input)
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
        separated_list0(tag(", "), alpha1)
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
            let wires = wiring.outputs.into_iter().filter_map(|target_label| {
                let to = index_assignment[target_label];
                if to >= inbound_count.len() {
                    eprintln!("Undefined module named {} found, ignoring wire", target_label);
                    None
                } else {
                    let index_count_ptr = &mut inbound_count[to];
                    let inbound_index = *index_count_ptr;
                    *index_count_ptr += 1;
                    Some(Wire {
                        to,
                        inbound_index,
                    })
                }
            }).collect();
            WiredModule {
                module: wiring.module,
                wires,
            }
        }).collect();

        for (i, module) in modules.iter_mut().enumerate() {
            if let Conjunction { latest_inputs } = &mut module.module {
                *latest_inputs = vec![false; inbound_count[i]];
            }
        }

        Input {
            broadcaster_index,
            modules,
        }
    })(input)
}

fn part_1(input: &Input) -> usize {
    let mut input = input.clone();
    let (l, r) = (0..1000).map(|_|input.push_button()).fold((0usize, 0usize), |(l0, r0), (l1, r1)| {
        (l0 + l1, r0 + r1)
    });

    l * r
}

nom_solution!(parse_and_reformat, part_1);