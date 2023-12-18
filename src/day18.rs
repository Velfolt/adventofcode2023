

use itertools::Itertools;

use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    sequence::{delimited, tuple},
    IResult, Parser,
};

use crate::{
    read_lines, Vec2,
};

fn enclosed_by_even_odd_rule(pos: &Vec2<i64>, path: &Vec<Vec2<i64>>) -> bool {
    let num = path.len();
    let mut j = num - 1;
    let mut c = false;

    let (x, y) = *pos;

    for i in 0..num {
        if x == path[i].0 && y == path[i].1 {
            // point is a corner
            return true;
        }

        if (path[i].1 > y) != (path[j].1 > y) {
            let slope: i64 = (x as i64 - path[i].0 as i64) * (path[j].1 as i64 - path[i].1 as i64)
                - (path[j].0 as i64 - path[i].0 as i64) * (y as i64 - path[i].1 as i64);
            if slope == 0 {
                // point is on boundary
                return true;
            }
            if (slope < 0) != (path[j].1 < path[i].1) {
                c = !c;
            }
        }

        j = i;
    }

    c
}

pub fn day18_1() {
    let mut position = (0_i64, 0_i64);

    let path = read_lines("inputs/day18.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| {
            line.split(" ")
                .map(|x| x.to_string())
                .collect_tuple::<(String, String, String)>()
        })
        .map(|(d, number, color)| {
            (
                d.as_bytes()[0] as char,
                number.parse::<usize>().unwrap(),
                color,
            )
        })
        .flat_map(|(d, number, _color)| {
            let dir = match d {
                'R' => (1, 0),
                'L' => (-1_i64, 0),
                'U' => (0, -1_i64),
                'D' => (0, 1),
                _ => unreachable!(),
            };

            let subpath = (0..number)
                .map(|i| {
                    let new_pos = (position.0 + dir.0 * i as i64, position.1 + dir.1 * i as i64);
                    new_pos
                })
                .collect_vec();

            position = (
                position.0 + dir.0 * number as i64,
                position.1 + dir.1 * number as i64,
            );

            subpath
        })
        .collect_vec();

    let (max_width, _) = path.iter().max_by(|(ax, _), (bx, _)| ax.cmp(bx)).unwrap();
    let (min_width, _) = path.iter().min_by(|(ax, _), (bx, _)| ax.cmp(bx)).unwrap();

    let (_, max_height) = path.iter().max_by(|(_, ay), (_, by)| ay.cmp(by)).unwrap();
    let (_, min_height) = path.iter().min_by(|(_, ay), (_, by)| ay.cmp(by)).unwrap();

    let width = min_width.abs() + max_width + 1;
    let height = min_height.abs() + max_height + 1;

    let enclosed = (0..(width * height))
        .map(|i| {
            (
                (i % width) as i64 + min_width,
                (i / width) as i64 + min_height,
            )
        })
        .filter(|pos| enclosed_by_even_odd_rule(pos, &path))
        .count();

    dbg!(enclosed);
}

fn instruction(input: &str) -> IResult<&str, (i64, i64)> {
    delimited(
        tag("(#"),
        tuple((
            map_res(take(5_usize), |x: &str| i64::from_str_radix(x, 16)),
            map_res(take(1_usize), str::parse),
        )),
        tag(")"),
    )
    .parse(input)
}

pub fn day18_2() {
    let mut position = (0_i64, 0_i64);

    let data = read_lines("inputs/day18.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| {
            line.split(" ")
                .map(|x| x.to_string())
                .collect_tuple::<(String, String, String)>()
        })
        .map(|(d, number, color)| {
            (
                d.as_bytes()[0] as char,
                number.parse::<i64>().unwrap(),
                instruction(&color).unwrap().1,
            )
        })
        .collect_vec();

    let boundary = data.iter().map(|(_, _, (number, _))| number).sum::<i64>();

    let path = data
        .into_iter()
        .map(|(_, _, (number, d))| {
            let dir = match d {
                0 => (1, 0),
                2 => (-1_i64, 0),
                3 => (0, -1_i64),
                1 => (0, 1),
                _ => unreachable!(),
            };

            let new_pos = (position.0 + dir.0 * number, position.1 + dir.1 * number);

            let response = new_pos;

            position = (
                position.0 + dir.0 * number as i64,
                position.1 + dir.1 * number as i64,
            );

            response
        })
        .collect_vec();

    // shoelace formula 
    let area = (0..path.len())
        .map(|i| {
            path[i].0 * path[(i + 1) % path.len()].1 - path[(i + 1) % path.len()].0 * path[i].1
        })
        .sum::<i64>()
        / 2;

    // pick's theorem: A = i + b/2 - 1
    // we want i for the internal points
    // so; i = A + 1 - boundary / 2
    let i = area + 1 - boundary / 2;
    dbg!(i + boundary);
}
