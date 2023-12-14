fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn ij_vacant(grid: &[Vec<char>], i: i32, j: i32) -> bool {
    if i < 0 || j < 0 {
        return false;
    }
    (0..grid.len() as i32).contains(&i)
        && (0..grid[0].len() as i32).contains(&j)
        && grid[i as usize][j as usize] == '.'
}

fn calc_north_load(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, l)| l.iter().filter(|ch| **ch == 'O').count() * (grid.len() - i))
        .sum()
}

fn find_max_cycle(arr: &[usize], min: usize) -> Option<&[usize]> {
    (0..arr.len()).find_map(|i| {
        (i + min..arr.len())
            .rev()
            .find_map(|j| match arr[j..].starts_with(&arr[i..j]) {
                true => Some(&arr[i..j]),
                false => None,
            })
    })
}

fn next_grid(grid: &mut Vec<Vec<char>>, dirs: &Vec<(i32, i32)>) {
    dirs.iter().for_each(|dir| loop {
        let mut nxt_grid = grid.clone();
        let moved = grid
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.iter()
                    .enumerate()
                    .map(move |(j, c)| (i, j, i as i32 + dir.0, j as i32 + dir.1, c))
            })
            .filter(|(_, _, di, dj, c)| **c == 'O' && ij_vacant(grid, *di, *dj))
            .map(|(i, j, di, dj, _)| {
                nxt_grid[di as usize][dj as usize] = 'O';
                nxt_grid[i][j] = '.';
            })
            .count();
        *grid = nxt_grid;
        if moved == 0 {
            break;
        }
    });
}

fn solve_1(grid: &mut Vec<Vec<char>>) -> usize {
    next_grid(grid, &vec![(-1, 0)]);
    calc_north_load(grid)
}

fn solve_2(grid: &mut Vec<Vec<char>>) -> usize {
    let mut i = 0;
    let n = 1_000_000_000;
    let dirs = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut cycle_vec = Vec::new();
    while i < n {
        i += 1;
        next_grid(grid, &dirs);
        cycle_vec.push(calc_north_load(&grid));
        if let Some(cycle) = find_max_cycle(&cycle_vec, 5) {
            let x = (n - i) / cycle.len();
            i += x * cycle.len();
        }
    }
    calc_north_load(grid)
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut grid = parse(input);
    (solve_1(&mut grid), solve_2(&mut grid))
}
