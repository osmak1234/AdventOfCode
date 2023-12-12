use std::cmp::Ordering;

fn card_value(card: char) -> u32 {
    match card {
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

fn compare_cards_in_hands(hand1: Vec<char>, hand2: Vec<char>) -> Ordering {
    assert_eq!(hand1.len(), 5);
    assert_eq!(hand2.len(), 5);

    for i in 0..hand1.len() {
        let val1 = card_value(hand1[i]);
        let val2 = card_value(hand2[i]);

        match val1.cmp(&val2) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => continue,
        }
    }

    Ordering::Equal
}

struct Game {
    hand: Vec<char>,
    bet: u32,
}

fn analyse_hand(hand: Vec<char>) -> HandType {
    // cound how many of each card there are
    let mut card_counts = [0; 15];
    for card in hand.iter() {
        card_counts[card_value(*card) as usize] += 1;
    }

    // get the hand type

    // five of a kind
    if card_counts.contains(&5) {
        return HandType::FiveOfAKind;
    }

    // four of a kind
    if card_counts.contains(&4) {
        return HandType::FourOfAKind;
    }

    // full FullHouse
    if card_counts.contains(&3) && card_counts.contains(&2) {
        return HandType::FullHouse;
    }

    // three of a kind
    if card_counts.contains(&3) {
        return HandType::ThreeOfAKind;
    }

    // two TwoPair
    if card_counts.iter().filter(|&&x| x == 2).count() == 2 {
        return HandType::TwoPair;
    }

    // one OnePair
    if card_counts.contains(&2) {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn analyse_games(games: &[Game]) -> Vec<AnalysedGame> {
    games
        .iter()
        .map(|game| {
            let hand_type = analyse_hand(game.hand.clone());
            AnalysedGame {
                hand: game.hand.clone(),
                bet: game.bet,
                hand_type,
            }
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let hand = split.next().unwrap().chars().collect();
            let bet = split.next().unwrap().parse::<u32>().unwrap();
            Game { hand, bet }
        })
        .collect()
}

fn sort_games(games: Vec<AnalysedGame>) -> Vec<AnalysedGame> {
    let mut games = games;
    games.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            compare_cards_in_hands(a.hand.clone(), b.hand.clone())
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });
    games
}

#[derive(Debug)]
struct AnalysedGame {
    hand: Vec<char>,
    bet: u32,
    hand_type: HandType,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn part1(input: &str) -> String {
    let games = parse_input(input);
    let analysed_games = analyse_games(&games);
    let sorted_games = sort_games(analysed_games);

    println!("{:#?}", sorted_games);

    sorted_games
        .iter()
        .rev()
        .enumerate()
        .map(|(i, game)| game.bet * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}
