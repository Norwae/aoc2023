use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::rc::Rc;

use nom::IResult;

use crate::util::Index2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum RockType {
    Round,
    Cubic,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Rock {
    kind: RockType,
    position: Index2D,
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(if self.kind == RockType::Round { 'O' } else { '#' })
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Input {
    max_row: i32,
    rocks: Vec<Rock>,
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.rocks.is_empty() {
            return Ok(())
        }
        let mut rocks = self.rocks.clone();
        let width = rocks.iter().map(|r|r.position.0).max().unwrap();
        rocks.sort_by_key(|r|1000000 * r.position.1 + r.position.0);
        let mut rocks = rocks.as_slice();

        for row in 0..=self.max_row {
            for column in 0..=width {
                if rocks.is_empty() || rocks[0].position.1 != row || rocks[0].position.0 != column {
                    f.write_char('.')?;
                } else {
                    rocks[0].fmt(f)?;
                    rocks = &rocks[1..];
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Input {
    fn load(&self) -> i32 {
        self.rocks.iter().filter_map(|r| if r.kind == RockType::Round {
            Some(1 + self.max_row - r.position.1)
        } else {
            None
        }).sum()
    }

    fn spin_cycles(&mut self, count: usize) {
        let mut cache = HashMap::new();
        let mut evolution = Vec::new();

        for current_cycle in 0..count {
            let slf = Rc::new(self.clone());
            evolution.push(slf.clone());
            if let Some(lead_in) = cache.insert(slf.clone(), current_cycle){
                let cycle_length = current_cycle - lead_in;
                let steps_past_cycle_length = (count - lead_in) % cycle_length;
                let state_at_goal = evolution[lead_in + steps_past_cycle_length].clone();
                drop(evolution);
                drop(cache);
                *self = Rc::into_inner(state_at_goal).expect("now sole owner");
                return;
            }

            self.spin_cycle_naive();
        }
    }

    fn spin_cycle_naive(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_east(&mut self) {
        self.rocks.sort_by_key(|r| -10000 * r.position.1 - r.position.0);
        let last = &mut self.rocks[0];
        if last.kind == RockType::Round {
            last.position.0 = self.max_row
        }

        for idx in 1..self.rocks.len() {
            let next_position = self.rocks[idx - 1].position;
            let rock = &mut self.rocks[idx];
            if rock.kind == RockType::Round {
                if next_position.1 != rock.position.1 {
                    // roll all the way to the bottom
                    rock.position.0 = self.max_row
                } else {
                    // roll just below the previous one
                    rock.position.0 = next_position.0 - 1
                }
            }
        }
    }
    fn tilt_west(&mut self) {
        self.rocks.sort_by_key(|r| 10000 * r.position.1 + r.position.0);
        let first = &mut self.rocks[0];
        if first.kind == RockType::Round {
            first.position.0 = 0
        }

        for idx in 1..self.rocks.len() {
            let previous_position = self.rocks[idx - 1].position;
            let rock = &mut self.rocks[idx];
            if rock.kind == RockType::Round {
                if previous_position.1 != rock.position.1 {
                    // roll all the way to the top
                    rock.position.0 = 0
                } else {
                    // roll just above the previous one
                    rock.position.0 = previous_position.0 + 1
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        self.rocks.sort_by_key(|r| -10000 * r.position.0 - r.position.1);
        let last = &mut self.rocks[0];
        if last.kind == RockType::Round {
            last.position.1 = self.max_row
        }

        for idx in 1..self.rocks.len() {
            let next_position = self.rocks[idx - 1].position;
            let rock = &mut self.rocks[idx];
            if rock.kind == RockType::Round {
                if next_position.0 != rock.position.0 {
                    // roll all the way to the bottom
                    rock.position.1 = self.max_row
                } else {
                    // roll just below the previous one
                    rock.position.1 = next_position.1 - 1
                }
            }
        }
    }

    fn tilt_north(&mut self) {
        self.rocks.sort_by_key(|r| 10000 * r.position.0 + r.position.1);
        let first = &mut self.rocks[0];
        if first.kind == RockType::Round {
            first.position.1 = 0
        }

        for idx in 1..self.rocks.len() {
            let previous_position = self.rocks[idx - 1].position;
            let rock = &mut self.rocks[idx];
            if rock.kind == RockType::Round {
                if previous_position.0 != rock.position.0 {
                    // roll all the way to the top
                    rock.position.1 = 0
                } else {
                    // roll just above the previous one
                    rock.position.1 = previous_position.1 + 1
                }
            }
        }
    }
}

fn parse(input: &str) -> IResult<&str, Input> {
    let mut rocks = Vec::new();
    let mut max_row = 0;

    for (y, line) in input.lines().enumerate() {
        if line.trim() == "" {
            continue;
        }
        for (x, byte) in line.bytes().enumerate() {
            if byte != b'.' {
                let position = Index2D(x as i32, y as i32);
                let kind = if byte == b'#' {
                    RockType::Cubic
                } else {
                    RockType::Round
                };

                rocks.push(Rock { position, kind })
            }
        }

        max_row = y as i32
    }

    Ok(("", Input { max_row, rocks }))
}

fn part_1(input: &Input) -> i32 {
    let mut input = input.clone();
    input.tilt_north();
    input.load()
}

fn part_2(input: &Input) -> i32 {
    let mut input = input.clone();
    input.spin_cycles(1000000000);
    input.load()
}

solution!(parse, part_1, part_2);