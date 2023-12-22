use ahash::AHashSet;
use itertools::Itertools;
use num::traits::Euclid;

fn neighbours(grid: &[Vec<char>], loc: (isize, isize)) -> Vec<(isize, isize)> {
    [(1, 0), (-1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dr, dc)| (loc.0 + dr, loc.1 + dc))
        .filter(|(r, c)| {
            let rr = r.rem_euclid(&(grid.len() as isize));
            let cc = c.rem_euclid(&(grid[0].len() as isize));
            grid[rr as usize][cc as usize] != '#'
        })
        .collect()
}

fn solve_puzzle(grid: &[Vec<char>], start: (isize, isize), steps_exact: usize) -> usize {
    let mut l_count = 0;
    let mut ll_count = 0;
    let mut lll_count = 0;
    let mut count = 1 - steps_exact % 2;
    let mut reached = 0;
    let mut cur_steps = 0;
    let mut incr = 0;
    let mut e = AHashSet::new();
    let mut q = AHashSet::new();
    q.insert(start);
    for i in 0..steps_exact {
        let x = q.clone();
        q = q
            .into_iter()
            .flat_map(|s| {
                neighbours(&grid, s)
                    .into_iter()
                    .filter(|n| !e.contains(n))
                    .map(|n| n)
                    .collect_vec()
            })
            .collect();
        e = x;
        if i % 2 != steps_exact % 2 {
            count += q.len()
        }
        if i % (grid.len() * 2) == steps_exact % grid.len() {
            if count - l_count - ll_count == lll_count {
                reached = i;
                incr = count - l_count - ll_count;
                cur_steps = count - l_count + incr;
                break;
            }
            lll_count = count - l_count - ll_count;
            ll_count = count - l_count;
            l_count = count;
        }
    }
    while reached < steps_exact {
        reached += 262;
        count += cur_steps;
        cur_steps += incr;
    }
    count
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, l)| {
            if let Some(c) = l.iter().position(|ch| ch == &'S') {
                Some((r as isize, c as isize))
            } else {
                None
            }
        })
        .unwrap();
    (
        solve_puzzle(&grid, start, 64),
        solve_puzzle(&grid, start, 26501365),
    )
}
