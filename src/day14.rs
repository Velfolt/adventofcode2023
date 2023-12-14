use core::panic;

use itertools::Itertools;

use crate::{read_lines, Grid};

impl Grid {
    fn tilt_step(&mut self, direction: (i64, i64)) -> bool {
        let mut changed = false;

        for (i, _) in self
            .grid
            .clone()
            .iter()
            .enumerate()
            .filter(|(_, element)| **element == 'O')
        {
            let pos = (i as i64 % self.width as i64, i as i64 / self.width as i64);
            let tilted_pos = (pos.0 - direction.0, pos.1 - direction.1);

            let width = self.width as i64;

            if direction == (0, 1) && tilted_pos.1 == -1 {
                continue;
            } else if direction == (-1, 0) && tilted_pos.0 == width {
                continue;
            } else if direction == (0, -1) && tilted_pos.1 == width {
                continue;
            } else if direction == (1, 0) && tilted_pos.0 == -1 {
                continue;
            }

            let grid = &mut self.grid;
            let tilted = grid
                .get_mut(tilted_pos.1 as usize * self.width as usize + tilted_pos.0 as usize)
                .unwrap();

            if *tilted != '#' && *tilted != 'O' {
                *tilted = 'O';

                let current = grid
                    .get_mut(pos.1 as usize * self.width as usize + pos.0 as usize)
                    .unwrap();
                *current = '.';
                changed = true;
            }
        }

        changed
    }

    fn tilt(&mut self, direction: (i64, i64)) {
        while self.tilt_step(direction) {}
    }

    fn load(&self) -> usize {
        self.grid
            .clone()
            .iter()
            .enumerate()
            .filter(|(_, element)| **element == 'O')
            .map(|(i, _)| {
                let pos = (i % self.width, i / self.width);

                self.width - pos.1
            })
            .sum()
    }
}

pub fn day14_1() {
    let input: String = read_lines("inputs/day14.txt")
        .map(|line| line.unwrap() + "\n")
        .flat_map(|line| line.chars().collect_vec())
        .collect();

    let mut grid = Grid::new(&input);

    let direction = (0, 1);

    grid.tilt(direction);
    dbg!(grid.load());
}

impl Grid {
    fn cycle(&mut self) {
        self.tilt((0, 1));
        self.tilt((1, 0));
        self.tilt((0, -1));
        self.tilt((-1, 0));
    }
}

fn find_cycle(loads: &[usize]) -> (usize, &[usize])
{
    for i in 0..loads.len() {
        for j in 2..loads.len() {
            if let Some((a, b)) = loads[i..]
                .chunks(j)
                .take(2)
                .collect_tuple::<(&[usize], &[usize])>()
            {
                if a.iter().zip(b.iter()).all(|(a, b)| *a == *b) {
                    return (i, a);
                }
            } else {
                break;
            }
        }
    }

    panic!()
}

pub fn day14_2() {
    let input: String = read_lines("inputs/day14.txt")
        .map(|line| line.unwrap() + "\n")
        .flat_map(|line| line.chars().collect_vec())
        .collect();

    let mut grid = Grid::new(&input);

    let loads = (0..1000)
        .map(|_| {
            grid.cycle();
            grid.load()
        })
        .collect_vec();

    let (index, cycle) = find_cycle(&loads);

    dbg!(cycle.iter().cycle().nth(1000000000 - index - 1).unwrap());
}
