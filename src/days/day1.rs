use itertools::{self, Itertools};

fn get_digit(a: &str) -> Option<char> {
    if let Some(d) = a.chars().find(|c| c.is_digit(10)) {
        Some(d)
    } else {
        match a {
            _ if a.contains("zero") => Some('0'),
            _ if a.contains("one") => Some('1'),
            _ if a.contains("two") => Some('2'),
            _ if a.contains("three") => Some('3'),
            _ if a.contains("four") => Some('4'),
            _ if a.contains("five") => Some('5'),
            _ if a.contains("six") => Some('6'),
            _ if a.contains("seven") => Some('7'),
            _ if a.contains("eight") => Some('8'),
            _ if a.contains("nine") => Some('9'),
            _ => None,
        }
    }
}

fn solve_2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            let first = (0..l.len()).find_map(|s| get_digit(&l[..=s])).unwrap();
            let last = (0..l.len())
                .find_map(|s| get_digit(&l[(l.len() - (s + 1))..]))
                .unwrap();
            let s = format!("{first}{last}");
            s.parse::<usize>()
        })
        .sum()
}

fn solve_1(input: &str) -> usize {
    input
        .lines() // could also be \n\n
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect_vec())
        .map(|v| format!("{}{}", v.first().unwrap(), v.last().unwrap()))
        .flat_map(|s| s.parse::<usize>())
        .sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_1(input), solve_2(input))
}
