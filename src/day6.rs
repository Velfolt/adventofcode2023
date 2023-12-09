use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use rayon::prelude::*;

use crate::read_lines;

fn space(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("\n"))(input)
}

fn digit(input: &str) -> IResult<&str, usize> {
    let (input, _) = many0(tag(" "))(input)?;
    map_res(take_while1(char::is_numeric), |x: &str| x.parse())(input)
}

fn boat_race(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, time) = separated_list1(many1(tag(" ")), digit)(input)?;
    let (input, _) = space(input)?;

    let (input, _) = tag("Distance: ")(input)?;
    let (input, distance) = separated_list1(many1(tag(" ")), digit)(input)?;
    let (input, _) = space(input)?;

    Ok((input, (time, distance)))
}

pub fn day6_1() {
    let input: String = read_lines("inputs/day6.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let (time, distance) = boat_race(&input).unwrap().1;

    let code: usize = time
        .iter()
        .zip(distance.iter())
        .map(|(time, distance)| {
            (0..*time)
                .filter(|hold| hold * (time - hold) > *distance)
                .count()
        })
        .inspect(|x| println!("{:?}", x))
        .product();

    dbg!(code);
}

fn digit_as_str(input: &str) -> IResult<&str, &str> {
    let (input, _) = many0(tag(" "))(input)?;
    take_while1(char::is_numeric)(input)
}

fn boat_race_ignore_space(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, time) = separated_list1(many1(tag(" ")), digit_as_str)(input)?;

    let time: usize = time
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    let (input, _) = space(input)?;

    let (input, _) = tag("Distance: ")(input)?;
    let (input, distance) = separated_list1(many1(tag(" ")), digit_as_str)(input)?;
    let distance: usize = distance
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    let (input, _) = space(input)?;

    Ok((input, (vec![time], vec![distance])))
}

pub fn day6_2() {
    let input: String = read_lines("inputs/day6.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let (time, distance) = boat_race_ignore_space(&input).unwrap().1;

    let code: usize = time
        .iter()
        .zip(distance.iter())
        .map(|(time, distance)| {
            (0..*time)
                .filter(|hold| hold * (time - hold) > *distance)
                .count()
        })
        .inspect(|x| println!("{:?}", x))
        .product();

    dbg!(code);
}
