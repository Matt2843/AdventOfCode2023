use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

fn adjacent_symbols(grid: &Vec<Vec<char>>, x: &usize, y: &usize) -> HashSet<(usize, usize, char)> {
    iproduct!(-1..=1, -1..=1)
        .filter(|&d| d != (0, 0))
        .filter_map(|(dx, dy)| {
            let dxx = (*x as i32 + dx) as usize;
            let dyy = (*y as i32 + dy) as usize;
            if !(0..grid.len()).contains(&dxx) || !(0..grid[0].len()).contains(&dyy) {
                return None;
            } else {
                let ch = grid[dxx][dyy];
                if !ch.is_ascii_digit() && ch != '.' {
                    Some((dxx, dyy, ch))
                } else {
                    None
                }
            }
        })
        .collect()
}

fn solve_d(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut res = 0;
    let mut nums = HashMap::<(usize, usize), Vec<usize>>::new();
    for (i, l) in grid.iter().enumerate() {
        let mut add = false;
        let mut num = String::new();
        let mut gears = HashSet::new();
        for (j, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                num.push(*c);
                let symbols = adjacent_symbols(&grid, &i, &j);
                for (x, y, _) in symbols.iter().filter(|(_, _, c)| *c == '*') {
                    gears.insert((x.clone(), y.clone()));
                }
                if !symbols.is_empty() {
                    add = true;
                }
            } else if !num.is_empty() {
                if add {
                    let nval = num.parse::<usize>().unwrap();
                    res += nval;
                    for g in gears {
                        nums.entry(g)
                            .and_modify(|v| v.push(nval))
                            .or_insert(vec![nval]);
                    }
                }
                gears = HashSet::new();
                add = false;
                num = String::new();
            }
            if j == l.len() - 1 {
                if add {
                    let nval = num.parse::<usize>().unwrap();
                    res += num.parse::<usize>().unwrap();
                    for g in gears {
                        nums.entry(g)
                            .and_modify(|v| v.push(nval))
                            .or_insert(vec![nval]);
                    }
                }
                gears = HashSet::new();
                add = false;
                num = String::new();
            }
        }
    }
    let mut s2 = 0;
    for (_, v) in nums {
        if v.len() == 2 {
            s2 += v[0] * v[1]
        }
    }
    (res, s2)
}

pub fn solve(input: &str) -> (usize, usize) {
    let (s1, s2) = solve_d(input);
    (s1, s2)
}
