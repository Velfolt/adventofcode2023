use nom::{
    bytes::complete::{tag, take_until, take_while1},
    combinator::map_res,
    multi::{many0, many1, separated_list0},
    IResult,
};

use crate::read_lines;

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = many1(tag(" "))(input)?;
    let (input, id) = map_res(take_until(":"), |x: &str| x.parse())(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = many1(tag(" "))(input)?;

    let (input, winning_numbers) = separated_list0(
        many1(tag(" ")),
        map_res(take_while1(char::is_alphanumeric), |x: &str| x.parse()),
    )(input)?;
    let (input, _) = tag(" | ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, my_numbers) = separated_list0(
        many1(tag(" ")),
        map_res(take_while1(char::is_alphanumeric), |x: &str| x.parse()),
    )(input)?;

    Ok((
        input,
        Card {
            id,
            winning_numbers,
            my_numbers,
        },
    ))
}

pub fn day4_1() {
    let lottery: Vec<_> = read_lines("inputs/day4.txt")
        .map(|line| line.unwrap())
        .map(|line| card(&line.as_str()).unwrap().1)
        .map(|card| {
            card.my_numbers.iter().fold(0, |acc, number| {
                if card.winning_numbers.contains(number) {
                    if acc == 0 {
                        return 1;
                    } else {
                        return acc << 1;
                    }
                }

                acc
            })
        })
        .collect();

    let code: i32 = lottery.iter().sum();

    dbg!(code);
}

#[derive(Clone)]
struct CardPrecalculated {
    card: Card,
    calculated: usize,
}

fn play_scratch_game(cards: &[CardPrecalculated], lookup: &[CardPrecalculated]) -> usize {
    let mut stack = cards.to_vec();
    let mut output = 0;

    while !stack.is_empty() {
        let card = stack.pop().unwrap();

        let winning_cards = card.calculated;

        let extra_cards = &lookup[card.card.id..(card.card.id + winning_cards)];
        stack.append(&mut extra_cards.to_vec());

        output += 1;
    }

    output
}

pub fn day4_2() {
    let lottery: Vec<_> = read_lines("inputs/day4.txt")
        .map(|line| line.unwrap())
        .map(|line| card(&line.as_str()).unwrap().1)
        .map(|card| CardPrecalculated {
            card: card.clone(), calculated: card
            .my_numbers
            .iter()
            .filter(|number| card.winning_numbers.contains(number))
            .count()
        })
        .collect();

    let scratchcards = play_scratch_game(&lottery[..], &lottery[..]);
    dbg!(scratchcards);
}
