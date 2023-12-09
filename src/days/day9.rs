use itertools::{unfold, Itertools};

fn parse(input: &str) -> Vec<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|l| l.split_ascii_whitespace())
        .map(|l| l.flat_map(|s| s.parse::<i64>()).collect_vec())
        .map(|iv| {
            std::iter::once(iv.clone())
                .chain(unfold(iv.clone(), |niv| {
                    let nxt = niv.windows(2).map(|w| w[1] - w[0]).collect_vec();
                    if nxt.iter().all(|&x| x == 0) {
                        None
                    } else {
                        *niv = nxt.clone();
                        Some(nxt)
                    }
                }))
                .collect_vec()
        })
        .collect_vec()
}

fn solve_2(v: &[Vec<Vec<i64>>]) -> usize {
    v.iter()
        .map(|n| n.iter().rev().fold(0, |acc, x| x.first().unwrap() - acc))
        .sum::<i64>() as usize
}

fn solve_1(v: &[Vec<Vec<i64>>]) -> usize {
    v.iter()
        .map(|n| n.iter().rev().fold(0, |acc, x| x.last().unwrap() + acc))
        .sum::<i64>() as usize
}

pub fn solve(input: &str) -> (usize, usize) {
    let v = parse(input);
    (solve_1(&v), solve_2(&v))
}
