use core::panic;

fn get_val(ch: &char) -> u32 {
    match ch {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card value"),
    }
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;

    println!("{}", input);

    let mut games_values: Vec<(u32, Vec<u32>, u32)> = vec![];

    for game in input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(&str, u32)>>()
    {
        let mut cards = Vec::<(char, u32)>::new();

        for card in game.0.chars() {
            // if card isn't in cards add it else increment the value
            if cards.iter().any(|c| c.0 == card) {
                for c in cards.iter_mut() {
                    if c.0 == card {
                        c.1 += 1;
                    }
                }
            } else {
                cards.push((card, 1));
            }
        }

        if cards.len() == 1 {
            games_values.push((
                600,
                game.0.chars().map(|c| get_val(&c)).collect::<Vec<u32>>(),
                game.1,
            ));
        } else if cards.len() == 2 {
            if cards[0].1 == 4 {
                games_values.push((
                    500,
                    game.0.chars().map(|c| get_val(&c)).collect::<Vec<u32>>(),
                    game.1,
                ));
            } else {
                games_values.push((
                    400,
                    game.0.chars().map(|c| get_val(&c)).collect::<Vec<u32>>(),
                    game.1,
                ));
            }
        } else if cards.iter().any(|c| c.1 == 3) {
            games_values.push((
                300,
                game.0.chars().map(|c| get_val(&c)).collect::<Vec<u32>>(),
                game.1,
            ));
        } else if cards.iter().filter(|&c| c.1 == 2).count() == 2 {
            games_values.push((
                200,
                game.0
                    .chars()
                    .map(|card| get_val(&card))
                    .collect::<Vec<u32>>(),
                game.1,
            ));
        } else if cards.iter().any(|c| c.1 == 2) {
            games_values.push((
                100,
                game.0.chars().map(|c| get_val(&c)).collect::<Vec<u32>>(),
                game.1,
            ));
        } else {
            let mut sorted_cards = game.0.chars().map(|c| get_val(&c)).collect::<Vec<u32>>();

            sorted_cards.sort();

            games_values.push((
                *sorted_cards.last().unwrap(),
                game.0.chars().map(|c| get_val(&c)).collect::<Vec<u32>>(),
                game.1,
            ));
        }
    }

    games_values.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => {
            for (i, card) in a.1.iter().enumerate() {
                match card.cmp(&b.1[i]) {
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                }
            }

            std::cmp::Ordering::Equal
        }
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
    });

    println!("{:#?}", games_values);

    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

    println!("{:#?}", games_values.len());

    println!("{:#?}", games_values.last().unwrap());

    for (i, game) in games_values.iter().enumerate() {
        println!("{} * {} = {}", game.2, i + 1, game.2 * (i as u32 + 1));
        sum += game.2 * (i as u32 + 1);
    }

    sum.to_string()
}

///////////////////////////////////////////////////////////////////////////
//  https://github.com/gdavidmassey/AOC_2023/blob/master/src/day07_p1.rs //
///////////////////////////////////////////////////////////////////////////
// used because I need to check if the sulution is correct, too many submits
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: usize,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Score {
    High,
    Pair,
    TwoPair,
    Triple,
    FullHouse,
    Four,
    Five,
    None,
}

impl Score {
    fn cmp(&self, score: &Score) -> Ordering {
        if self > score {
            Ordering::Greater
        } else if self == score {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

fn card(c: char) -> Option<usize> {
    match c {
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        '8' => Some(7),
        '9' => Some(8),
        'T' => Some(9),
        'J' => Some(10),
        'Q' => Some(11),
        'K' => Some(12),
        'A' => Some(13),
        _ => None,
    }
}

fn score_hand(hand: &str) -> Score {
    let mut tally = HashMap::new();

    hand.chars().for_each(|x| match tally.get_mut(&x) {
        Some(v) => {
            *v += 1;
        }
        None => {
            tally.insert(x, 1);
        }
    });

    let mut tally: Vec<usize> = tally.values().copied().collect();

    tally.sort_by(|x, y| y.cmp(x));

    match tally[..] {
        [1, 1, 1, 1, 1] => Score::High,
        [2, 1, 1, 1] => Score::Pair,
        [3, 1, 1] => Score::Triple,
        [2, 2, 1] => Score::TwoPair,
        [3, 2] => Score::FullHouse,
        [4, 1] => Score::Four,
        [5] => Score::Five,
        _ => Score::None,
    }
}

fn cmp_high(h1: &str, h2: &str) -> Ordering {
    for (h1c, h2c) in h1.chars().zip(h2.chars()) {
        match card(h1c).cmp(&card(h2c)) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        }
    }
    Ordering::Equal
}

pub fn day07_p1_from_reddit(input_file: &str) -> String {
    let mut games: Vec<Hand> = input_file
        .lines()
        .map(|x| {
            let mut line = x.split_whitespace();
            Hand {
                cards: line.next().unwrap().to_owned(),
                bet: line.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    games.sort_by(
        |x, y| match score_hand(&x.cards).cmp(&score_hand(&y.cards)) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => cmp_high(&x.cards, &y.cards),
        },
    );

    let result = games
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i + 1) * v.bet);

    result.to_string()
}
