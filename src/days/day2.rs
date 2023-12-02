use itertools::Itertools;
use std::collections::HashMap;

struct Game {
    id: usize,
    draws: HashMap<String, usize>,
}

impl Game {
    fn new(inp: &str) -> Self {
        let (g, ds) = inp.split_once(':').unwrap();
        let (_, g) = g.split_once(' ').unwrap();
        let draws = ds
            .trim()
            .split(';')
            .fold(HashMap::<String, usize>::new(), |mut acc, x| {
                _ = x
                    .trim()
                    .split(',')
                    .map(|d| {
                        let (q, c) = d.trim().split_once(' ').unwrap();
                        let q: usize = q.parse().unwrap();
                        acc.entry(c.to_string())
                            .and_modify(|eq| *eq = q.max(*eq))
                            .or_insert(q);
                    })
                    .collect_vec();
                acc
            });
        Game {
            id: g.parse().unwrap(),
            draws,
        }
    }

    fn power(&self) -> usize {
        self.draws.values().product()
    }

    fn is_valid(&self, bag: &HashMap<String, usize>) -> bool {
        !bag.iter().any(|(k, v)| {
            if let Some(iv) = self.draws.get(k) {
                iv > v
            } else {
                false
            }
        })
    }
}

fn solve_1(input: &str) -> usize {
    let bag: HashMap<_, _> = vec![
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .into_iter()
    .collect();
    input
        .lines()
        .map(Game::new)
        .filter(|g| g.is_valid(&bag))
        .fold(0, |acc, x| acc + x.id)
}

fn solve_2(input: &str) -> usize {
    input.lines().map(Game::new).map(|g| g.power()).sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_1(input), solve_2(input))
}
