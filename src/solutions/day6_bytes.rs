/// Largely stolen from https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/06.rs
/// and https://www.reddit.com/r/rust/comments/k7nq0h/advent_of_code_2020_day_6/getewqo/

const INPUT: &str = include_str!("../inputs/day6");

fn parse1(s: &str) -> u32 {
    s.bytes()
        .filter(|&c| c != b'\n')
        .fold(0u32, |acc, answer| acc | 1 << (answer - b'a'))
        .count_ones()
}

fn parse2(s: &str) -> u32 {
    s.lines()
        .map(|part| {
            part.bytes()
                .fold(0u32, |acc, answer| acc | 1 << (answer - b'a'))
        })
        .fold(!0, |acc, x| acc & x)
        .count_ones()
}

pub fn run() -> (String, String) {
    let part1 = INPUT.split("\n\n").map(|s| parse1(s)).sum::<u32>();
    let part2 = INPUT.split("\n\n").map(|s| parse2(s)).sum::<u32>();

    (part1.to_string(), part2.to_string())
}
