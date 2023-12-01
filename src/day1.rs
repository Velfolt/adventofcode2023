use crate::read_lines;

pub fn day1_1() {
    let code = read_lines("inputs/day1.txt")
        .map(|line| line.unwrap())
        .map(|line| {
            let first_digit_pos = line.find(|c: char| c.is_digit(10)).unwrap();
            let second_digit_pos = line.rfind(|c: char| c.is_digit(10)).unwrap();

            (
                line.chars()
                    .nth(first_digit_pos)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32,
                line.chars()
                    .nth(second_digit_pos)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32,
            )
        })
        .map(|(a, b)| a * 10 + b)
        .sum::<i32>();

    dbg!(code);
}

fn match_substring(substring: &str) -> Option<i32> {
    match substring {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn find_first_digit(line: &String) -> Option<i32> {
    let first_digit_pos = line.find(|c: char| c.is_digit(10));
    let max_len = if let Some(max) = first_digit_pos {
        max
    } else {
        line.len()
    };

    for x in 0..max_len {
        for y in x..max_len {
            if let Some(digit) = match_substring(&line[x..=y]) {
                return Some(digit);
            }
        }
    }

    if let Some(first_digit_pos) = first_digit_pos {
        return Some(
            line.chars()
                .nth(first_digit_pos)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32,
        );
    }

    None
}

fn find_last_digit(line: &String) -> Option<i32> {
    let last_digit_pos = line.rfind(|c: char| c.is_digit(10));
    let min_len = if let Some(min) = last_digit_pos {
        min
    } else {
        0
    };

    for x in (min_len..line.len()).rev() {
        for y in (min_len..=x).rev() {
            if let Some(digit) = match_substring(&line[y..=x]) {
                return Some(digit);
            }
        }
    }

    if let Some(last_digit_pos) = last_digit_pos {
        return Some(
            line.chars()
                .nth(last_digit_pos)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32,
        );
    }

    None
}

pub fn day1_2() {
    let code = read_lines("inputs/day1.txt")
        .map(|line| line.unwrap())
        .map(|line| {
            (
                find_first_digit(&line).unwrap(),
                find_last_digit(&line).unwrap(),
            )
        })
        .map(|(a, b)| a * 10 + b)
        .sum::<i32>();

    dbg!(code);
}
