use std::{collections::BTreeMap, cmp::Ordering};
use itertools::Itertools;

use nom::{
    bytes::complete::{tag, take_while1, take},
    combinator::map_res,
    multi::many0,
    sequence::tuple,
    IResult
};

use crate::read_lines;

fn digit(input: &str) -> IResult<&str, usize> {
    let (input, _) = many0(tag(" "))(input)?;
    map_res(take_while1(char::is_numeric), |x: &str| x.parse())(input)
}

fn card(input: &str) -> IResult<&str, usize> {
    let (input, card) = take(1_usize)(input)?;

    let card = match card {
        "A" => 0xe,
        "K" => 0xd,
        "Q" => 0xc,
        "J" => 0xb,
        "T" => 0xa,
        a => a.parse().unwrap()
    };

    Ok((input, card))
}

fn hand(input: &str) -> IResult<&str, (HandTuple, usize)> {
    let (input, hand) = tuple((card, card, card, card, card))(input)?;
    let (input, bid) = digit(input)?;

    Ok((input, (hand, bid)))
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn group_by_label<A, I>(v: I) -> BTreeMap<A, usize>
where
    A: Ord,
    I: IntoIterator<Item = A>,
{
    v.into_iter().fold(BTreeMap::new(), |mut acc, a| {
        *acc.entry(a).or_default() += 1;
        acc
    })
}

type HandTuple = (usize, usize, usize, usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord)]
struct Hand(HandTuple, HandType, usize);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.0.partial_cmp(&other.0)
    }
}

fn hand_type(hand: &HandTuple) -> HandType { 
    let (a, b, c, d, e) = hand;
    let sorted_hand = [a,b,c,d,e].to_vec();
    let labels = group_by_label(sorted_hand);
    let mut values: Vec<usize> = labels.into_values().collect();
    values.sort();

    let hand = match values.len() {
        1 => {
            let (a,) = values.iter().rev().take(1).collect_tuple::<(&usize,)>().unwrap();
            (a, &0_usize, &0_usize)
        },
        2 => {
            let (a, b) = values.iter().rev().take(2).collect_tuple::<(&usize, &usize)>().unwrap();
            (a, b, &0_usize)
        },
        _ => values.iter().rev().take(3).collect_tuple::<(&usize, &usize, &usize)>().unwrap()
    };

    match hand {
        (5, _, _) => HandType::FiveOfAKind,
        (4, _, _) => HandType::FourOfAKind,
        (3, 2, _) => HandType::FullHouse,
        (3, 1, 1) => HandType::ThreeOfAKind,
        (2, 2, _) => HandType::TwoPair,
        (2, _, _) => HandType::OnePair,
        (1, _, _) => HandType::HighCard,
        _ => HandType::HighCard
    }
}

pub fn day7_1() {
    let winnings: usize = read_lines("inputs/day7.txt")
        .map(|line| line.unwrap())
        .map(|line| hand(&line).unwrap().1)
        .map(|(hand, bid)| Hand(hand, hand_type(&hand), bid))
        .sorted()
        .enumerate()
        .map(|(rank, hand)| hand.2 * (rank + 1))
        .sum();

    dbg!(winnings);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord)]
struct HandWithJoker(HandTuple, HandType, usize);

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        match (self.0.0, other.0.0, self.0.0.partial_cmp(&other.0.0)) {
            (11, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, _) => return Some(Ordering::Less),
            (2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, 11, _) => return Some(Ordering::Greater),
            (_, _, Some(core::cmp::Ordering::Equal)) => {},
            (_, _, ord) => return ord,
        }

        match (self.0.1, other.0.1, self.0.1.partial_cmp(&other.0.1)) {
            (11, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, _) => return Some(Ordering::Less),
            (2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, 11, _) => return Some(Ordering::Greater),
            (_, _, Some(core::cmp::Ordering::Equal)) => {},
            (_, _, ord) => return ord,
        }

        match (self.0.2, other.0.2, self.0.2.partial_cmp(&other.0.2)) {
            (11, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, _) => return Some(Ordering::Less),
            (2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, 11, _) => return Some(Ordering::Greater),
            (_, _, Some(core::cmp::Ordering::Equal)) => {},
            (_, _, ord) => return ord,
        }

        match (self.0.3, other.0.3, self.0.3.partial_cmp(&other.0.3)) {
            (11, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, _) => return Some(Ordering::Less),
            (2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, 11, _) => return Some(Ordering::Greater),
            (_, _, Some(core::cmp::Ordering::Equal)) => {},
            (_, _, ord) => return ord,
        }

        match (self.0.4, other.0.4, self.0.4.partial_cmp(&other.0.4)) {
            (11, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, _) => return Some(Ordering::Less),
            (2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13, 11, _) => return Some(Ordering::Greater),
            (_, _, Some(core::cmp::Ordering::Equal)) => {},
            (_, _, ord) => return ord,
        }

        self.0.partial_cmp(&other.0)
    }
}

fn hand_type_with_joker(hand: &HandTuple) -> HandType { 
    let (a, b, c, d, e) = hand;
    let hand_list = [a,b,c,d,e].to_vec();
    let labels = group_by_label(hand_list);
        
    if let Some(jokers) = labels.get(&11).cloned() {
        let mut hands = vec!();

        for (&&key, _) in labels.iter().filter(|&(&&card, &amount)| amount < 5 && card != 11) {
            let mut partial_hand = labels.iter().filter(|&(&&card, &amount)| amount < 5 && card != 11).flat_map(|(&&value, &amount)| vec![value; amount]).collect_vec();
            for _ in 0..jokers {
                partial_hand.push(key);
            }

            let hand_tuple = if partial_hand.is_empty() {
                *hand
            } else {
                let [a, b, c,d,e] = partial_hand[..] else { todo!() };
                (a, b, c, d, e)
            };

            hands.push(hand_type(&hand_tuple));
            
        }

        hands.sort();

        if let Some(hand_type) = hands.last() {
            return *hand_type;
        }
    }

    hand_type(hand)
}

pub fn day7_2() {
    let winnings: usize = read_lines("inputs/day7.txt")
        .map(|line| line.unwrap())
        .map(|line| hand(&line).unwrap().1)
        .map(|(hand, bid)| HandWithJoker(hand, hand_type_with_joker(&hand), bid))
        .sorted()
        .enumerate()
        // .inspect(|x| println!("{:?}", x))
        .map(|(rank, hand)| hand.2 * (rank + 1))
        .sum();

    dbg!(winnings);
}