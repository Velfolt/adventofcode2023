mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    // day1::day1_1();
    // day1::day1_2();

    // day2::day2_1();
    // day2::day2_2();

    // day3::day3_1();
    // day3::day3_2();

    // day4::day4_1();
    // day4::day4_2();

    // day5::day5_1();
    // day5::day5_2();

    // day6::day6_1();
    // day6::day6_2();

    // day7::day7_1();
    // day7::day7_2();

    // day8::day8_1();
    // day8::day8_2();

    // day9::day9_1();
    // day9::day9_2();

    // day10::day10_1();
    // day10::day10_2();

    // day11::day11_1();
    // day11::day11_2();

    // day12::day12_1();

    // day13::day13_1();
    // day13::day13_2();

    day14::day14_1();
    day14::day14_2();
}

use std::{
    fs::File,
    io::{self, BufRead}, collections::HashSet,
    hash::Hash
};

use itertools::Itertools;

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub type Vec2 = (usize, usize);

pub fn manhattan_distance(a: &Vec2, b: &Vec2) -> usize {
    (a.0 as i64 - b.0 as i64).abs() as usize + (a.1 as i64 - b.1 as i64).abs() as usize
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let width = input.find("\n").unwrap();

        Grid {
            grid: input.chars().filter(|c| *c != '\n').collect_vec(),
            width,
        }
    }

    pub fn print(&self) {
        for (i, tile) in self.grid.iter().enumerate() {
            if i % self.width == 0 {
                print!("\n");
            }

            print!("{}", tile);
        }
        println!();
    }
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}