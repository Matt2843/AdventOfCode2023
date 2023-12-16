use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

struct MirrorMap {
    grid: Vec<Vec<char>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Beam {
    row: usize,
    col: usize,
    dir: Dir,
}

impl Beam {
    fn new(row: usize, col: usize, d: Dir) -> Self {
        Self { row, col, dir: d }
    }

    fn from_beam(other: &Beam, dir: Dir) -> Self {
        Self {
            row: other.row,
            col: other.col,
            dir,
        }
    }
    fn split(&self, ch: &char, splits: &mut HashSet<(usize, usize)>) -> Vec<Beam> {
        if splits.contains(&self.pos()) {
            vec![]
        } else {
            splits.insert(self.pos());
            match (ch, self.dir) {
                ('-', Dir::North | Dir::South) => vec![
                    Beam::from_beam(self, Dir::West),
                    Beam::from_beam(self, Dir::East),
                ],
                ('-', Dir::West | Dir::East) => vec![self.clone()],
                ('|', Dir::West | Dir::East) => vec![
                    Beam::from_beam(self, Dir::North),
                    Beam::from_beam(self, Dir::South),
                ],
                ('|', Dir::North | Dir::South) => vec![self.clone()],
                _ => unreachable!(),
            }
        }
    }

    fn reflect(&self, ch: &char) -> Vec<Beam> {
        match ch {
            '/' => match self.dir {
                Dir::North => vec![Beam::from_beam(self, Dir::East)],
                Dir::West => vec![Beam::from_beam(self, Dir::South)],
                Dir::East => vec![Beam::from_beam(self, Dir::North)],
                Dir::South => vec![Beam::from_beam(self, Dir::West)],
            },
            '\\' => match self.dir {
                Dir::North => vec![Beam::from_beam(self, Dir::West)],
                Dir::West => vec![Beam::from_beam(self, Dir::North)],
                Dir::East => vec![Beam::from_beam(self, Dir::South)],
                Dir::South => vec![Beam::from_beam(self, Dir::East)],
            },
            _ => unreachable!(),
        }
    }

    fn shine(&self, mirror_map: &MirrorMap) -> Option<Beam> {
        let (new_row, new_col) = match self.dir {
            Dir::North => (self.row as isize - 1, self.col as isize),
            Dir::South => (self.row as isize + 1, self.col as isize),
            Dir::West => (self.row as isize, self.col as isize - 1),
            Dir::East => (self.row as isize, self.col as isize + 1),
        };
        if mirror_map.in_bounds(&(new_row, new_col)) {
            Some(Beam::new(new_row as usize, new_col as usize, self.dir))
        } else {
            None
        }
    }

    fn pos(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl MirrorMap {
    fn from_str(s: &str) -> Self {
        Self {
            grid: s
                .trim()
                .lines()
                .map(|l| l.trim().chars().collect())
                .collect(),
        }
    }

    fn in_bounds(&self, pos: &(isize, isize)) -> bool {
        (0..self.grid.len() as isize).contains(&pos.0)
            && (0..self.grid[0].len() as isize).contains(&pos.1)
    }

    fn get(&self, pos: &(usize, usize)) -> char {
        self.grid[pos.0][pos.1]
    }

    fn get_bounds(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn beam_effect(&self, beam: &Beam, splits: &mut HashSet<(usize, usize)>) -> Vec<Beam> {
        match self.get(&beam.pos()) {
            '.' => beam.shine(self).into_iter().collect(),
            ch if ['|', '-'].contains(&ch) => beam
                .split(&ch, splits)
                .into_iter()
                .flat_map(|b| b.shine(self))
                .collect(),
            ch if ['/', '\\'].contains(&ch) => beam
                .reflect(&ch)
                .into_iter()
                .flat_map(|b| b.shine(self))
                .collect(),
            _ => vec![],
        }
    }
}

fn run_beams(mirror_map: &MirrorMap, start: Beam) -> usize {
    let mut beams = vec![start];
    let mut energized = HashSet::new();
    let mut splits = HashSet::new();
    while !beams.is_empty() {
        let mut nxt_beams = Vec::new();
        for b in beams.iter() {
            energized.insert((b.row, b.col));
            let mut bb = mirror_map.beam_effect(b, &mut splits);
            nxt_beams.append(&mut bb);
        }
        beams = nxt_beams;
    }
    energized.len()
}

fn solve_1(mirror_map: &MirrorMap) -> usize {
    run_beams(mirror_map, Beam::new(0, 0, Dir::East))
}

fn solve_2(mirror_map: &MirrorMap) -> usize {
    let (rows, cols) = mirror_map.get_bounds();
    (0..rows)
        .chain(0..cols)
        .flat_map(|i| {
            vec![
                run_beams(mirror_map, Beam::new(i, 0, Dir::East)),
                run_beams(mirror_map, Beam::new(i, cols - 1, Dir::West)),
                run_beams(mirror_map, Beam::new(0, i, Dir::South)),
                run_beams(mirror_map, Beam::new(rows - 1, i, Dir::North)),
            ]
        })
        .max()
        .unwrap()
}

pub fn solve(input: &str) -> (usize, usize) {
    let mirror_map = MirrorMap::from_str(input);
    (solve_1(&mirror_map), solve_2(&mirror_map))
}
