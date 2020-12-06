/// Largely stolen from https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/06.rs
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day6");

fn parse1(s: &str) -> usize {
    s.split_whitespace()
        .flat_map(|p| p.chars())
        .unique()
        .count()
}

fn parse2(s: &str) -> u32 {
    s.split_whitespace()
        .map(|part| part.bytes().fold(0u32, |x, b| x | 1 << (b - b'a')))
        .fold(!0, |acc, x| acc & x)
        .count_ones()
}

pub fn run() -> (String, String) {
    let part1 = INPUT.split("\n\n").map(|s| parse1(s)).sum::<usize>();
    let part2 = INPUT.split("\n\n").map(|s| parse2(s)).sum::<u32>();

    (part1.to_string(), part2.to_string())
}
