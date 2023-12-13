use itertools::Itertools;

use crate::read_lines;

// fn arrangements((conditions, groups): &(String, Vec<usize>)) -> usize {
//     dbg!(&groups);
//     dbg!(&conditions);

//     let mut condition_iter = conditions.chars();
//     let mut group_iter = groups.iter().permutations(k)

//     let mut arrangement = 1;

//     loop {
//         if let Some(group) = group_iter.next() {
//             let mut current_group = *group as i64;
//             if let Some(char) = condition_iter.next() {
//                 println!("{:?}", (char, current_group));
//                 match (char, current_group) {
//                     ('.' | '?', 0) => break,
//                     ('?' | '#', _) => {
//                         arrangement += arrangements(&(condition_iter.clone().collect(), group_iter.clone().map(|x| *x).collect()));
//                         current_group -= 1
//                     },
//                     _ => {},
//                 };
//             } else {
//                 break
//             }
//         } else {
//             break
//         }
//     }

//     arrangement
// }

fn arrangements((conditions, groups): &(String, Vec<usize>)) -> usize {
    dbg!(conditions.chars().map(|x| match x { '?' | '#' => 1, _ => 0 }).collect_vec());
    let unknowns = conditions.chars().filter(|x| *x == '?').count();
    let permutations = conditions.chars().filter(|x| *x == '?').enumerate().map(|(i,_)| i).permutations(unknowns).collect_vec();
    dbg!(unknowns);
    dbg!(permutations);
    // let permutations = 2_usize.pow(unknowns.len() as u32)


    0
}

pub fn day12_1() {
    // let arrangements: usize = read_lines("inputs/day12.txt")
    //     .map(|line| line.unwrap())
    //     .flat_map(|line| line.split(" ").map(|x| x.to_string()).collect_tuple::<(String, String)>())
    //     .map(|(conditions, contiguous_groups_of_damaged_springs)| (conditions, contiguous_groups_of_damaged_springs.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec()))
    //     .map(|record| arrangements(&record))
    //     .inspect(|x| println!("{:?}", x))
    //     .take(1)
    //     .sum();

    // dbg!(arrangements);
}
