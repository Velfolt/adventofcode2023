use itertools::Itertools;

use crate::read_lines;
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, opt, recognize},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

type Day9Integer = i128;

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789"))).parse(input)
}

fn digit(input: &str) -> IResult<&str, Day9Integer> {
    map_res(recognize(tuple((opt(tag("-")), decimal))), |x: &str| {
        x.parse()
    })
    .parse(input)
}

fn history(input: &str) -> IResult<&str, Vec<Day9Integer>> {
    separated_list1(tag(" "), digit)(input)
}

fn extrapolate(history: Vec<Day9Integer>) -> Day9Integer {
    let iter = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a);

    let diff = if iter.clone().all(|x| x == 0) {
        0
    } else {
        extrapolate(iter.collect_vec())
    };

    history.last().unwrap() + diff
}

pub fn day9_1() {
    let code = read_lines("inputs/day9.txt")
        .map(|line| line.unwrap())
        .map(|line| history(&line).unwrap().1)
        .map(extrapolate)
        .sum1::<i128>()
        .unwrap();

    dbg!(code);
}

fn extrapolate_first(history: Vec<Day9Integer>) -> Day9Integer {
    let iter = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a);

    let diff = if iter.clone().all(|x| x == 0) {
        0
    } else {
        extrapolate_first(iter.collect_vec())
    };

    history.first().unwrap() - diff
}

pub fn day9_2() {
    let code = read_lines("inputs/day9.txt")
        .map(|line| line.unwrap())
        .map(|line| history(&line).unwrap().1)
        .map(extrapolate_first)
        .sum1::<Day9Integer>()
        .unwrap();

    dbg!(code);
}
