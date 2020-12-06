use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day1");
const TARGET: u32 = 2020;

pub fn run() -> (String, String) {
    let entries: Vec<u32> = INPUT.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let part1: u32 = find_combination(&entries, 2).iter().copied().product();
    let part2: u32 = find_combination(&entries, 3).iter().copied().product();
    (part1.to_string(), part2.to_string())
}

fn find_combination(entries: &[u32], len: usize) -> Vec<&u32> {
    entries
        .iter()
        .combinations(len)
        .find(|e| e.iter().copied().sum::<u32>() == TARGET)
        .unwrap()
}
