use num::integer::lcm;
use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while1},
    multi::{many0, separated_list1},
    sequence::tuple,
    IResult,
};

use crate::read_lines;

fn space(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("\n"))(input)
}

fn node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, node) = take_while1(char::is_alphanumeric)(input)?;
    let (input, _) = many0(tag(" = "))(input)?;
    let (input, (_, left, _, right, _)) = tuple((
        tag("("),
        take_while1(char::is_alphanumeric),
        tag(", "),
        take_while1(char::is_alphanumeric),
        tag(")"),
    ))(input)?;

    Ok((input, (node, (left, right))))
}

fn network(input: &str) -> IResult<&str, (&str, HashMap<&str, (&str, &str)>)> {
    let (input, paths) = take_while1(char::is_alphabetic)(input)?;
    let (input, _) = space(input)?;
    let (input, nodes) = separated_list1(tag("\n"), node)(input)?;
    let (input, _) = space(input)?;

    let mut hashmap = HashMap::new();
    for (key, value) in nodes {
        hashmap.insert(key, value);
    }

    Ok((input, (paths, hashmap)))
}

fn take_path<'a>(
    path: char,
    node: &'a str,
    nodes: &HashMap<&'a str, (&'a str, &'a str)>,
) -> &'a str {
    match path {
        'R' => nodes[node].1,
        'L' => nodes[node].0,
        _ => panic!("Unknown path"),
    }
}

pub fn day8_1() {
    let input: String = read_lines("inputs/day8.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let (paths, nodes) = network(&input).unwrap().1;
    let mut node = "AAA";

    let mut steps = 0;

    for path in paths.chars().cycle() {
        node = take_path(path, node, &nodes);

        steps += 1;

        if node == "ZZZ" {
            break;
        }
    }

    dbg!(steps);
}

fn take_path_owned<'a>(
    path: char,
    node: &'a str,
    nodes: &HashMap<&'a str, (&'a str, &'a str)>,
) -> String {
    match path {
        'R' => nodes[node].1.to_string(),
        'L' => nodes[node].0.to_string(),
        _ => panic!("Unknown path"),
    }
}

pub fn day8_2() {
    let input: String = read_lines("inputs/day8.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let (paths, nodes) = network(&input).unwrap().1;

    let start_nodes: Vec<String> = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|x| x.to_string())
        .collect();

    let mut step_vec = vec!();
    
    for starting_node in start_nodes.into_iter() {
        let mut steps = 0;
        let mut node = starting_node.clone();

        for path in paths.chars().cycle() {
            node = take_path_owned(path, &node, &nodes);

            steps += 1;

            if node.ends_with("Z") {
                break
            }
        }

        step_vec.push(steps);
    }

    let code: usize = step_vec.iter().map(|x| *x).reduce(|acc, x| lcm(x, acc)).unwrap();
    dbg!(code);
}
