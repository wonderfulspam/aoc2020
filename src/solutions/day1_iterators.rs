const INPUT: &str = include_str!("../inputs/day1");
const TARGET: u32 = 2020;

pub fn run() -> (String, String) {
    let entries: Vec<u32> = INPUT.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let part1 = part1(&entries);
    let part2 = part2(&entries);
    (part1.to_string(), part2.to_string())
}

fn part1(entries: &[u32]) -> u32 {
    for i in entries.iter() {
        for j in entries.iter() {
            if i + j == TARGET {
                return i * j;
            }
        }
    }
    unreachable!()
}

fn part2(entries: &[u32]) -> u32 {
    for (i_index, i) in entries.iter().enumerate() {
        for j_index in i_index..entries.len() {
            let j = entries[j_index];
            for k in entries.iter().skip(j_index) {
                if i + j + k == TARGET {
                    return i * j * k;
                }
            }
        }
    }
    unreachable!()
}
