use std::cmp::Ordering;

const INPUT: &str = include_str!("../inputs/day1");
const TARGET: u32 = 2020;

pub fn run() -> (String, String) {
    let mut entries: Vec<u32> = INPUT.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    entries.sort_unstable();

    let part1 = part1(&entries);
    let part2 = part2(&entries);
    (part1.to_string(), part2.to_string())
}

fn part1(entries: &[u32]) -> u32 {
    for (i_index, &i) in entries.iter().enumerate() {
        for &j in entries.iter().skip(i_index + 1) {
            let sum = i + j;
            match sum.cmp(&TARGET) {
                Ordering::Less => {}
                Ordering::Equal => return i * j,
                Ordering::Greater => break,
            }
        }
    }
    unreachable!()
}

fn part2(entries: &[u32]) -> u32 {
    for (i_index, &i) in entries.iter().enumerate() {
        for (j_index, &j) in entries.iter().skip(i_index + 1).enumerate() {
            for &k in entries.iter().skip(j_index) {
                let sum = i + j + k;
                match sum.cmp(&TARGET) {
                    Ordering::Less => {}
                    Ordering::Equal => return i * j * k,
                    Ordering::Greater => break,
                }
            }
        }
    }
    unreachable!()
}
