use ahash::AHashSet;
use itertools::Itertools;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
struct CrucibleState {
    loc: (isize, isize),
    g: usize,
    d: Direction,
    dr: usize,
}

impl Ord for CrucibleState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.g.cmp(&self.g)
    }
}

impl PartialOrd for CrucibleState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CrucibleState {
    fn new(loc: (isize, isize), g: usize, d: Direction, dr: usize) -> Self {
        Self { loc, g, d, dr }
    }

    fn try_get(&self, grid: &[Vec<usize>], d: Direction) -> Option<(isize, isize)> {
        let nxt_pos = match d {
            Direction::North => (self.loc.0 - 1, self.loc.1),
            Direction::South => (self.loc.0 + 1, self.loc.1),
            Direction::West => (self.loc.0, self.loc.1 - 1),
            Direction::East => (self.loc.0, self.loc.1 + 1),
        };
        if (0..grid.len() as isize).contains(&nxt_pos.0)
            && (0..grid[0].len() as isize).contains(&nxt_pos.1)
        {
            Some(nxt_pos)
        } else {
            None
        }
    }

    fn successors(&self, grid: &[Vec<usize>], pt2: bool) -> Vec<CrucibleState> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
        .filter(|&nd| match (nd, self.d) {
            (Direction::South, Direction::North) => false,
            (Direction::North, Direction::South) => false,
            (Direction::West, Direction::East) => false,
            (Direction::East, Direction::West) => false,
            _ => true,
        })
        .filter(|&nd| {
            if pt2 {
                if nd != self.d {
                    self.dr > 3
                } else {
                    self.dr < 10
                }
            } else if nd == self.d {
                self.dr < 3
            } else {
                true
            }
        })
        .flat_map(|dr| {
            if dr == self.d {
                self.try_get(grid, dr).map(|l| CrucibleState::new(
                        l,
                        self.g + grid[l.0 as usize][l.1 as usize],
                        dr,
                        self.dr + 1,
                    ))
            } else {
                self.try_get(grid, dr).map(|l| CrucibleState::new(
                        l,
                        self.g + grid[l.0 as usize][l.1 as usize],
                        dr,
                        1,
                    ))
            }
        })
        .collect()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn dijkstra(
    grid: &[Vec<usize>],
    start: Vec<CrucibleState>,
    goal_pos: (isize, isize),
    pt2: bool,
) -> usize {
    let mut explored = AHashSet::new();
    let mut frontier = BinaryHeap::new();
    for st in start {
        frontier.push(st);
    }
    while let Some(s) = frontier.pop() {
        if explored.contains(&(s.loc, s.d, s.dr)) {
            continue;
        }
        if pt2 && s.loc == goal_pos && s.dr >= 4 {
            return s.g;
        } else if !pt2 && s.loc == goal_pos {
            return s.g;
        }
        for ss in s.successors(grid, pt2) {
            frontier.push(ss)
        }
        explored.insert((s.loc, s.d, s.dr));
    }
    panic!()
}

fn solve_puzzle(grid: &[Vec<usize>], pt2: bool) -> usize {
    dijkstra(
        grid,
        vec![
            CrucibleState::new((0, 0), 0, Direction::East, 0),
            CrucibleState::new((0, 0), 0, Direction::South, 0),
        ],
        (grid.len() as isize - 1, grid[0].len() as isize - 1),
        pt2,
    )
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| c.to_digit(10))
                .map(|d| d as usize)
                .collect()
        })
        .collect_vec();
    (solve_puzzle(&grid, false), solve_puzzle(&grid, true))
}
