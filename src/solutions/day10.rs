use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day10");

pub fn run() -> (String, String) {
    let mut adapters: Vec<_> = INPUT.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    adapters.sort_unstable();

    let part1 = part1(&adapters);
    let part2 = count_paths(&adapters);

    (part1.to_string(), part2.to_string())
}

fn part1(adapters: &[usize]) -> usize {
    let (one_diff, three_diff) =
        adapters
            .array_windows::<2>()
            .fold((1, 1), |(one_diff, three_diff), a| match a[1] - a[0] {
                1 => (one_diff + 1, three_diff),
                3 => (one_diff, three_diff + 1),
                _ => (one_diff, three_diff),
            });
    one_diff * three_diff
}

fn count_paths(adapters: &[usize]) -> usize {
    let mut map = HashMap::new();
    map.insert(0, 1);
    for &i in adapters {
        let ans = map.get(&(i - 1)).unwrap_or(&0)
            + map.get(&(i - 2)).unwrap_or(&0)
            + map.get(&(i - 3)).unwrap_or(&0);
        map.insert(i, ans);
    }
    map[adapters.last().unwrap()]
}
