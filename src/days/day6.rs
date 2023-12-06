use itertools::Itertools;
use regex::{self, Regex};

fn solve_puzzle(input: Vec<(usize, usize)>) -> usize {
    input
        .into_iter()
        .map(|(t, md)| (1..t).filter(move |&dt| (t - dt) * dt > md).count())
        .product()
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"\d+").unwrap();
    let w = input
        .lines()
        .map(|l| {
            re.find_iter(l)
                .flat_map(|m| m.as_str().parse())
                .collect_vec()
        })
        .collect_vec();
    w[0].clone().into_iter().zip(w[1].clone()).collect_vec()
}

fn parse_input_2(input: &str) -> (usize, usize) {
    input
        .lines()
        .flat_map(|l| l.split_once(':'))
        .map(|(_, n)| n.replace(' ', ""))
        .flat_map(|l| l.parse())
        .collect_tuple()
        .unwrap()
}

pub fn solve(input: &str) -> (usize, usize) {
    (
        solve_puzzle(parse_input(input)),
        solve_puzzle(vec![parse_input_2(input)]),
    )
}
