use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    character::complete::one_of,
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

use crate::read_lines;

#[derive(Debug, Clone, Copy)]
enum Rule<'a> {
    Rule(char, char, usize, &'a str),
    Destination(&'a str),
    Accept,
    Reject,
}

fn rule(input: &str) -> IResult<&str, Rule> {
    alt((
        map_res(
            tuple((
                one_of::<&str, &str, Error<_>>("xmas"),
                one_of("<>"),
                take_while1(char::is_numeric),
                one_of(":"),
                take_while1(char::is_alphabetic),
            )),
            |(var, op, num, _, dest)| {
                Ok::<Rule<'_>, ()>(Rule::Rule(var, op, num.parse().unwrap(), dest))
            },
        ),
        map_res(tag("A"), |_| Ok::<Rule<'_>, ()>(Rule::Accept)),
        map_res(tag("R"), |_| Ok::<Rule<'_>, ()>(Rule::Reject)),
        map_res(take_while1(char::is_alphabetic), |dest| {
            Ok::<Rule<'_>, ()>(Rule::Destination(dest))
        }),
    ))
    .parse(input)
}

fn workflow(input: &str) -> IResult<&str, (&str, Vec<Rule<'_>>)> {
    let (input, name) = take_while1(char::is_alphabetic).parse(input)?;
    let (input, rules) =
        delimited(tag("{"), separated_list1(tag(","), rule), tag("}")).parse(input)?;

    Ok((input, (name, rules)))
}

fn part_ratings(input: &str) -> IResult<&str, (usize, usize, usize, usize)> {
    let (input, (x, _, m, _, a, _, s)) = delimited(
        tag("{"),
        tuple((
            preceded(
                take(2_usize),
                map_res(take_while1(char::is_numeric), str::parse),
            ),
            tag(","),
            preceded(
                take(2_usize),
                map_res(take_while1(char::is_numeric), str::parse),
            ),
            tag(","),
            preceded(
                take(2_usize),
                map_res(take_while1(char::is_numeric), str::parse),
            ),
            tag(","),
            preceded(
                take(2_usize),
                map_res(take_while1(char::is_numeric), str::parse),
            ),
        )),
        tag("}"),
    )
    .parse(input)?;

    Ok((input, (x, m, a, s)))
}

fn system(
    input: &str,
) -> IResult<
    &str,
    (
        Vec<(&str, Vec<Rule<'_>>)>,
        Vec<(usize, usize, usize, usize)>,
    ),
> {
    let (input, workflows) = separated_list1(tag("\n"), workflow).parse(input)?;
    let (input, _) = tag("\n\n").parse(input)?;
    let (input, part_ratings) = separated_list1(tag("\n"), part_ratings).parse(input)?;

    Ok((input, (workflows, part_ratings)))
}

pub fn day19_1() {
    let input: String = read_lines("inputs/day19.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let (workflows, part_ratings) = system(&input).unwrap().1;

    let part_sum = part_ratings
        .iter()
        .filter(|(x, m, a, s)| {
            let mut result = None;

            let mut part_name = "in";

            while result.is_none() {
                let (_, rules) = workflows
                    .iter()
                    .find(|(name, _)| *name == part_name)
                    .unwrap();

                for rule in rules {
                    match rule {
                        Rule::Rule(var, op, rhs, dest) => {
                            let lhs = match var {
                                'x' => x,
                                'm' => m,
                                'a' => a,
                                's' => s,
                                _ => unreachable!(),
                            };

                            let comparison = match op {
                                '>' => lhs > rhs,
                                '<' => lhs < rhs,
                                _ => unreachable!(),
                            };

                            if comparison && *dest == "A" {
                                result = Some(true);
                                break;
                            } else if comparison && *dest == "R" {
                                result = Some(false);
                                break;
                            } else if comparison {
                                part_name = dest;
                                break;
                            }
                        }
                        Rule::Destination(name) => part_name = name,
                        Rule::Accept => result = Some(true),
                        Rule::Reject => result = Some(false),
                    }
                }
            }

            result.is_some_and(|x| x)
        })
        .map(|(x, m, a, s)| x + m + a + s)
        .sum::<usize>();

    dbg!(part_sum);
}

pub fn day19_2() {
    let input: String = read_lines("inputs/day19.txt")
        .map(|line| line.unwrap() + "\n")
        .collect();

    let (workflows, _) = system(&input).unwrap().1;

    let mut distinct_part_numbers = vec![];
    let mut stack = vec![((1, 4000), (1, 4000), (1, 4000), (1, 4000), "in", 0)];

    while let Some((x, m, a, s, part_name, rule_index)) = stack.pop() {
        if part_name == "A" {
            distinct_part_numbers.push((x, m, a, s));
            continue;
        }

        if part_name == "R" {
            continue;
        }

        let (_, rules) = workflows
            .iter()
            .find(|(name, _)| *name == part_name)
            .unwrap();

        let rule = &rules[rule_index];

        match rule {
            Rule::Rule(var, op, rhs, dest) => match (var, op) {
                ('x', '>') => {
                    stack.push(((rhs + 1, x.1), m, a, s, *dest, 0));
                    stack.push(((x.0, *rhs), m, a, s, part_name, rule_index + 1));
                }
                ('x', '<') => {
                    stack.push(((x.0, *rhs - 1), m, a, s, *dest, 0));
                    stack.push(((*rhs, x.1), m, a, s, part_name, rule_index + 1));
                }
                ('m', '>') => {
                    stack.push((x, (rhs + 1, m.1), a, s, *dest, 0));
                    stack.push((x, (m.0, *rhs), a, s, part_name, rule_index + 1));
                }
                ('m', '<') => {
                    stack.push((x, (m.0, *rhs - 1), a, s, *dest, 0));
                    stack.push((x, (*rhs, m.1), a, s, part_name, rule_index + 1));
                }
                ('a', '>') => {
                    stack.push((x, m, (rhs + 1, a.1), s, *dest, 0));
                    stack.push((x, m, (a.0, *rhs), s, part_name, rule_index + 1));
                }
                ('a', '<') => {
                    stack.push((x, m, (a.0, *rhs - 1), s, *dest, 0));
                    stack.push((x, m, (*rhs, a.1), s, part_name, rule_index + 1));
                }
                ('s', '>') => {
                    stack.push((x, m, a, (rhs + 1, s.1), *dest, 0));
                    stack.push((x, m, a, (s.0, *rhs), part_name, rule_index + 1));
                }
                ('s', '<') => {
                    stack.push((x, m, a, (s.0, *rhs - 1), *dest, 0));
                    stack.push((x, m, a, (*rhs, s.1), part_name, rule_index + 1));
                }
                _ => unreachable!(),
            },
            Rule::Destination(name) => stack.push((x, m, a, s, *name, 0)),
            Rule::Accept => stack.push((x, m, a, s, "A", 0)),
            Rule::Reject => stack.push((x, m, a, s, "R", 0)),
        }
    }

    let sum: usize = distinct_part_numbers
        .iter()
        .map(|(x, m, a, s)| (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1))
        .sum();

    dbg!(sum);
}
