use ahash::AHashMap;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Module {
    dst: Vec<String>,
    t: ModuleType,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(AHashMap<String, bool>),
}

impl Module {
    fn new(s: &str) -> (String, Self) {
        let (id, dst) = s.trim().split_once(" -> ").unwrap();
        let dst = dst.split(", ").map(|s| s.to_owned()).collect();
        let (m, mid) = id.split_at(1);
        match m {
            "%" => (
                mid.to_string(),
                Self {
                    dst,
                    t: ModuleType::FlipFlop(false),
                },
            ),
            "&" => (
                mid.to_string(),
                Self {
                    dst,
                    t: ModuleType::Conjunction(AHashMap::new()),
                },
            ),
            "b" => (
                id.to_string(),
                Self {
                    dst,
                    t: ModuleType::Broadcaster,
                },
            ),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> AHashMap<String, Module> {
    let mut v = input.trim().lines().map(Module::new).collect_vec();
    let v_clone = v.clone();
    v.iter_mut().for_each(|(k, m)| match m.t {
        ModuleType::Conjunction(ref mut mem) => {
            *mem = v_clone
                .iter()
                .flat_map(|(ik, im)| match im.dst.contains(&k.to_string()) {
                    true => Some((ik.to_string(), false)),
                    false => None,
                })
                .collect()
        }
        _ => {}
    });
    v.into_iter().collect()
}

fn press_button(
    modules: &mut AHashMap<String, Module>,
    track_conj: &String,
) -> (usize, usize, bool) {
    let mut high = 0;
    let mut low = 0;
    let mut queue = VecDeque::new();
    let mut conj_triggered = false;
    queue.push_back((String::from("broadcaster"), String::from("button"), false));
    while let Some((t, f, p)) = queue.pop_front() {
        if p {
            high += 1
        } else {
            low += 1
        }
        modules.entry(t.clone()).and_modify(|m| match m.t {
            ModuleType::Broadcaster => m
                .dst
                .iter()
                .for_each(|d| queue.push_back((d.clone(), t.clone(), p))),
            ModuleType::FlipFlop(ref mut s) => {
                if !p {
                    *s = !*s;
                    m.dst
                        .iter()
                        .for_each(|d| queue.push_back((d.clone(), t.clone(), *s)))
                }
            }
            ModuleType::Conjunction(ref mut mem) => {
                mem.entry(f.clone()).and_modify(|v| *v = p);
                if p && &t == track_conj {
                    conj_triggered = true;
                }
                match mem.values().all(|v| *v) {
                    true => m
                        .dst
                        .iter()
                        .for_each(|d| queue.push_back((d.clone(), t.clone(), false))),
                    false => m
                        .dst
                        .iter()
                        .for_each(|d| queue.push_back((d.clone(), t.clone(), true))),
                }
            }
        });
    }
    (high, low, conj_triggered)
}

fn solve_1(input: &str) -> usize {
    let mut modules = parse(input);
    let (high, low, _) = (0..1000)
        .map(|_| press_button(&mut modules, &String::from("")))
        .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2))
        .unwrap();
    high * low
}

fn solve_2(input: &str) -> usize {
    let mut modules = parse(input);
    let mut button_presses: usize = 0;
    let modules_clone = modules.clone();
    let (rx_ckey, _) = modules_clone
        .iter()
        .find(|(_, m)| m.dst.contains(&String::from("rx")))
        .unwrap();
    let conjunction_input_count = modules
        .iter()
        .filter(|(_, m)| m.dst.contains(rx_ckey))
        .count();
    let mut cycles = Vec::new();
    loop {
        button_presses += 1;
        let (_, _, triggered) = press_button(&mut modules, rx_ckey);
        if triggered {
            cycles.push(button_presses)
        }
        if cycles.len() == conjunction_input_count {
            break;
        }
    }
    cycles.into_iter().reduce(num::integer::lcm).unwrap() as usize
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_1(input), solve_2(input))
}
