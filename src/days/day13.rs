fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|j| (0..grid.len()).map(|i| grid[i][j]).collect())
        .collect()
}

fn find_reflection(grid: &[Vec<char>], symmetric_error: usize) -> usize {
    match (0..grid[0].len() as i32 - 1)
        .find(|&c| {
            (0..=c).fold(0, |acc, dc| {
                let (left, right) = (c - dc, c + 1 + dc);
                if left < right && right < grid[0].len() as i32 {
                    acc + (0..grid.len())
                        .filter(|&r| grid[r][left as usize] != grid[r][right as usize])
                        .count()
                } else {
                    acc
                }
            }) == symmetric_error
        })
        .map(|c| c + 1)
    {
        Some(reflection) => reflection as usize,
        None => 100 * find_reflection(&transpose(grid), symmetric_error) as usize,
    }
}

fn solve_puzzle(grids: &[Vec<Vec<char>>], symmetric_error: usize) -> usize {
    grids
        .iter()
        .map(|t| find_reflection(t, symmetric_error))
        .sum()
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .trim()
        .split("\n\n")
        .map(|l| l.trim().lines().map(|l| l.chars().collect()).collect())
        .collect()
}

pub fn solve(input: &str) -> (usize, usize) {
    let grids = parse(input);
    (solve_puzzle(&grids, 0), solve_puzzle(&grids, 1))
}
