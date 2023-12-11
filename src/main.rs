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

fn main() {
    day1::day1_1();
    day1::day1_2();

    day2::day2_1();
    day2::day2_2();

    day3::day3_1();
    day3::day3_2();

    day4::day4_1();
    day4::day4_2();

    // day5::day5_1();
    // day5::day5_2();

    day6::day6_1();
    day6::day6_2();

    day7::day7_1();
    day7::day7_2();

    day8::day8_1();
    day8::day8_2();

    day9::day9_1();
    day9::day9_2();

    day10::day10_1();
    day10::day10_2();

    day11::day11_1();
    day11::day11_2();
}

use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub type Vec2 = (usize, usize);

pub fn manhattan_distance(a: &Vec2, b: &Vec2) -> usize {
    (a.0 as i64 - b.0 as i64).abs() as usize + (a.1 as i64 - b.1 as i64).abs() as usize
}
