pub mod aoc_util;
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn day_modules() -> Vec<fn(&str) -> (usize, usize)> {
    vec![
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
        day19::solve,
        day20::solve,
        day21::solve,
        day22::solve,
        day23::solve,
        day24::solve,
        day25::solve,
    ]
}

#[allow(dead_code)]
pub fn solve_all(year: u32) {
    day_modules().iter().enumerate().for_each(|(i, solve)| {
        let day = (i + 1) as u32;
        let input = aoc_util::get_input(year, day, false);
        let solution = solve(input.trim());
        println!("Day{day}: {solution:?}")
    })
}

#[allow(dead_code)]
pub fn solve_day(year: u32, day: usize) {
    let input = aoc_util::get_input(year, day as u32, false);
    let solution = day_modules()[day - 1](input.trim());
    println!("Day{day}: {solution:?}")
}

#[allow(dead_code)]
pub fn solve_latest(year: u32) {
    let latest = day_modules().len();
    solve_day(year, latest)
}

// pub fn solve(input: &str) -> (usize, usize) {
//     println!("{input}");
//     (0,0)
// }
