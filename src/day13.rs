use itertools::Itertools;

use crate::read_lines;

#[derive(Debug, Clone)]
struct Pattern {
    pattern: Vec<char>,
    width: usize,
}

impl Pattern {
    fn new(pattern: &str) -> Pattern {
        let width = pattern.find("\n").unwrap();

        Pattern {
            pattern: pattern.chars().filter(|c| *c != '\n').collect_vec(),
            width,
        }
    }

    fn print(&self) {
        for (i, tile) in self.pattern.iter().enumerate() {
            if i % self.width == 0 {
                print!("\n");
            }

            print!("{}", tile);
        }
        println!();
    }
}

fn vertical_reflection(pattern: &Pattern, not_value: Option<usize>) -> Option<usize> {
    let mut reflecting_cols = vec![];
    let mut columns = vec![];

    for col in 0..pattern.width {
        let column = pattern
            .pattern
            .iter()
            .skip(col)
            .step_by(pattern.width)
            .collect_vec();

        if columns.contains(&column) {
            reflecting_cols.push(col);
        }

        columns.push(column);
    }

    let mut output = vec![];

    'outer: for start_col in reflecting_cols {
        for (i, col) in (start_col..pattern.width).enumerate() {
            if start_col - i == 0 {
                break;
            }

            let reflected_column = pattern
                .pattern
                .iter()
                .skip(start_col - i - 1)
                .step_by(pattern.width)
                .collect_vec();
            let column = pattern
                .pattern
                .iter()
                .skip(col)
                .step_by(pattern.width)
                .collect_vec();

            if column != reflected_column {
                continue 'outer;
            }
        }

        output.push(start_col);
    }

    let iter = output.iter().copied();
    if let Some(not_value) = not_value {
        iter.filter(|&x| x != not_value).last()
    } else {
        iter.last()
    }
}

fn horizontal_reflection(pattern: &Pattern, not_value: Option<usize>) -> Option<usize> {
    let mut reflecting_rows = vec![];

    let mut rows = vec![];

    for row_index in 0..(pattern.pattern.len() / pattern.width) {
        let row = pattern.pattern
            [row_index * pattern.width..row_index * pattern.width + pattern.width]
            .to_vec();

        if rows.contains(&row) {
            reflecting_rows.push(row_index);
        }

        rows.push(row);
    }

    let mut output = vec![];

    'outer: for start_row in reflecting_rows {
        for (i, row_index) in (start_row..(pattern.pattern.len() / pattern.width)).enumerate() {
            if start_row - i == 0 {
                break;
            }

            let reflected_row = pattern.pattern[(start_row - i - 1) * pattern.width
                ..(start_row - i - 1) * pattern.width + pattern.width]
                .to_vec();
            let row = pattern.pattern
                [row_index * pattern.width..row_index * pattern.width + pattern.width]
                .to_vec();

            if row != reflected_row {
                continue 'outer;
            }
        }

        output.push(start_row);
    }

    let iter = output.iter().copied();
    if let Some(not_value) = not_value {
        let not_value = not_value / 100;
        iter.filter(|&x| x != not_value).last()
    } else {
        iter.last()
    }
}

fn reflection(pattern: Pattern, not_value: Option<usize>) -> (Option<usize>, Option<usize>) {
    let vertical = vertical_reflection(&pattern, not_value);
    let horizontal = horizontal_reflection(&pattern, not_value);

    (vertical, horizontal)
}

fn choose_reflection((vertical, horizontal): (Option<usize>, Option<usize>)) -> usize {
    match (vertical, horizontal) {
        (Some(vertical), _) => vertical,
        (_, Some(horizontal)) => 100 * horizontal,
        _ => panic!(),
    }
}

fn fix_smudge(pattern: Pattern) -> usize {
    let old_reflection = choose_reflection(reflection(pattern.clone(), None));

    for i in 0..pattern.pattern.len() {
        let mut new_pattern = pattern.clone();

        if let Some(char) = new_pattern.pattern.get_mut(i) {
            *char = match char {
                '#' => '.',
                '.' => '#',
                _ => panic!(),
            };
        }

        let (vertical, horizontal) = reflection(new_pattern.clone(), Some(old_reflection));

        if let Some(vertical) = vertical {
            if old_reflection != vertical {
                return vertical;
            }
        }

        if let Some(horizontal) = horizontal {
            if old_reflection != horizontal * 100 {
                return 100 * horizontal;
            }
        }
    }

    panic!()
}

pub fn day13_1() {
    let sum: usize = read_lines("inputs/day13.txt")
        .map(|line| line.unwrap() + "\n")
        .coalesce(|x, y| match (x.as_str(), y.as_str()) {
            (_, "\n") => Err((x, y)),
            ("\n", _) => Ok(y),
            _ => Ok(x + &y[..]),
        })
        .map(|pattern| Pattern::new(&pattern))
        .map(|pattern| reflection(pattern, None))
        .map(choose_reflection)
        // .inspect(|x| println!("{:?}", x))
        .sum();

    dbg!(sum);
}

pub fn day13_2() {
    let sum: usize = read_lines("inputs/day13.txt")
        .map(|line| line.unwrap() + "\n")
        .coalesce(|x, y| match (x.as_str(), y.as_str()) {
            (_, "\n") => Err((x, y)),
            ("\n", _) => Ok(y),
            _ => Ok(x + &y[..]),
        })
        .map(|pattern| Pattern::new(&pattern))
        .map(fix_smudge)
        // .inspect(|x| println!("{:?}", x))
        .sum();

    dbg!(sum);
}
