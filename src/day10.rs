use itertools::Itertools;
use num::integer::Roots;

use crate::{read_lines, Vec2};

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn direction_to_pos(direction: &Direction, position: Vec2) -> Vec2 {
    match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::South => (position.0, position.1 + 1),
        Direction::East => (position.0 + 1, position.1),
        Direction::West => (position.0 - 1, position.1),
    }
}

fn is_reachable(pos: Vec2, from: Vec2, map: &Vec<char>, side: usize) -> bool {
    if let Some(tile) = map.get(pos.1 * side + pos.0) {
        let direction = match (pos.0 as i64 - from.0 as i64, pos.1 as i64 - from.1 as i64) {
            (0, 1) => Direction::North,
            (0, -1) => Direction::South,
            (1, 0) => Direction::West,
            (-1, 0) => Direction::East,
            _ => panic!()
        };

        tile_directions(*tile).contains(&direction)
    } else {
        false
    }
}

fn tile_directions(tile: char) -> Vec<Direction> {
    match tile {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::South, Direction::East],
        '.' => vec![],
        'S' => vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
        _ => panic!(),
    }
}

fn step_pipe(
    position: Vec2,
    map: &Vec<char>,
    side: usize,
) -> Vec<Vec2> {
    let directions_to_consider = tile_directions(map[position.1 * side + position.0]);

    directions_to_consider
        .iter()
        .map(|direction| direction_to_pos(direction, position))
        .filter(|pos| is_reachable(*pos, position, map, side))
        .collect_vec()
}

fn walk_pipe(start_position: Vec2,
    map: &Vec<char>,
    side: usize) -> Vec<(usize, usize)> {
    let mut history = vec!(start_position);

    let mut next_positions = step_pipe(start_position, &map, side);

    while !next_positions.is_empty() {
        let new_pos: (usize, usize) = next_positions.pop().unwrap();

        if history.contains(&new_pos) {
            continue
        }

        history.push(new_pos);

        next_positions = step_pipe(new_pos, &map, side);
    }

    history
}

fn find_start_pos(map: &Vec<char>, side: usize) -> Vec2 {
    let start_pos = map
        .iter()
        .enumerate()
        .find(|(_, &pipe)| pipe == 'S')
        .unwrap()
        .0;

    (start_pos % side, start_pos / side)
}

pub fn day10_1() {
    let map = read_lines("inputs/day10.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();

    let side = map.len().sqrt();
    let start_pos = find_start_pos(&map, side);
    let path = walk_pipe(start_pos, &map, side);
    let farthest_from_starting_pos = path.len() / 2;

    dbg!(farthest_from_starting_pos);
}

fn enclosed_by_even_odd_rule(pos: Vec2, path: &Vec<Vec2>) -> bool {
    let num = path.len();
    let mut j = num - 1;
    let mut c = false;

    let (x, y) = pos;

    for i in 0..num {
        if x == path[i].0 && y == path[i].1 {
            // point is a corner
            return true
        }

        if (path[i].1 > y) != (path[j].1 > y) {
            let slope: i64 = (x as i64 - path[i].0 as i64) * (path[j].1 as i64 - path[i].1 as i64) - (path[j].0 as i64 - path[i].0 as i64) * (y as i64 - path[i].1 as i64);
            if slope == 0 {
                // point is on boundary
                return true
            }
            if (slope < 0) != (path[j].1 < path[i].1) {
                c = !c;
            }
        }

        j = i;
    }

    c
}

// fn print_map(map: &Vec<char>, side: usize, path: &Vec<Vec2>) {
//     for (i, tile) in map.iter().enumerate() {
//         if i % side == 0 {
//             print!("\n");
//         }

//         if !path.contains(&(i % side, i / side)) {
//             if enclosed_by_even_odd_rule((i % side, i / side), &path) {
//                 print!("I");
//             } else {
//                 print!("O");
//             }
//         } else {
//             print!("{}", tile);
//         }
//     }
//     println!();
// }

pub fn day10_2() {
    let map = read_lines("inputs/day10.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();

    let side = map.len().sqrt();
    let start_pos = find_start_pos(&map, side);
    let path = walk_pipe(start_pos, &map, side);
    
    // print_map(&map, side, &path);

    let enclosed_tiles = map.iter().enumerate().filter(|(i, _)| !path.contains(&(i % side, i / side)) && enclosed_by_even_odd_rule((i % side, i / side), &path)).count();
    dbg!(enclosed_tiles);
}
