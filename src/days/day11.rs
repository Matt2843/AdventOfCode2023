use itertools::Itertools;

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![' '; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn empty_r(grid: &Vec<Vec<char>>) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, l)| match l.iter().all(|&c| c == '.') {
            true => Some(i),
            _ => None,
        })
        .collect()
}

fn manhatten_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn solve_puzzle(grid: &Vec<Vec<char>>, expansion_factor: usize) -> usize {
    let empty_rows = empty_r(&grid);
    let empty_cols = empty_r(&transpose(&grid));
    grid.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter().enumerate().flat_map(move |(j, c)| match c {
                '#' => Some((i, j)),
                _ => None,
            })
        })
        .combinations(2)
        .map(|x| {
            let mut md = manhatten_distance(x[0], x[1]);
            empty_rows.iter().for_each(|er| {
                if x[0].0.min(x[1].0) < *er && x[0].0.max(x[1].0) > *er {
                    md += expansion_factor - 1
                }
            });
            empty_cols.iter().for_each(|ec| {
                if x[0].1.min(x[1].1) < *ec && x[0].1.max(x[1].1) > *ec {
                    md += expansion_factor - 1
                }
            });
            md
        })
        .sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    (solve_puzzle(&grid, 2), solve_puzzle(&grid, 1_000_000))
}
