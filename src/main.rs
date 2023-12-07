mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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

    day7::day7_1();
    day7::day7_2();
}

use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

