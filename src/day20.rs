use std::collections::{HashMap, VecDeque};
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::tuple;
use crate::day20::Module::{Broadcaster, Conjunction, FlipFlop};

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
struct WiredModule {
    label: String,
    module: Module,
    wires: Vec<Wire>,
}

#[derive(Debug)]
struct Pulse {
    emitter: String,
    receiver: String,
    to_slot: usize,
    high: bool
}

impl WiredModule {
        fn process_pulse(&mut self, high: bool, to_slot: usize, mut enqueue: impl FnMut(Pulse)) {
            match &mut self.module {
                Broadcaster => {
                    for wire in self.wires.iter_mut() {
                        enqueue(Pulse {
                            emitter: self.label.clone(),
                            receiver: wire.to.clone(),
                            to_slot: wire.inbound_index,
                            high
                        })
                    }
                }
                FlipFlop { memory } => {
                    if !high {
                        *memory = !*memory;
                        for wire in self.wires.iter_mut() {
                            enqueue(Pulse {
                                emitter: self.label.clone(),
                                receiver: wire.to.clone(),
                                to_slot: wire.inbound_index,
                                high: *memory,
                            })
                        }
                    }
                }
                Conjunction { latest_inputs } => {
                    latest_inputs[to_slot] = high;
                    let all_high = latest_inputs.iter().all(|v|*v);

                    for wire in self.wires.iter_mut() {
                        enqueue(Pulse {
                            emitter: self.label.clone(),
                            receiver: wire.to.clone(),
                            to_slot: wire.inbound_index,
                            high: !all_high,
                        })
                    }
                }
            }
        }
}

#[derive(Debug, Clone)]
struct Wire {
    to: String,
    inbound_index: usize,
}

#[derive(Debug, Clone)]
struct Input {
    modules: HashMap<String, WiredModule>,
}

impl Input {

        fn push_button(&mut self, mut on_pulse: impl FnMut(&Pulse)) {
            let mut queue = VecDeque::from([Pulse { emitter: "button".to_string(), receiver: "broadcaster".to_string(), to_slot: 0, high: false }]);

            while let Some(next) = queue.pop_front() {
                on_pulse(&next);
                let handling_module = self.modules.get_mut(&next.receiver);
                if let Some(wm) = handling_module  {
                    wm.process_pulse(next.high, next.to_slot, |p| queue.push_back(p))
                }
            }
        }
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

fn parse_wired(input: &str) -> IResult<&str, (&str, Module, Vec<&str>)> {
    map(tuple((
        parse_module,
        tag(" -> "),
        separated_list0(tag(", "), alpha1)
    )), |((module, label), _, outputs)| {
        (label, module, outputs)
    })(input)
}

fn parse_and_reformat(input: &str) -> IResult<&str, Input> {
    map(separated_list1(line_ending, parse_wired), |wirings| {
        let mut modules = HashMap::new();
        let mut inbound_count = HashMap::new();

        for (label, module, outputs) in wirings {
            let mut module = WiredModule {
                module,
                label: label.to_string(),
                wires: Vec::with_capacity(outputs.len()),
            };

            for output in outputs {
                let inbound_index = inbound_count.entry(output).or_insert(0usize);
                module.wires.push(Wire {
                    to: output.to_string(),
                    inbound_index: *inbound_index,
                });
                *inbound_index += 1
            }

            modules.insert(label.to_string(), module);
        }

        for (key, module) in &mut modules {
            if let Conjunction { latest_inputs} = &mut module.module {
                *latest_inputs = vec![false; inbound_count[&key[..]]]
            }
        }

        Input { modules }
    })(input)
}

fn part_1(input: &Input) -> usize {
    let mut low_count =0;
    let mut high_count = 0;
    let mut input = input.clone();

    for _ in 0..1000 {
        input.push_button(|Pulse{ high, ..}|{
            if *high {
                high_count += 1
            } else {
                low_count += 1
            }
        });
    }

    low_count * high_count
}

fn part_2(input: &Input) -> usize {
    /* observations:

    rx is fed into by a single NAND (rs),
    which is in turn fed by NANDs (bt, dl, fr, rv), each servicing it individually

    There are 4 additional NANDs in the network, with significant fan-out
    mj, qs, rd, cs.

    There are counter-chains of flip-flops which feed into each other, and into collector nodes... as well
    as the "chaos factors of the 4 rogue NANDs
     */

    let mut pushes = 0;
    let mut input = input.clone();
    let mut bt_cycle: Option<usize> = None;
    let mut fr_cycle: Option<usize> = None;
    let mut rv_cycle: Option<usize> = None;
    let mut dl_cycle: Option<usize> = None;

    while  bt_cycle.is_none() || fr_cycle.is_none() || rv_cycle.is_none() || dl_cycle.is_none() {
        pushes += 1;
        input.push_button(|Pulse{high, emitter, ..}|{
            if *high {
                if emitter == "bt" {
                    bt_cycle.get_or_insert(pushes);
                }

                if emitter == "fr" {
                    fr_cycle.get_or_insert(pushes);
                }

                if emitter == "rv" {
                    rv_cycle.get_or_insert(pushes);
                }

                if emitter == "dl" {
                    dl_cycle.get_or_insert(pushes);
                }
            }
        })
    }

    dl_cycle.unwrap() * rv_cycle.unwrap() * fr_cycle.unwrap() * bt_cycle.unwrap()
}

nom_solution!(parse_and_reformat, part_1, part_2);