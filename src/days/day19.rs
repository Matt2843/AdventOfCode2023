use ahash::AHashMap;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug)]
struct Rule {
    cmp_tuple: Option<(usize, usize, Ordering)>,
    destination: String,
}

impl Rule {
    fn new(s: &str) -> Self {
        if let Some((left, dst)) = s.replace('}', "").trim().split_once(':') {
            let (xmas, num, ordering) = if let Some((xmas, num)) = left.split_once('<') {
                (xmas, num.parse().unwrap(), std::cmp::Ordering::Less)
            } else if let Some((xmas, num)) = left.split_once('>') {
                (xmas, num.parse().unwrap(), std::cmp::Ordering::Greater)
            } else {
                panic!();
            };

            let cmp_tuple = match xmas {
                "x" => Some((0, num, ordering)),
                "m" => Some((1, num, ordering)),
                "a" => Some((2, num, ordering)),
                "s" => Some((3, num, ordering)),
                _ => unreachable!(),
            };

            return Self {
                cmp_tuple,
                destination: dst.to_string(),
            };
        }

        Self {
            cmp_tuple: None,
            destination: s.replace('}', ""),
        }
    }
}

fn parse_rules(input: &str) -> AHashMap<String, Vec<Rule>> {
    input
        .trim()
        .lines()
        .flat_map(|l| l.trim().split_once('{'))
        .map(|(k, v)| (k.to_string(), v.split(',').map(Rule::new).collect()))
        .collect()
}

fn parse_xmas(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l[1..l.len() - 1]
                .split(',')
                .flat_map(|s| s[2..].parse::<usize>())
                .collect()
        })
        .collect()
}

fn filter_xmas(xmas: &Vec<usize>, rules: &AHashMap<String, Vec<Rule>>, key: &str) -> bool {
    match key {
        "A" => true,
        "R" => false,
        key => rules[key]
            .iter()
            .find_map(|r| match r.cmp_tuple {
                Some((xmas_key, num, cmp)) => {
                    if match cmp {
                        Ordering::Greater => xmas[xmas_key] > num,
                        Ordering::Less => xmas[xmas_key] < num,
                        _ => unreachable!(),
                    } {
                        Some(filter_xmas(xmas, rules, &r.destination))
                    } else {
                        None
                    }
                }
                None => Some(filter_xmas(xmas, rules, &r.destination)),
            })
            .unwrap(),
    }
}

fn solve_ranges(
    xmas: &mut Vec<std::ops::RangeInclusive<usize>>,
    rules: &AHashMap<String, Vec<Rule>>,
    key: &str,
) -> usize {
    match key {
        "A" => xmas.iter().flat_map(|r| r.try_len()).product(),
        "R" => 0,
        key => rules[key]
            .iter()
            .map(|r| {
                if let Some((xmas_key, num, cmp)) = r.cmp_tuple {
                    let (start, end) = (*xmas[xmas_key].start(), *xmas[xmas_key].end());
                    let (a_range, r_range) = match cmp {
                        Ordering::Greater => ((num + 1..=end), (start..=num)),
                        Ordering::Less => ((start..=num - 1), (num..=end)),
                        _ => unreachable!(),
                    };
                    xmas[xmas_key] = r_range;
                    let mut xmas = xmas.clone();
                    xmas[xmas_key] = a_range;
                    solve_ranges(&mut xmas, rules, &r.destination)
                } else {
                    solve_ranges(xmas, rules, &r.destination)
                }
            })
            .sum(),
    }
}

fn solve_1(rules: &AHashMap<String, Vec<Rule>>, xmas: &[Vec<usize>]) -> usize {
    xmas.iter()
        .filter(|x| filter_xmas(x, rules, &String::from("in")))
        .map(|x| x.iter().sum::<usize>())
        .sum()
}

fn solve_2(rules: &AHashMap<String, Vec<Rule>>) -> usize {
    let mut xmas = std::iter::repeat(1..=4000).take(4).collect_vec();
    solve_ranges(&mut xmas, rules, &String::from("in"))
}

pub fn solve(input: &str) -> (usize, usize) {
    let (rules, xmas) = input.trim().split_once("\n\n").unwrap();
    let rules = parse_rules(rules);
    let xmas = parse_xmas(xmas);
    (solve_1(&rules, &xmas), solve_2(&rules))
}
