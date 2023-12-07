use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Eq, Ord)]
struct Hand {
    cards: Vec<u8>,
    bet: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn from_str(s: &str, joker: bool) -> Self {
        let (cards, bet) = s.split_once(' ').unwrap();
        let cards = cards.chars().map(|ch| parse_ch(&ch, joker)).collect_vec();
        Hand {
            cards,
            bet: bet.parse().unwrap(),
        }
    }

    fn seq_highest(&self, other: &Self) -> std::cmp::Ordering {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(s, o)| match s.cmp(o) {
                Equal => None,
                Less => Some(Less),
                Greater => Some(Greater),
            })
            .unwrap()
    }

    fn get_type(&self) -> HandType {
        let cby = self.cards.iter().counts_by(|c| c);
        let jokers = cby.get(&1);
        match cby.len() {
            1 => HandType::FiveOfAKind,
            2 => match jokers {
                Some(_) => HandType::FiveOfAKind,
                _ => match cby.values().contains(&4) {
                    true => HandType::FourOfAKind,
                    false => HandType::FullHouse,
                },
            },
            3 => match jokers {
                Some(2) => HandType::FourOfAKind,
                Some(_) => match cby.values().contains(&3) {
                    true => HandType::FourOfAKind,
                    false => HandType::FullHouse,
                },
                None => match cby.values().contains(&3) {
                    true => HandType::ThreeOfAKind,
                    false => HandType::TwoPair,
                },
            },
            4 => match jokers {
                Some(_) => HandType::ThreeOfAKind,
                _ => HandType::OnePair,
            },
            5 => match jokers {
                Some(_) => HandType::OnePair,
                _ => HandType::HighCard,
            },
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_type().cmp(&other.get_type()) {
            Less => Some(Less),
            Greater => Some(Greater),
            Equal => Some(self.seq_highest(other)),
        }
    }
}

fn parse_ch(ch: &char, joker: bool) -> u8 {
    match ch {
        'T' => 10,
        'J' => match joker {
            true => 1,
            _ => 11,
        },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => ch.to_digit(10).unwrap() as u8,
    }
}

fn solve_puzzle(input: &str, joker: bool) -> usize {
    input
        .lines()
        .map(|l| Hand::from_str(l, joker))
        .sorted()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bet)
        .sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_puzzle(input, false), solve_puzzle(input, true))
}
