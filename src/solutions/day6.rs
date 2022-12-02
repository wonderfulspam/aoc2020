use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day6");

fn parse1(s: &str) -> usize {
    s.split_whitespace()
        .flat_map(|p| p.chars())
        .collect::<HashSet<_>>()
        .len()
}

fn parse2(s: &str) -> usize {
    s.split_whitespace()
        .map(|l| l.chars().collect::<HashSet<_>>())
        // HashSet's BitAnd implementation returns the intersection
        .reduce(|a, b| &a & &b)
        .unwrap()
        .len()
}

pub fn run() -> (String, String) {
    let part1 = INPUT.split("\n\n").map(parse1).sum::<usize>();
    let part2 = INPUT.split("\n\n").map(parse2).sum::<usize>();

    (part1.to_string(), part2.to_string())
}
