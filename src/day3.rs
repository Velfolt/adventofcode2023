use crate::read_lines;

fn expand_value(position: (usize, usize), schematic: &Vec<char>, width: usize) -> String {
    let mut left = position.0;
    let mut right = position.0;

    for x in (0..=position.0).rev() {
        if schematic[position.1 * width + x].is_numeric() {
            left = x;
        } else {
            break;
        }
    }

    for x in position.0..width {
        if schematic[position.1 * width + x].is_numeric() {
            right = x;
        } else {
            break;
        }
    }

    schematic[(position.1 * width + left)..=(position.1 * width + right)].into_iter().collect()
}

fn find_part_numbers(symbol: (usize, usize), schematic: &Vec<char>, width: usize) -> Vec<String> {
    let mut values = vec![];

    for x in (symbol.0 - 1)..=(symbol.0 + 1) {
        for y in (symbol.1 - 1)..=(symbol.1 + 1) {
            if (x, y) == symbol {
                continue;
            }

            let value = schematic[y * width + x];

            if !value.is_numeric() {
                continue;
            }

            values.push(expand_value((x, y), &schematic, width))
        }
    }
    
    values.sort();
    values.dedup();

    values
}

pub fn day3_1() {
    let schematic: Vec<_> = read_lines("inputs/day3.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let width = (schematic.len() as f32).sqrt() as usize;

    let part_numbers: Vec<_> = schematic
        .iter()
        .enumerate()
        .filter(|(_, value)| !value.is_numeric() && **value != '.')
        .flat_map(|(i, _)| find_part_numbers((i % width, i / width), &schematic, width))
        .collect();

    let code: usize = part_numbers.iter().map(|x| x.parse::<usize>().unwrap()).sum();

    dbg!(code);
}

pub fn day3_2() {
    let schematic: Vec<_> = read_lines("inputs/day3.txt")
        .map(|line| line.unwrap())
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let width = (schematic.len() as f32).sqrt() as usize;

    let gears: Vec<_> = schematic
        .iter()
        .enumerate()
        .filter(|(_, value)| !value.is_numeric() && **value != '.')
        .filter(|(_, value)| **value == '*')
        .map(|(i, _)| find_part_numbers((i % width, i / width), &schematic, width))
        .filter(|x| x.len() == 2)
        .map(|gears| gears.iter().map(|x| x.parse::<usize>().unwrap()).product())
        .collect();

    let code: usize = gears.iter().sum();

    dbg!(code);
}
