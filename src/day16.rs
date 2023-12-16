use std::io::{self, BufRead, Lines};

use itertools::Itertools;

use crate::{
    grid::{Grid, GridWalkWithDirection, ToGrid},
    read_lines, Vec2,
};

fn energize_step(grid: &Grid, pos: &Vec2<i64>, direction: &Vec2<i64>) -> Vec<Vec2<i64>> {
    let tile = grid.data[(pos.1 * grid.width as i64 + pos.0) as usize];

    match (tile, direction) {
        ('.', _) => vec![*direction],

        ('/', (1, 0)) => vec![(0, -1)],
        ('/', (-1, 0)) => vec![(0, 1)],
        ('/', (0, -1)) => vec![(1, 0)],
        ('/', (0, 1)) => vec![(-1, 0)],

        ('\\', (1, 0)) => vec![(0, 1)],
        ('\\', (-1, 0)) => vec![(0, -1)],
        ('\\', (0, -1)) => vec![(-1, 0)],
        ('\\', (0, 1)) => vec![(1, 0)],

        ('|', (0, 1 | -1)) => vec![*direction],
        ('|', (1 | -1, 0)) => vec![(0, 1), (0, -1)],

        ('-', (0, 1 | -1)) => vec![(1, 0), (-1, 0)],
        ('-', (1 | -1, 0)) => vec![*direction],
        _ => panic!(),
    }
}

impl Grid {
    fn energized(&self) -> usize {
        self.data.iter().filter(|x| **x == '#').count()
    }
}

pub fn day16_1() {
    let mut grid = read_lines("inputs/day16.txt").to_grid();
    let mut energized = grid.clone();
    grid.walk_with_direction(&(0, 0), &(1, 0), energize_step, |pos| {
        energized.data[pos.1 as usize * energized.width + pos.0 as usize] = '#'
    });

    dbg!(energized.energized());
}

pub fn day16_2() {
    let mut grid = read_lines("inputs/day16.txt").to_grid();
    let length = grid.data.len();
    let width = grid.width;

    let max_energized = [(1_i64, 0_i64), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .flat_map(|direction| {
            (0..length)
                .map(|i| (i % width, i / width))
                .filter(|pos| pos.0 == 0 || pos.0 == width - 1 || pos.1 == 0 || pos.1 == width - 1)
                .map(|start_pos| ((start_pos.0 as i64, start_pos.1 as i64), *direction))
        })
        .map(|(pos, direction)| {
            let mut energized = grid.clone();
            grid.walk_with_direction(&pos, &direction, energize_step, |pos| {
                energized.data[pos.1 as usize * energized.width + pos.0 as usize] = '#'
            });
            energized.energized()
        })
        .max();

    dbg!(max_energized);
}
