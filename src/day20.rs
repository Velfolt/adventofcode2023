use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::one_of,
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{terminated, tuple},
    IResult, Parser,
};
use num::Integer;

use crate::read_lines;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum ModuleType<'a> {
    Broadcaster,
    Button,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, PulseType>),
}

#[derive(Debug, Clone)]
struct Module<'a> {
    name: &'a str,
    module_type: ModuleType<'a>,
}

fn module_name(input: &str) -> IResult<&str, Module> {
    let (input, (module_type, name)) =
        tuple((opt(one_of("%&")), take_while1(char::is_alphabetic))).parse(input)?;

    Ok((
        input,
        Module {
            name,
            module_type: match module_type {
                Some(module_type) => {
                    if module_type == '%' {
                        ModuleType::FlipFlop(false)
                    } else {
                        ModuleType::Conjunction(HashMap::new())
                    }
                }
                None => ModuleType::Broadcaster,
            },
        },
    ))
}

fn module(input: &str) -> IResult<&str, (Module, Vec<&str>)> {
    let (input, module) = module_name(input)?;
    let (input, _) = tag(" -> ").parse(input)?;
    let (input, destinations) =
        separated_list1(tag(", "), take_while1(char::is_alphanumeric)).parse(input)?;

    Ok((input, (module, destinations)))
}

fn modules(input: &str) -> IResult<&str, HashMap<&str, (Module, Vec<&str>)>> {
    fold_many1(
        terminated(module, tag("\n")),
        || HashMap::new(),
        |mut map, (module, destinations)| {
            map.insert(module.name, (module, destinations));
            map
        },
    )
    .parse(input)
}

pub fn day20_1() {
    let input: String = read_lines("inputs/day20.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let mut modules = modules(&input).unwrap().1;
    modules.insert(
        "button",
        (
            Module {
                name: "button",
                module_type: ModuleType::Button,
            },
            vec!["broadcaster"],
        ),
    );

    let lookup = modules.clone();

    for (name, (module, _)) in modules.iter_mut() {
        if let ModuleType::Conjunction(memory) = &mut module.module_type {
            lookup
                .clone()
                .iter()
                .filter(|(_, (_, destinations))| destinations.contains(name))
                .for_each(|(name, _)| {
                    memory.insert(&name, PulseType::Low);
                });
        }
    }

    let mut history = vec![];

    for _ in 0..1000 {
        let mut pulses = VecDeque::from([("button", PulseType::Low, "")]);

        while let Some((module_name, pulse, sender)) = pulses.pop_front() {
            if module_name != "button" {
                history.push((module_name, pulse));
            }

            if let Some((ref mut module, destinations)) = modules.get_mut(module_name) {
                if let Some(new_pulse) = match module.module_type {
                    ModuleType::Broadcaster => Some(pulse),
                    ModuleType::Button => Some(PulseType::Low),
                    ModuleType::FlipFlop(ref mut on) => match pulse {
                        PulseType::High => None,
                        PulseType::Low => match on {
                            false => {
                                *on = true;
                                Some(PulseType::High)
                            }
                            true => {
                                *on = false;
                                Some(PulseType::Low)
                            }
                        },
                    },
                    ModuleType::Conjunction(ref mut memory) => {
                        memory.insert(sender, pulse);

                        // dbg!(&memory);

                        let all_high = memory.iter().all(|x| *x.1 == PulseType::High);

                        if all_high {
                            Some(PulseType::Low)
                        } else {
                            Some(PulseType::High)
                        }
                    }
                } {
                    destinations
                        .iter()
                        .for_each(|name| pulses.push_back((*name, new_pulse, module_name)));
                }
            }
        }
    }

    let (high, low): (Vec<PulseType>, Vec<PulseType>) = history
        .into_iter()
        .map(|(_, pulse)| pulse)
        .partition(|pulse| *pulse == PulseType::High);

    let product = high.len() * low.len();
    dbg!(product);
}

pub fn day20_2() {
    let input: String = read_lines("inputs/day20.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let mut set = HashSet::new();

    // taken from my input - all the modules that lead to rx
    let modules_to_investigate = ["st", "tn", "hh", "dt"];

    for module in modules_to_investigate {
        let mut counter = 0_usize;

        let mut modules = modules(&input).unwrap().1;
        modules.insert(
            "button",
            (
                Module {
                    name: "button",
                    module_type: ModuleType::Button,
                },
                vec!["broadcaster"],
            ),
        );

        let lookup = modules.clone();
        for (name, (module, _)) in modules.iter_mut() {
            if let ModuleType::Conjunction(memory) = &mut module.module_type {
                lookup
                    .clone()
                    .iter()
                    .filter(|(_, (_, destinations))| destinations.contains(name))
                    .for_each(|(name, _)| {
                        memory.insert(&name, PulseType::Low);
                    });
            }
        }

        'outer: loop {
            let mut pulses = VecDeque::from([("button", PulseType::Low, "")]);

            counter += 1;

            while let Some((module_name, pulse, sender)) = pulses.pop_front() {
                if sender == module && pulse == PulseType::High {
                    set.insert(counter);
                    break 'outer;
                }

                if let Some((ref mut module, destinations)) = modules.get_mut(module_name) {
                    if let Some(new_pulse) = match module.module_type {
                        ModuleType::Broadcaster => Some(pulse),
                        ModuleType::Button => Some(PulseType::Low),
                        ModuleType::FlipFlop(ref mut on) => match pulse {
                            PulseType::High => None,
                            PulseType::Low => match on {
                                false => {
                                    *on = true;
                                    Some(PulseType::High)
                                }
                                true => {
                                    *on = false;
                                    Some(PulseType::Low)
                                }
                            },
                        },
                        ModuleType::Conjunction(ref mut memory) => {
                            memory.insert(sender, pulse);

                            let all_high = memory.iter().all(|x| *x.1 == PulseType::High);

                            if all_high {
                                Some(PulseType::Low)
                            } else {
                                Some(PulseType::High)
                            }
                        }
                    } {
                        destinations
                            .iter()
                            .for_each(|name| pulses.push_back((*name, new_pulse, module_name)));
                    }
                }
            }
        }
    }

    let lcm = set.clone().into_iter().reduce(|acc, x| acc.lcm(&x));
    println!("{:?}", &lcm);
}
