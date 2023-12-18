use itertools::Itertools;

fn parse(input: &str) -> Vec<(char, isize, String)> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().split_ascii_whitespace())
        .map(|mut s| (s.next().unwrap(), s.next().unwrap(), s.collect::<String>()))
        .map(|(d, s, c)| {
            (
                d.chars().next().unwrap(),
                s.parse().unwrap(),
                c.replace("(#", "").replace(')', "").to_string(),
            )
        })
        .collect()
}

fn parse_poly(instructions: &[(char, isize)]) -> Vec<(isize, isize)> {
    let mut r = 0;
    let mut c = 0;
    instructions
        .iter()
        .map(move |(ch, s)| match ch {
            'R' => (r, c + s),
            'L' => (r, c - s),
            'U' => (r - s, c),
            'D' => (r + s, c),
            _ => unreachable!(),
        })
        .map(|(dr, dc)| {
            let p = (r + dr, c + dc);
            r += dr;
            c += dc;
            p
        })
        .collect_vec()
        .into_iter()
        .rev()
        .collect()
}

fn parse_hexa(hexa: &String) -> (char, isize) {
    let (hex_num, d) = hexa.split_at(5);
    let d = match d {
        "0" => 'R',
        "1" => 'D',
        "2" => 'L',
        "3" => 'U',
        c => unimplemented!("{}", c),
    };
    (d, isize::from_str_radix(hex_num, 16).unwrap())
}

fn polygon_area(polygon: &[(isize, isize)], perimeter: isize) -> isize {
    polygon[..polygon.len() - 1]
        .iter()
        .zip(polygon[1..].iter())
        .fold(0, |mut acc, (p1, p2)| {
            acc += (p1.0 + p2.0) * (p1.1 - p2.1); // trapezoid/shoelace formula
            acc
        })
        .abs()
        / 2
        // pick's theorem
        + perimeter / 2
        + 1
}

fn solve_1(input: &[(char, isize, String)]) -> usize {
    let poly = parse_poly(&input.iter().map(|(c, d, _)| (*c, *d)).collect_vec());
    polygon_area(&poly, input.iter().map(|(_, d, _)| d).sum()) as usize
}

fn solve_2(input: &[(char, isize, String)]) -> usize {
    let instructions = input.iter().map(|(_, _, s)| parse_hexa(s)).collect_vec();
    let poly = parse_poly(&instructions);
    polygon_area(&poly, instructions.iter().map(|(_, d)| d).sum()) as usize
}

pub fn solve(input: &str) -> (usize, usize) {
    let input = parse(input);
    (solve_1(&input), solve_2(&input))
}
