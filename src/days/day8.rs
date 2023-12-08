use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let re = Regex::new(r"\w+").unwrap();
    let (instructions, map) = input.trim().split_once("\n\n").unwrap();
    let map = map.lines().fold(HashMap::new(), |mut acc, x| {
        let (key, val) = x.split_once(" = ").unwrap();
        let (left, right) = re
            .find_iter(val)
            .map(|m| m.as_str())
            .collect_tuple()
            .unwrap();
        acc.entry(key).or_insert((left, right));
        acc
    });
    (instructions.chars().collect(), map)
}

fn solve_1(i: &[char], m: &HashMap<&str, (&str, &str)>, start: &str, end: &str) -> usize {
    i.iter()
        .cycle()
        .fold_while((m[start], 1), |(n, t), d| {
            let next = match d {
                'L' => n.0,
                'R' => n.1,
                _ => unreachable!(),
            };
            if next.ends_with(end) {
                Done((m[next], t))
            } else {
                Continue((m[next], t + 1))
            }
        })
        .into_inner()
        .1
}

fn solve_2(i: &[char], m: &HashMap<&str, (&str, &str)>) -> usize {
    m.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| solve_1(i, m, k, "Z"))
        .reduce(lcm)
        .unwrap()
}

pub fn solve(input: &str) -> (usize, usize) {
    let (instructions, map) = parse(input);
    (
        solve_1(&instructions, &map, "AAA", "ZZZ"),
        solve_2(&instructions, &map),
    )
}
