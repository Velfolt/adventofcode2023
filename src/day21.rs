use std::collections::HashSet;

use itertools::Itertools;
use num::{integer::Roots, Integer};

use crate::{
    aoc_iteratorutils::AdventOfCodeIteratorUtils,
    grid::{Grid, GridBounds, GridFindPosition, GridGet, ToGrid},
    read_lines, Vec2,
};

impl Grid {
    fn step(&mut self, pos: &Vec2<i64>) -> Vec<Vec2<i64>> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
            .filter(|pos| self.in_bounds(pos) && self.get(pos) != '#')
            .collect()
    }
}

pub fn day21_1() {
    let mut grid = read_lines("inputs/day21.txt").to_grid();

    let start_pos = grid.find_pos(&'S').unwrap();
    let mut positions = vec![vec![start_pos]];

    for _ in 0..64 {
        let step_positions = positions.pop().unwrap();

        let mut pos = HashSet::new();
        for position in step_positions {
            let new_positions = grid.step(&position);
            // dbg!(&new_positions);
            pos.extend(new_positions);
        }

        positions.push(pos.into_iter().collect_vec());
    }

    dbg!(positions.pop().unwrap().len());
}

impl Grid {
    fn step_infinite(&self, pos: &Vec2<i64>) -> Vec<Vec2<i64>> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
            .filter(|pos| {
                self.get(&(
                    pos.0.rem_euclid(self.width as i64),
                    pos.1.rem_euclid(self.width as i64),
                )) != '#'
            })
            .collect()
    }
}

fn walk(grid: &Grid, steps: usize) -> usize {
    let start_pos = grid.find_pos(&'S').unwrap();
    let mut positions = vec![vec![start_pos]];

    for _ in 0..steps {
        let step_positions = positions.pop().unwrap();

        let mut pos = HashSet::new();
        for position in step_positions {
            let new_positions = grid.step_infinite(&position);
            pos.extend(new_positions);
        }

        positions.push(pos.into_iter().collect_vec());
    }

    positions.pop().unwrap().len()
}

pub fn day21_2() {
    let grid = read_lines("inputs/day21.txt").to_grid();

    let total_steps = 26501365;

    // let's find three data points to help us find a quadratic formula
    // using https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html
    let mut points = vec![];

    for steps in 1..1000000 {
        if steps % grid.width == total_steps % grid.width {
            let point = walk(&grid, steps);
            points.push(point)
        }

        if points.len() == 3 {
            break;
        }
    }

    if let [y0, y1, y2] = points[..] {
        let y0 = y0 as i64;
        let y1 = y1 as i64;
        let y2 = y2 as i64;

        let x = (total_steps.div_ceil(grid.width)) as i64;

        let (diff1, diff2) = (y1 - y0, y2 - y1);
        let second_diff = diff2 - diff1;

        let a = second_diff as i64 / 2;
        let b = diff1 as i64 - 3*a as i64;
        let c = y0 as i64 - b - a;

        dbg!(a * x * x + b * x + c);
    }
}
