mod day1;

fn main() {
    day1::day1_1();
    day1::day1_2();
}

use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

