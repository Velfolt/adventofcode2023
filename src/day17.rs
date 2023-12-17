use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

use crate::{
    grid::{GridBounds, GridGet, ToGrid, Grid}, read_lines, Vec2,
};

fn same_direction(pos: &Vec2<i64>, came_from: &HashMap<Vec2<i64>, Option<Vec2<i64>>>) -> bool {
    if let Some(Some(last1)) = came_from.get(pos) {
        let dir = (pos.0 - last1.0, pos.1 - last1.1);

        if let Some(Some(last2)) = came_from.get(last1) {
            let dir2 = (last1.0 - last2.0, last1.1 - last2.1);

            if let Some(Some(last3)) = came_from.get(last2) {
                let dir3 = (last2.0 - last3.0, last2.1 - last3.1);

                if dir == dir2 && dir2 == dir3 {
                    return true;
                }
            }
        }
    }

    false
}

fn find_coldest_path_using_dijkstra(grid: &Grid, end_pos: Vec2<i64>, min: usize, max: usize) -> i64 {
    let mut distances = HashMap::new();
    let mut q = BinaryHeap::new();

    q.push((Reverse(0), (0, 0), (0, 0)));

    while let Some((Reverse(cost), pos, dir)) = q.pop() {
        if pos == end_pos {
            return cost;
        }

        if distances.get(&(pos, dir)).is_some_and(|&c| cost > c) {
            continue;
        }

        for new_dir in [(-1,0), (1,0), (0, -1), (0, 1)] {
            if dir == new_dir || dir == (-new_dir.0, -new_dir.1) {
                continue
            }

            let mut new_cost = cost;

            for distance in 1..=max {
                let new_pos = (pos.0 + new_dir.0 * distance as i64, pos.1 + new_dir.1 * distance as i64);
                if !grid.in_bounds(&new_pos) {
                    continue
                }

                new_cost += grid.get(&new_pos) as i64 - '0' as i64;

                if distance < min {
                    continue
                }

                if new_cost < *distances.get(&(new_pos, new_dir)).unwrap_or(&i64::MAX) {
                    distances.insert((new_pos, new_dir), new_cost);
                    q.push((Reverse(new_cost), new_pos, new_dir));
                }
            }
        }
    }

    panic!()
}

pub fn day17_1() {
    let grid = read_lines("inputs/day17.txt").to_grid();

    let end_pos = (grid.width as i64 - 1, grid.width as i64 - 1);

    let cost = find_coldest_path_using_dijkstra(&grid, end_pos, 1, 3);
    dbg!(cost);
}

pub fn day17_2() {
    let grid = read_lines("inputs/day17.txt").to_grid();

    let end_pos = (grid.width as i64 - 1, grid.width as i64 - 1);

    let cost = find_coldest_path_using_dijkstra(&grid, end_pos, 4, 10);
    dbg!(cost);
}
