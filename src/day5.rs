use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until, take_while1},
    combinator::map_res,
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    IResult,
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

#[derive(Debug, Clone)]
struct Mapping {
    dest: usize,
    source: usize,
    range: usize,
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, (dest, source, range)) = tuple((digit, digit, digit))(input)?;

    Ok((
        input,
        Mapping {
            dest,
            source,
            range,
        },
    ))
}

fn map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = space(input)?;
    let (input, _) = take_until(" map:\n")(input)?;
    let (input, _) = tag(" map:\n")(input)?;

    separated_list1(many1(tag("\n")), mapping)(input)
}

#[derive(Debug, Clone)]
struct Mapper {
    seeds: Vec<usize>,
    soil: Vec<Mapping>,
    fertilizer: Vec<Mapping>,
    water: Vec<Mapping>,
    light: Vec<Mapping>,
    temperature: Vec<Mapping>,
    humidity: Vec<Mapping>,
    location: Vec<Mapping>,
}

fn mapper(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(
        tag(" "),
        map_res(take_while1(char::is_numeric), |x: &str| x.parse()),
    )(input)?;
    let (input, _) = space(input)?;

    let (input, soil) = map(input)?;
    let (input, fertilizer) = map(input)?;
    let (input, water) = map(input)?;
    let (input, light) = map(input)?;
    let (input, temperature) = map(input)?;
    let (input, humidity) = map(input)?;
    let (input, location) = map(input)?;

    let (input, _) = space(input)?;

    Ok((
        input,
        Mapper {
            seeds,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location,
        },
    ))
}

fn map_seed(seeds: &Vec<usize>, mappings: &Vec<Mapping>) -> Vec<usize> {
    seeds
        .iter()
        .map(|seed| {
            if let Some(mapping) = mappings
                .iter()
                .find(|mapping| (mapping.source..(mapping.source + mapping.range)).contains(seed))
            {
                seed - mapping.source + mapping.dest
            } else {
                *seed
            }
        })
        .collect()
}

pub fn day5_1() {
    let input: String = read_lines("inputs/day5.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let mapper = mapper(&input).unwrap().1;

    let soil = map_seed(&mapper.seeds, &mapper.soil);
    let fertilizer = map_seed(&soil, &mapper.fertilizer);
    let water = map_seed(&fertilizer, &mapper.water);
    let light = map_seed(&water, &mapper.light);
    let temperature = map_seed(&light, &mapper.temperature);
    let humidity = map_seed(&temperature, &mapper.humidity);
    let location = map_seed(&humidity, &mapper.location);

    let min_location = location.iter().min().unwrap();

    dbg!(min_location);
}

fn map_seed_range(seed_range: &Vec<Range<usize>>, mapper: &Mapper) -> usize {
    seed_range
        .par_iter()
        .flat_map(|range| {
            range
                .clone()
                .into_par_iter()
                .map(|seed| {
                    let soil = map_seed(&[seed].to_vec(), &mapper.soil);
                    let fertilizer = map_seed(&soil, &mapper.fertilizer);
                    let water = map_seed(&fertilizer, &mapper.water);
                    let light = map_seed(&water, &mapper.light);
                    let temperature = map_seed(&light, &mapper.temperature);
                    let humidity = map_seed(&temperature, &mapper.humidity);
                    let location = map_seed(&humidity, &mapper.location);
                    location
                })
                .min()
                .unwrap()
        })
        .inspect(|x| println!("{:?}", x))
        .min()
        .unwrap()
}

pub fn day5_2() {
    let input: String = read_lines("inputs/day5.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let mapper = mapper(&input).unwrap().1;

    let seed_range: Vec<Range<usize>> = mapper
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();

    let min_location = map_seed_range(&seed_range, &mapper);

    dbg!(min_location);
}
