use std::collections::HashMap;

fn parse(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn hash_algo(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|e| {
            e.chars()
                .map(|c| c as usize)
                .fold(0, |acc, x| ((acc + x) * 17).rem_euclid(256))
        })
        .sum()
}

fn solve_1(input: &[&str]) -> usize {
    input.iter().map(|e| hash_algo(e)).sum()
}

fn solve_2(input: &[&str]) -> usize {
    input
        .iter()
        .fold(
            HashMap::<usize, Vec<(&str, usize)>>::new(),
            |mut boxes, op| {
                if let Some((label, focal_length)) = op.split_once('=') {
                    let focal_length = focal_length.parse::<usize>().unwrap();
                    boxes
                        .entry(hash_algo(label))
                        .and_modify(|v| {
                            if let Some(idx) = v.iter().position(|(la, _)| la == &label) {
                                v[idx] = (label, focal_length)
                            } else {
                                v.push((label, focal_length))
                            }
                        })
                        .or_insert(vec![(label, focal_length)]);
                } else {
                    let label = op.replace('-', "");
                    boxes.entry(hash_algo(label.as_str())).and_modify(|v| {
                        if let Some(idx) = v.iter().position(|(la, _)| la == &label) {
                            v.remove(idx);
                        }
                    });
                }
                boxes
            },
        )
        .iter()
        .fold(0, |acc, (k, v)| {
            acc + v
                .iter()
                .enumerate()
                .map(|(i, (_, fl))| (1 + k) * (1 + i) * fl)
                .sum::<usize>()
        })
}

pub fn solve(input: &str) -> (usize, usize) {
    let input = parse(input);
    (solve_1(&input), solve_2(&input))
}
