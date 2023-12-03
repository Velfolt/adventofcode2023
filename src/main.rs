mod day1;
mod day2;
mod day3;

fn main() {
    day1::day1_1();
    day1::day1_2();

    day2::day2_1();
    day2::day2_2();

    day3::day3_1();
    day3::day3_2();
}

use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

