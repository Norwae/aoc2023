use std::collections::{HashMap, VecDeque};
use std::mem::swap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::tuple;

use crate::day20::Module::{Broadcaster, Conjunction, FlipFlop};
use crate::util::FixedLengthAsciiString;

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
    label: FixedLengthAsciiString<2>,
    module: Module,
    wires: Vec<Wire>,
}

#[derive(Debug)]
struct Pulse {
    emitter: FixedLengthAsciiString<2>,
    receiver: usize,
    to_slot: usize,
    high: bool,
}

impl WiredModule {
    fn process_pulse(&mut self, high: bool, to_slot: usize, mut enqueue: impl FnMut(Pulse)) {
        match &mut self.module {
            Broadcaster => {
                for Wire { to, inbound_index } in self.wires.iter() {
                    enqueue(Pulse {
                        emitter: self.label.clone(),
                        receiver: *to,
                        to_slot: *inbound_index,
                        high,
                    })
                }
            }
            FlipFlop { memory } => {
                if !high {
                    *memory = !*memory;
                    for Wire { to, inbound_index } in self.wires.iter() {
                        enqueue(Pulse {
                            emitter: self.label.clone(),
                            receiver: *to,
                            to_slot: *inbound_index,
                            high: *memory,
                        })
                    }
                }
            }
            Conjunction { latest_inputs } => {
                latest_inputs[to_slot] = high;
                let all_high = latest_inputs.iter().all(|v| *v);

                for Wire { to, inbound_index } in self.wires.iter() {
                    enqueue(Pulse {
                        emitter: self.label.clone(),
                        receiver: *to,
                        to_slot: *inbound_index,
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

const BUTTON_LABEL: &'static str = "!!";
const BROADCASTER_LABEL: &'static str = ">>";

const OUTSIDE_INDEX: usize = usize::MAX - 1;

impl Input {
    fn push_button(&mut self, mut on_pulse: impl FnMut(&Pulse)) {
        let mut queue = VecDeque::from([Pulse { emitter: FixedLengthAsciiString::new(BUTTON_LABEL), receiver: self.broadcaster_index, to_slot: 0, high: false }]);

        while let Some(next) = queue.pop_front() {
            on_pulse(&next);
            let handling_module = self.modules.get_mut(next.receiver);
            if let Some(wm) = handling_module {
                wm.process_pulse(next.high, next.to_slot, |p| queue.push_back(p))
            }
        }
    }
}

fn parse_broadcaster(input: &str) -> IResult<&str, (Module, &str)> {
    value((Broadcaster, BROADCASTER_LABEL), tag("broadcaster"))(input)
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
    map(separated_list1(line_ending, parse_wired), |mut wirings| {
        let mut module_indices = HashMap::with_capacity(wirings.len());
        let mut modules = Vec::with_capacity(wirings.len());
        let mut inbound_count = HashMap::with_capacity(wirings.len());

        for (label, module, wires) in &mut wirings {
            let label = FixedLengthAsciiString::new(label);
            module_indices.insert(label.clone(), modules.len());
            let mut wired = WiredModule {
                label,
                module: Broadcaster,
                wires: Vec::with_capacity(wires.len()),
            };

            swap(&mut wired.module, module);
            modules.push(wired)
        }

        for (n, (_, _, outputs)) in wirings.into_iter().enumerate() {
            let wired = &mut modules[n];

            for output in outputs {
                let output = FixedLengthAsciiString::new(output);
                let inbound_index = inbound_count.entry(output.clone()).or_insert(0usize);
                let to = *module_indices.get(&output).unwrap_or(&OUTSIDE_INDEX);

                wired.wires.push(Wire {
                    to,
                    inbound_index: *inbound_index,
                });
                *inbound_index += 1
            }
        }

        let mut broadcaster_index = usize::MAX;

        for (n, module) in modules.iter_mut().enumerate() {
            match &mut module.module {
                Conjunction { latest_inputs } =>
                    *latest_inputs = vec![false; inbound_count[&module.label]],
                Broadcaster => broadcaster_index = n,
                _ => ()
            }
        }

        Input { modules, broadcaster_index }
    })(input)
}

fn part_1(input: &Input) -> usize {
    let mut low_count = 0;
    let mut high_count = 0;
    let mut input = input.clone();

    for _ in 0..1000 {
        input.push_button(|Pulse { high, .. }| {
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

    let last_nand = input.modules.iter().position(|m| m.wires.len() == 1 && m.wires[0].to == OUTSIDE_INDEX).unwrap();
    let mut feeders = HashMap::<FixedLengthAsciiString<2>, Option<usize>>::new();
    for module in &input.modules {
        if module.wires.len() == 1 && module.wires[0].to == last_nand {
            feeders.insert(module.label.clone(), None);
        }
    }

    while feeders.values().any(|it| it.is_none()) {
        pushes += 1;
        input.push_button(|Pulse { high, emitter, .. }| {
            if *high {
                if let Some(opt) = feeders.get_mut(emitter) {
                    opt.get_or_insert(pushes);
                }
            }
        })
    }

    feeders.values().map(|x| x.unwrap()).product()
}

nom_solution!(parse_and_reformat, part_1, part_2);