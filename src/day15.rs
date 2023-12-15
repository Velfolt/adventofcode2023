use std::collections::HashMap;

use indexmap::IndexMap;
use itertools::Itertools;
use nom::{
    bytes::complete::take_while1,
    character::complete::{digit1, one_of},
    combinator::{map_res, opt},
    sequence::tuple,
    IResult, Parser,
};

use crate::read_lines;

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, current| ((acc + current as usize) * 17) % 256)
}

pub fn day15_1() {
    let input: usize = read_lines("inputs/day15.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.split(",").map(hash).collect_vec())
        .sum();

    dbg!(input);
}

fn instruction(input: &str) -> IResult<&str, (&str, char, Option<usize>)> {
    tuple((
        take_while1(char::is_alphabetic),
        one_of("=-"),
        opt(map_res(digit1, str::parse::<usize>)),
    ))
    .parse(input)
}

pub fn day15_2() {
    let input = read_lines("inputs/day15.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.split(",").map(|x| x.to_string()).collect_vec())
        .collect_vec();

    let mut hashmap: HashMap<usize, IndexMap<String, usize>> = HashMap::new();

    for term in input {
        let (label, op, focal_length) = instruction(&term).unwrap().1;
        let boxhash = hash(label);
        let label = label.to_string();

        match op {
            '=' => {
                if let Some(entry) = hashmap.get_mut(&boxhash) {
                    entry.insert(label, focal_length.unwrap());
                } else {
                    let mut value = IndexMap::new();
                    value.insert(label, focal_length.unwrap());

                    hashmap.insert(boxhash, value);
                }
            }
            '-' => {
                if let Some(entry) = hashmap.get_mut(&boxhash) {
                    entry.shift_remove_entry(&label);
                }
            }
            _ => panic!(),
        }
    }

    let sum: usize = hashmap
        .iter()
        .map(|(box_index, map)| {
            map.iter()
                .enumerate()
                .map(|(slot_index, (label, focal_length))| {
                    (box_index + 1) * (slot_index + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum();

    dbg!(sum);
}
