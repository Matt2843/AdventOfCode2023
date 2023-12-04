use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Card {
    id: usize,
    win_nums: HashSet<i64>,
    drawn_nums: HashSet<i64>,
}

impl Card {
    fn new(input: &str, id: usize) -> Self {
        let (_, nums) = input.trim().split_once(':').unwrap();
        let (w, d) = nums.trim().split_once('|').unwrap();
        Card {
            id,
            win_nums: w.split_whitespace().flat_map(|wn| wn.parse()).collect(),
            drawn_nums: d.split_whitespace().flat_map(|wn| wn.parse()).collect(),
        }
    }

    fn score(&self) -> usize {
        let base: usize = 2;
        base.pow(self.matching_nums().saturating_sub(1).try_into().unwrap())
    }

    fn matching_nums(&self) -> usize {
        let matching = &self.win_nums & &self.drawn_nums;
        matching.len()
    }
}

fn solve_1(cards: &[Card]) -> usize {
    cards.iter().map(|c| c.score()).sum()
}

fn solve_2(cards: &[Card]) -> usize {
    let mut counts: HashMap<usize, usize> = cards.iter().map(|c| (c.id, 1)).collect();
    for c in cards.iter() {
        for j in c.id + 1..c.id + 1 + c.matching_nums() {
            let cc = counts[&c.id];
            counts.entry(j).and_modify(|c| *c += cc);
        }
    }
    counts.values().sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    let cards = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| Card::new(l, i))
        .collect_vec();
    (solve_1(&cards), solve_2(&cards))
}
