use itertools::Itertools;
use num::integer::Roots;

use crate::{manhattan_distance, read_lines, Vec2};

#[derive(Debug)]
struct Map {
    map: Vec<char>,
    width: usize,
}

impl Map {
    fn new(map: Vec<char>) -> Self {
        let width = map.len().sqrt();
        Map { map, width }
    }

    fn expand(&mut self, amount: usize) -> Vec<Vec2> {
        let rows_to_add = (0..self.width).filter(|row| (0..self.width).all(|x| self.map[row * self.width + x] == '.')).collect_vec();
        let cols_to_add = (0..self.width).filter(|col| (0..self.width).all(|y| self.map[y * self.width + col] == '.')).collect_vec();

        self
            .map
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == '#')
            .map(|(i, _)| {
                let rows = rows_to_add.iter().filter(|&&x| x < i / self.width).count();
                let cols = cols_to_add.iter().filter(|&&x| x < i % self.width).count();

                let new_width = self.width + (cols_to_add.len() * amount);

                let x = ((i % self.width) + (cols * amount)) % new_width;
                let y = (i / self.width) + rows * amount;

                (x, y)
            })
            .collect_vec()
    }
}

pub fn day11_1() {
    let map = read_lines("inputs/day11.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut map = Map::new(map);
    let pairs: usize = map.expand(1).iter()
        .combinations(2)
        .map(|combinations| manhattan_distance(&combinations[0], &combinations[1]))
        .sum();

    dbg!(pairs);
}

pub fn day11_2() {
    let map = read_lines("inputs/day11.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut map = Map::new(map);
    let pairs: usize = map.expand(1_000_000 - 1).iter()
        .combinations(2)
        .map(|combinations| manhattan_distance(&combinations[0], &combinations[1]))
        .sum();

    dbg!(pairs);
}
