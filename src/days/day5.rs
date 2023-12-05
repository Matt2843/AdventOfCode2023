use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::HashMap,
    ops::Range,
    sync::atomic::{AtomicU8, AtomicUsize},
};

#[derive(Debug)]
struct GardenMap {
    src: String,
    dst: String,
    ranges: Vec<(Range<usize>, isize)>,
}

impl GardenMap {
    fn new(input: &str) -> Self {
        let mut iter = input.lines();
        let id = iter.next().unwrap();
        let (src, dst) = id.split_once(' ').unwrap().0.split_once("-to-").unwrap();
        let ranges = iter
            .flat_map(|l| {
                if let Some((dr, sr, le)) = l
                    .split_whitespace()
                    .flat_map(|s| s.parse::<usize>())
                    .collect_tuple()
                {
                    return Some((sr..sr + le, dr as isize - sr as isize));
                }
                None
            })
            .collect();
        GardenMap {
            src: src.trim().to_string(),
            dst: dst.trim().to_string(),
            ranges,
        }
    }

    fn eval(&self, inp: &usize) -> (usize, String) {
        if let Some((_, d)) = self.ranges.iter().find(|(r, _)| r.contains(inp)) {
            ((*inp as isize + d) as usize, self.dst.clone())
        } else {
            (*inp, self.dst.clone())
        }
    }
}

fn solve_1(seeds: &Vec<usize>, garden_lookups: &HashMap<String, GardenMap>) -> usize {
    seeds
        .iter()
        .map(|&s| {
            let mut x = s;
            let mut k = "seed".to_string();
            while let Some(v) = garden_lookups.get(&k) {
                (x, k) = v.eval(&x);
            }
            x
        })
        .min()
        .unwrap()
}

fn solve_2(seeds: &Vec<Range<usize>>, garden_lookups: &HashMap<String, GardenMap>) -> usize {
    let total = seeds.len();
    let c = AtomicU8::new(0);
    let c_min = AtomicUsize::new(usize::MAX);
    seeds
        .par_iter()
        .map(|s| {
            let huge = s.clone().collect_vec();
            let m = solve_1(&huge, garden_lookups);
            c.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            c_min.fetch_min(m, std::sync::atomic::Ordering::Relaxed);
            m
        })
        .inspect(|_| println!("Prog: {c:?}/{total} - current_min: {c_min:?}"))
        .min()
        .unwrap()
}

fn parse(input: &str) -> (Vec<usize>, HashMap<String, GardenMap>) {
    let mut iter = input.split("\n\n");
    let seeds = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|s| s.parse())
        .collect_vec();

    let maps = iter
        .map(|l| {
            let gm = GardenMap::new(l);
            (gm.src.clone(), gm)
        })
        .collect();
    (seeds, maps)
}

fn seed_ranges(input: &str) -> Vec<Range<usize>> {
    input
        .split("\n\n")
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .collect_vec()
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect_vec()
}

pub fn solve(input: &str) -> (usize, usize) {
    let (seeds, maps) = parse(input);
    (solve_1(&seeds, &maps), solve_2(&seed_ranges(input), &maps))
}
