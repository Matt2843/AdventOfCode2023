use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<char>, Vec<usize>) {
    let (seq, rec) = input.trim().split_once(' ').unwrap();
    (
        seq.chars().collect(),
        rec.split(',').flat_map(|s| s.parse()).collect(),
    )
}

fn unfold(seq: &[char], rec: &[usize]) -> (Vec<char>, Vec<usize>) {
    (
        std::iter::repeat(seq.iter().collect::<String>())
            .take(5)
            .join("?")
            .chars()
            .collect(),
        rec.iter().cycle().take(5 * rec.len()).copied().collect(),
    )
}

fn count_c(seq: &[char], ch: char) -> usize {
    seq.iter().filter(|&&c| c == ch).count()
}

fn arrangements_brute(seq: &[char], rec: &[usize]) -> usize {
    let unknowns = seq.iter().filter(|c| **c == '?').count();
    (0..unknowns)
        .map(|_| vec!['.', '#'])
        .multi_cartesian_product()
        .filter(|mcp| count_c(mcp, '#') + count_c(seq, '#') == rec.iter().sum())
        .map(|mcp| {
            let mut it = mcp.iter();
            seq.iter()
                .map(|&c| if c == '?' { *it.next().unwrap() } else { c })
                .collect_vec()
        })
        .filter(|mcp| {
            mcp.iter()
                .group_by(|&&c| c == '#')
                .into_iter()
                .filter(|(s, _)| *s)
                .map(|(_, cs)| cs.count())
                .collect_vec()
                == *rec
        })
        .count()
}

fn arrangements_memo(
    seq: &[char],
    rec: &[usize],
    seqi: usize,
    reci: usize,
    hash_count: usize,
    memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let memo_key = (seqi, reci, hash_count);
    if let Some(v) = memo.get(&memo_key) {
        return *v;
    }
    // stop condition
    if seqi == seq.len() {
        return if reci == rec.len() && hash_count == 0 {
            1
        } else if reci == rec.len() - 1 && rec[reci] == hash_count {
            1
        } else {
            0
        };
    }
    let sub_arr = vec!['#', '.']
        .into_iter()
        .filter(|&c| seq[seqi] == c || seq[seqi] == '?')
        .fold(0, |acc, c| match c {
            '.' => {
                if hash_count == 0 {
                    acc + arrangements_memo(seq, rec, seqi + 1, reci, 0, memo)
                } else if hash_count > 0 && reci < rec.len() && rec[reci] == hash_count {
                    acc + arrangements_memo(seq, rec, seqi + 1, reci + 1, 0, memo)
                } else {
                    acc
                }
            }
            '#' => acc + arrangements_memo(seq, rec, seqi + 1, reci, hash_count + 1, memo),
            _ => unreachable!(),
        });
    memo.insert(memo_key, sub_arr);
    sub_arr
}

fn solve_1(input: &[(Vec<char>, Vec<usize>)]) -> usize {
    input
        .iter()
        .map(|(seq, rec)| arrangements_brute(seq, rec))
        .sum()
}

fn solve_2(input: &[(Vec<char>, Vec<usize>)]) -> usize {
    input
        .iter()
        .map(|(seq, rec)| unfold(seq, rec))
        .map(|(seq, rec)| {
            let mut cache = HashMap::new();
            arrangements_memo(&seq, &rec, 0, 0, 0, &mut cache)
        })
        .sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    let parsed = input.trim().lines().map(|l| parse(l)).collect_vec();
    (solve_1(&parsed), solve_2(&parsed))
}
