use core::panic;
use std::{
    collections::{HashMap, HashSet, BinaryHeap},
    fmt::Debug,
    io::{BufRead, Lines},
    ops::Add, cmp::Reverse,
};

use itertools::Itertools;

use crate::{aoc_iteratorutils::AdventOfCodeIteratorUtils, Vec2};

pub trait ToGrid {
    fn to_grid(self) -> Grid;
}

impl<B: BufRead> ToGrid for Lines<B> {
    fn to_grid(self) -> Grid
    where
        Self: Sized,
    {
        let input: String = self
            .map(|line| line.unwrap() + "\n")
            .flat_map(|line| line.chars().collect_vec())
            .collect();

        Grid::new(&input)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub data: Vec<char>,
    pub width: usize,
}

pub trait GridPrinter {
    fn print(&self);
}

impl GridPrinter for Grid {
    fn print(&self) {
        for (i, tile) in self.data.iter().enumerate() {
            if i % self.width == 0 {
                print!("\n");
            }

            print!("{}", tile);
        }
        println!();
    }
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let width = input.find("\n").unwrap();

        Grid {
            data: input.chars().filter(|c| *c != '\n').collect_vec(),
            width,
        }
    }

    pub fn print(&self) {
        for (i, tile) in self.data.iter().enumerate() {
            if i % self.width == 0 {
                print!("\n");
            }

            print!("{}", tile);
        }
        println!();
    }
}

pub trait GridBounds<T> {
    fn in_bounds(&self, pos: &Vec2<T>) -> bool;
}

pub trait GridWalk<T> {
    fn walk<F: FnMut(&Self, &Vec2<T>) -> Vec<Vec2<T>>>(
        &mut self,
        start_pos: &Vec2<T>,
        step: F,
    ) -> Vec<Vec2<T>>;
}

impl<T: GridBounds<VecT>, VecT: Copy + PartialEq> GridWalk<VecT> for T {
    fn walk<F: FnMut(&Self, &Vec2<VecT>) -> Vec<Vec2<VecT>>>(
        &mut self,
        start_pos: &Vec2<VecT>,
        mut step: F,
    ) -> Vec<Vec2<VecT>> {
        let mut visited = vec![];

        let mut next_positions = vec![*start_pos];
        while !next_positions.is_empty() {
            let pos = next_positions.pop().unwrap();

            visited.push(pos);

            let positions = step(&self, &pos);

            for position in positions.iter().filter(|&&pos| self.in_bounds(&pos)) {
                if visited.iter().find(|vis| **vis == *position) == None {
                    next_positions.push(*position);
                }
            }
        }

        visited
    }
}

pub trait GridWalkWithDirection<VecT> {
    fn walk_with_direction(
        &mut self,
        start_pos: &Vec2<VecT>,
        start_direction: &Vec2<VecT>,
        step: impl FnMut(&Self, &Vec2<VecT>, &Vec2<VecT>) -> Vec<Vec2<VecT>>,
        visit: impl FnMut(&Vec2<VecT>),
    ) -> Vec<(Vec2<VecT>, Vec2<VecT>)>;
}

impl<T: GridBounds<VecT>, VecT: PartialEq + Copy + Add<Output = VecT>> GridWalkWithDirection<VecT>
    for T
{
    fn walk_with_direction(
        &mut self,
        start_pos: &Vec2<VecT>,
        start_direction: &Vec2<VecT>,
        mut step: impl FnMut(&Self, &Vec2<VecT>, &Vec2<VecT>) -> Vec<Vec2<VecT>>,
        mut visit: impl FnMut(&Vec2<VecT>),
    ) -> Vec<(Vec2<VecT>, Vec2<VecT>)> {
        let mut visited = vec![];

        let mut next_directions = vec![(*start_pos, *start_direction)];
        while !next_directions.is_empty() {
            let (pos, direction) = next_directions.pop().unwrap();

            visit(&pos);

            visited.push((pos, direction));

            let directions = step(&self, &pos, &direction);

            for direction in directions
                .iter()
                .filter(|&&dir| self.in_bounds(&(pos.0 + dir.0, pos.1 + dir.1)))
            {
                let value = ((pos.0 + direction.0, pos.1 + direction.1), *direction);

                if visited.iter().find(|vis| **vis == value) == None {
                    next_directions.push((value));
                }
            }
        }

        visited
    }
}

impl GridBounds<i64> for Grid {
    fn in_bounds(&self, pos: &Vec2<i64>) -> bool {
        if pos.0 < 0 || pos.0 >= self.width as i64 {
            false
        } else if pos.1 < 0 || pos.1 >= self.data.len() as i64 / self.width as i64 {
            false
        } else {
            true
        }
    }
}

impl GridBounds<usize> for Grid {
    fn in_bounds(&self, pos: &Vec2<usize>) -> bool {
        if pos.0 < 0 || pos.0 >= self.width as usize {
            false
        } else if pos.1 < 0 || pos.1 >= self.data.len() as usize / self.width as usize {
            false
        } else {
            true
        }
    }
}

impl GridBounds<i32> for Grid {
    fn in_bounds(&self, pos: &Vec2<i32>) -> bool {
        if pos.0 < 0 || pos.0 >= self.width as i32 {
            false
        } else if pos.1 < 0 || pos.1 >= self.data.len() as i32 / self.width as i32 {
            false
        } else {
            true
        }
    }
}

pub trait GridGet<VecT> {
    fn get(&self, pos: &Vec2<VecT>) -> char;
}

impl GridGet<usize> for Grid {
    fn get(&self, pos: &Vec2<usize>) -> char {
        self.data[pos.1 * self.width + pos.0]
    }
}

impl GridGet<i64> for Grid {
    fn get(&self, pos: &Vec2<i64>) -> char {
        self.data[pos.1 as usize * self.width + pos.0 as usize]
    }
}

pub trait GridAStar<VecT> {
    fn a_star(
        &mut self,
        start_pos: &Vec2<VecT>,
        end_pos: &Vec2<VecT>,
        max: &VecT,
        neighbours: impl FnMut(&Self, &Vec2<VecT>, &HashMap<Vec2<VecT>, Vec2<VecT>>) -> Vec<Vec2<VecT>>,
        heur: impl Fn(&Self, &Vec2<VecT>, &HashMap<Vec2<VecT>, Vec2<VecT>>) -> VecT,
        distance: impl Fn(&Self, &Vec2<VecT>, &Vec2<VecT>) -> VecT,
    ) -> Vec<Vec2<VecT>>;
}

impl<
        T: GridBounds<VecT>,
        VecT: PartialEq + Copy + Add<Output = VecT> + Ord + Debug + std::hash::Hash + Default,
    > GridAStar<VecT> for T
{
    fn a_star(
        &mut self,
        start_pos: &Vec2<VecT>,
        end_pos: &Vec2<VecT>,
        max: &VecT,
        mut neighbours: impl FnMut(
            &Self,
            &Vec2<VecT>,
            &HashMap<Vec2<VecT>, Vec2<VecT>>,
        ) -> Vec<Vec2<VecT>>,
        heur: impl Fn(&Self, &Vec2<VecT>, &HashMap<Vec2<VecT>, Vec2<VecT>>) -> VecT,
        distance: impl Fn(&Self, &Vec2<VecT>, &Vec2<VecT>) -> VecT,
    ) -> Vec<Vec2<VecT>> {
        let mut open_set = HashSet::new();
        open_set.insert(*start_pos);

        let mut came_from: HashMap<Vec2<VecT>, Vec2<VecT>> = HashMap::new();

        let mut g_score: HashMap<Vec2<VecT>, VecT> = HashMap::new();
        g_score.insert(*start_pos, Default::default());

        let mut f_score = HashMap::new();
        f_score.insert(*start_pos, heur(&self, start_pos, &came_from));

        while !open_set.is_empty() {
            let (mut current, _) = open_set
                .clone()
                .into_iter()
                .map(|pos| (pos, f_score.get(&pos).map_or(*max, |x| *x)))
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();

            if current == *end_pos {
                let mut total_path = vec![current];
                while came_from.contains_key(&current) {
                    current = came_from[&current];
                    total_path.insert(0, current);
                }
                return total_path;
            }

            open_set.remove(&current);

            for neighbour in neighbours(&self, &current, &came_from) {
                let tentative_g_score = g_score[&current] + distance(&self, &current, &neighbour);

                if !g_score.contains_key(&neighbour)
                    || tentative_g_score < g_score.get(&neighbour).map_or(*max, |x| *x)
                {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);
                    f_score.insert(
                        neighbour,
                        tentative_g_score + heur(&self, &neighbour, &came_from),
                    );
                    if !open_set.contains(&neighbour) {
                        open_set.insert(neighbour);
                    }
                }
            }
        }

        panic!()
    }
}
