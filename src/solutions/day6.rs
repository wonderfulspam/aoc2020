use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day6");

fn parse1(s: &str) -> usize {
    let mut chars: Vec<char> = s.split_whitespace().flat_map(|p| p.chars()).collect();
    chars.sort_unstable();
    chars.dedup();
    chars.len()
}

fn parse2(s: &str) -> usize {
    s.split_whitespace()
        .map(|l| l.chars().collect::<HashSet<char>>())
        // HashSet's BitAnd implementation returns the intersection
        .fold_first(|a, b| &a & &b)
        .unwrap()
        .len()
}

pub fn run() {
    let sum = INPUT.split("\n\n").map(|s| parse1(s)).sum::<usize>();
    println!("{}", sum);

    let sum = INPUT.split("\n\n").map(|s| parse2(s)).sum::<usize>();
    println!("{}", sum);
}
