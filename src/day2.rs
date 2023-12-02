use std::cmp;

use nom::{
    bytes::complete::{tag, take_until, take_while},
    combinator::map_res,
    multi::{separated_list0},
    IResult,
};

use crate::read_lines;

#[derive(Debug, PartialEq, Clone)]
struct Cube {
    color: String,
    amount: usize,
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, amount) = map_res(take_until(" "), |x: &str| x.parse())(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = take_while(char::is_alphanumeric)(input)?;

    Ok((
        input,
        Cube {
            color: color.to_string(),
            amount,
        },
    ))
}

#[test]
fn test_cube() {
    assert_eq!(
        cube("2 red").unwrap().1,
        Cube {
            color: "red".to_string(),
            amount: 2
        }
    );
}

fn bunch(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, bunch) = separated_list0(tag(", "), cube)(input)?;

    Ok((input, bunch))
}

#[test]
fn test_bunch() {
    assert_eq!(
        bunch("2 red, 4 blue").unwrap().1,
        vec!(
            Cube {
                color: "red".to_string(),
                amount: 2
            },
            Cube {
                color: "blue".to_string(),
                amount: 4
            }
        )
    );
}

fn bunches(input: &str) -> IResult<&str, Vec<Vec<Cube>>> {
    let (input, bunches) = separated_list0(tag("; "), bunch)(input)?;

    Ok((input, bunches))
}

#[test]
fn test_bunches() {
    assert_eq!(
        bunches("2 red, 4 blue; 4 red, 2 blue").unwrap().1,
        vec!(
            vec!(
                Cube {
                    color: "red".to_string(),
                    amount: 2
                },
                Cube {
                    color: "blue".to_string(),
                    amount: 4
                }
            ),
            vec!(
                Cube {
                    color: "red".to_string(),
                    amount: 4
                },
                Cube {
                    color: "blue".to_string(),
                    amount: 2
                }
            )
        )
    );
}

#[derive(Debug)]
struct Game {
    id: usize,
    bunches: Vec<Vec<Cube>>,
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(take_until(":"), |x: &str| x.parse())(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, bunches) = bunches(input)?;

    Ok((input, Game { id, bunches }))
}

fn is_game_possible(game: &Game, control: &Vec<Cube>) -> bool {
    for control_cube in control {
        for bunch in &game.bunches {
            for cube in bunch {
                if cube.color == control_cube.color && cube.amount > control_cube.amount {
                    println!("control: {:?} against {:?}", control_cube, cube);
                    return false
                }
            }
        }
    }

    true
}

pub fn day2_1() {
    let control = vec![
        Cube {
            color: "red".to_string(),
            amount: 12,
        },
        Cube {
            color: "green".to_string(),
            amount: 13,
        },
        Cube {
            color: "blue".to_string(),
            amount: 14,
        },
    ];

    let code: usize = read_lines("inputs/day2.txt")
        .map(|line| line.unwrap())
        .map(|line| game(&line.as_str()).unwrap().1)
        .filter(|game| is_game_possible(game, &control))
        .map(|game| game.id)
        .sum();

    dbg!(code);
}

fn game_max(game: &Game) -> Game {
    let mut cubes: Vec<Cube> = vec!();

    for bunch in &game.bunches {
        for cube in bunch {
            if let Some(max_cube) = cubes.iter_mut().find(|c| (**c).color == cube.color) {
                max_cube.amount = cmp::max(max_cube.amount, cube.amount)
            } else {
                cubes.push(cube.clone());
            }
        }
    }

    Game { id: game.id, bunches: vec!(cubes) }
}

fn game_power(game: &Game) -> usize {
    game.bunches.iter().flat_map(|b| b.iter().map(|c| c.amount)).product()
}

pub fn day2_2() {
    let code: usize = read_lines("inputs/day2.txt")
        .map(|line| line.unwrap())
        .map(|line| game(&line.as_str()).unwrap().1)
        .map(|game| game_max(&game))
        .map(|game| game_power(&game))
        .sum();

    dbg!(code);
}
