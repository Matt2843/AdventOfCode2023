use itertools::Itertools;
use num::traits::Euclid;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn include(grid: &Vec<Vec<char>>, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    let (x, y) = match dir {
        Direction::North => (pos.0.saturating_sub(1), pos.1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::West => (pos.0, pos.1.saturating_sub(1)),
    };
    let in_bounds = (0..grid.len()).contains(&x) && (0..grid[0].len()).contains(&y);
    if !in_bounds {
        return None;
    }
    let ch = grid[x][y];
    match dir {
        Direction::North => {
            if vec!['|', 'F', '7'].contains(&ch) {
                Some((x, y))
            } else {
                None
            }
        }
        Direction::South => {
            if vec!['|', 'J', 'L'].contains(&ch) {
                Some((x, y))
            } else {
                None
            }
        }
        Direction::West => {
            if vec!['-', 'F', 'L'].contains(&ch) {
                Some((x, y))
            } else {
                None
            }
        }
        Direction::East => {
            if vec!['-', 'J', '7'].contains(&ch) {
                Some((x, y))
            } else {
                None
            }
        }
    }
}

fn neighbours(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut nes = Vec::new();
    match grid[pos.0][pos.1] {
        'F' | 'S' => {
            nes.push(include(grid, pos, Direction::South));
            nes.push(include(grid, pos, Direction::East));
        }
        '7' => {
            nes.push(include(grid, pos, Direction::West));
            nes.push(include(grid, pos, Direction::South));
        }
        'J' => {
            nes.push(include(grid, pos, Direction::North));
            nes.push(include(grid, pos, Direction::West));
        }
        'L' => {
            nes.push(include(grid, pos, Direction::North));
            nes.push(include(grid, pos, Direction::East));
        }
        '|' => {
            nes.push(include(grid, pos, Direction::North));
            nes.push(include(grid, pos, Direction::South));
        }
        '-' => {
            nes.push(include(grid, pos, Direction::West));
            nes.push(include(grid, pos, Direction::East));
        }
        _ => unreachable!(),
    }
    nes.into_iter().flatten().collect_vec()
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize)) -> (HashSet<(usize, usize)>, usize) {
    let mut explored = HashSet::new();
    let mut frontier = VecDeque::new();
    frontier.push_back((start, 0));
    let mut max_g = 0;
    while let Some((pos, g)) = frontier.pop_front() {
        if explored.contains(&pos) {
            continue;
        }
        max_g = max_g.max(g);
        for n in neighbours(grid, pos) {
            frontier.push_back((n, g + 1))
        }
        explored.insert(pos);
    }
    (explored, max_g)
}

fn solve_2(grid: &Vec<Vec<char>>, explored: &HashSet<(usize, usize)>) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(i, l)| l.iter().enumerate().map(move |(j, _)| (i, j)))
        .filter(|&p| !explored.contains(&p))
        .filter(|&(i, j)| {
            (0..j)
                .fold(0, |acc, d| {
                    if explored.contains(&(i, d)) && vec!['|', 'J', 'L'].contains(&grid[i][d]) {
                        acc + 1
                    } else {
                        acc
                    }
                })
                .rem_euclid(&2)
                == 1
        })
        .count()
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, l)| match l.iter().position(|c| *c == 'S') {
            Some(j) => Some((i, j)),
            _ => None,
        })
        .unwrap();
    let (explored, s1) = bfs(&grid, start);
    (s1, solve_2(&grid, &explored))
}
