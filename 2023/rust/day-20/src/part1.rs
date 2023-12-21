use std::collections::{HashMap, VecDeque};

use crate::custom_error::AocError;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use tracing::debug;

#[derive(Debug, Clone)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Pulse {
    src: String,
    dst: String,
    signal: Signal,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    FlipFlop { state: State },
    Conjunction { state: HashMap<String, Signal> },
    Broadcast,
    // Button,
}

#[derive(Debug, Clone)]
struct Module<'a> {
    id: &'a str,
    kind: ModuleKind,
    destination_modules: Vec<&'a str>,
}

impl Module<'_> {
    fn send(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match self.kind {
            ModuleKind::Broadcast => self
                .destination_modules
                .iter()
                .map(|dst| Pulse {
                    src: String::from(self.id),
                    dst: String::from(*dst),
                    signal: pulse.signal.clone(),
                })
                .collect(),
            ModuleKind::FlipFlop { ref mut state } => match pulse.signal {
                Signal::High => {
                    vec![]
                }
                Signal::Low => match state {
                    State::Off => {
                        *state = State::On;
                        self.destination_modules
                            .iter()
                            .map(|dst| Pulse {
                                src: String::from(self.id),
                                dst: String::from(*dst),
                                signal: Signal::High,
                            })
                            .collect()
                    }
                    State::On => {
                        *state = State::Off;
                        self.destination_modules
                            .iter()
                            .map(|dst| Pulse {
                                src: String::from(self.id),
                                dst: String::from(*dst),
                                signal: Signal::Low,
                            })
                            .collect()
                    }
                },
            },
            ModuleKind::Conjunction { ref mut state } => {
                state.entry(pulse.src).and_modify(|src| {
                    *src = pulse.signal.clone();
                });

                let signal = match state.values().all(|signal| match signal {
                    Signal::High => true,
                    Signal::Low => false,
                }) {
                    true => Signal::Low,
                    false => Signal::High,
                };

                self.destination_modules
                    .iter()
                    .map(|dst| Pulse {
                        src: String::from(self.id),
                        dst: String::from(*dst),
                        signal: signal.clone(),
                    })
                    .collect()
            }
        }
    }
}

fn parse_modules(input: &str) -> IResult<&str, HashMap<&str, Module>> {
    let (input, modules) = separated_list0(
        line_ending,
        separated_pair(
            alt((
                tag("broadcaster").map(|_| ("broadcaster", ModuleKind::Broadcast)),
                preceded(complete::char('%'), alpha1)
                    .map(|id| (id, ModuleKind::FlipFlop { state: State::Off })),
                preceded(complete::char('&'), alpha1).map(|id| {
                    (
                        id,
                        ModuleKind::Conjunction {
                            state: HashMap::new(),
                        },
                    )
                }),
            )),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
    )(input)?;

    Ok((
        input,
        modules
            .into_iter()
            .map(|((id, kind), destination_modules)| {
                (
                    id,
                    Module {
                        id,
                        kind,
                        destination_modules,
                    },
                )
            })
            .collect(),
    ))
}

fn update_conjunction_inputs(modules: &mut HashMap<&str, Module>) {
    let conjunction_ids = modules
        .iter()
        .filter_map(|(id, module)| match module.kind {
            ModuleKind::Conjunction { .. } => Some(*id),
            _ => None,
        })
        .collect::<Vec<_>>();

    let conjunction_inputs = modules.iter().fold(
        HashMap::<&str, Vec<&str>>::new(),
        |mut acc, (id, module)| {
            for c in conjunction_ids.iter() {
                if module.destination_modules.contains(c) {
                    acc.entry(c)
                        .and_modify(|item| {
                            item.push(id);
                        })
                        .or_insert(vec![id]);
                }
            }
            acc
        },
    );

    conjunction_inputs.into_iter().for_each(|(id, inputs)| {
        let conjunction = modules.get_mut(id).unwrap();
        if let ModuleKind::Conjunction { ref mut state } = conjunction.kind {
            for input in inputs {
                state.insert(String::from(input), Signal::Low);
            }
        }
    });
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, mut modules) = parse_modules(input).expect("should parse modules");
    update_conjunction_inputs(&mut modules);

    debug!(?modules);

    let (mut low_count, mut high_count) = (0u64, 0u64);
    let button_pushes = 1000;

    for push in 0..button_pushes {
        debug!("pushing the button for the {} time", push + 1);
        let mut pulse_bus = VecDeque::new();

        pulse_bus.push_back(Pulse {
            src: String::from("button"),
            dst: String::from("broadcaster"),
            signal: Signal::Low,
        });

        while let Some(pulse) = pulse_bus.pop_front() {
            debug!("{} -{:?}-> {}", pulse.src, pulse.signal, pulse.dst);
            match pulse.signal {
                Signal::High => high_count += 1,
                Signal::Low => low_count += 1,
            }

            let output = modules
                .get_mut(pulse.dst.as_str())
                .map(|module| module.send(pulse.clone()))
                .unwrap_or(vec![]);
            pulse_bus.extend(output);
        }
    }

    Ok(low_count * high_count)
}

#[cfg(test)]
mod day_20_part1 {
    use super::*;

    #[test_log::test]
    fn example1() -> miette::Result<()> {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(32000000, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn example2() -> miette::Result<()> {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(11687500, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(861743850, process(input)?);
        Ok(())
    }
}
