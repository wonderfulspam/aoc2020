use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day1");
const TARGET: u32 = 2020;

pub fn run() {
    let entries: Vec<u32> = INPUT.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let result: u32 = find_combination(&entries, 2).iter().map(|&i| i).product();
    println!("{}", result);
    let result: u32 = find_combination(&entries, 3).iter().map(|&i| i).product();
    println!("{}", result);
}

fn find_combination(entries: &[u32], len: usize) -> Vec<&u32> {
    entries
        .iter()
        .combinations(len)
        .find(|e| e.iter().map(|&i| i).sum::<u32>() == TARGET)
        .unwrap()
}
