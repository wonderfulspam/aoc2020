use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day1");
const TARGET: u32 = 2020;

pub fn run() -> (String, String) {
    let entries: Vec<u32> = INPUT.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let (a, b) = entries
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == TARGET)
        .unwrap();
    let part1 = a * b;

    let (a, b, c) = entries
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == TARGET)
        .unwrap();
    let part2: u32 = a * b * c;
    (part1.to_string(), part2.to_string())
}
