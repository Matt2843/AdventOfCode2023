use ahash::AHashSet;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<(isize, isize, isize)>> {
    input
        .trim()
        .lines()
        .flat_map(|l| l.trim().split_once('~'))
        .map(|(s, e)| {
            (
                s.split(',').flat_map(|n| n.parse::<isize>()).collect_vec(),
                e.split(',').flat_map(|n| n.parse::<isize>()).collect_vec(),
            )
        })
        .map(|(s, e)| {
            if s[0] != e[0] {
                (s[0]..=e[0]).map(|x| (x, s[1], s[2])).collect()
            } else if s[1] != e[1] {
                (s[1]..=e[1]).map(|y| (s[0], y, s[2])).collect()
            } else {
                (s[2]..=e[2]).map(|z| (s[0], s[1], z)).collect()
            }
        })
        .collect()
}

fn fall(snapshot: &Vec<Vec<(isize, isize, isize)>>) -> (Vec<Vec<(isize, isize, isize)>>, usize) {
    let mut all_bricks: AHashSet<(isize, isize, isize)> =
        snapshot.iter().flatten().copied().collect();
    let mut snapshot = snapshot.clone();
    let mut count = AHashSet::new();
    loop {
        let moved = snapshot.iter_mut().enumerate().any(|(i, bl)| {
            if bl.iter().any(|b| b.2 == 1) {
                false
            } else if bl.iter().any(|b| {
                all_bricks.contains(&(b.0, b.1, b.2 - 1)) && !bl.contains(&(b.0, b.1, b.2 - 1))
            }) {
                false
            } else {
                count.insert(i);
                bl.iter_mut().for_each(|b| {
                    all_bricks.remove(b);
                    all_bricks.insert((b.0, b.1, b.2 - 1));
                    b.2 -= 1;
                });
                true
            }
        });
        if !moved {
            break;
        }
    }
    (snapshot, count.len())
}

fn disintegrate(bricks: &Vec<Vec<(isize, isize, isize)>>) -> usize {
    bricks
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let mut removed = bricks.clone();
            removed.remove(*i);
            let (after, _) = fall(&removed);
            after == removed
        })
        .count()
}

fn chain_reaction(bricks: &Vec<Vec<(isize, isize, isize)>>) -> usize {
    bricks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut removed = bricks.clone();
            removed.remove(i);
            let (_, moved) = fall(&removed);
            moved
        })
        .sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    let snapshot = parse(input);
    let (bricks, _) = fall(&snapshot);
    (disintegrate(&bricks), chain_reaction(&bricks))
}
