use ahash::AHashMap as HashMap;

const INPUT: [u32; 7] = [0, 1, 5, 10, 3, 12, 19];

pub fn run() -> (String, String) {
    let part1 = solve_map(&INPUT, 2020);
    let part2 = solve_map(&INPUT, 30_000_000);
    (part1.to_string(), part2.to_string())
}

fn solve_map(input: &[u32], turns: u32) -> u32 {
    let mut history: HashMap<_, _> = input
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i as u32 + 1))
        .collect();
    (input.len() as u32..turns).fold(*input.last().unwrap(), |last, index| {
        index - history.insert(last, index).unwrap_or(index)
    })
}

#[allow(unused)]
fn solve_naive(input: &[usize], turns: usize) -> usize {
    let mut history = vec![0; turns];
    let starting_length = input.len();
    history.splice(..starting_length, input.iter().cloned());
    dbg!(&history[0..10]);
    (starting_length..turns).fold(0, |_, turn| {
        let previous = &history[turn - 1];
        let time_since_last = history
            .iter()
            .rev()
            .skip(turns - turn + 1)
            .enumerate()
            .find(|&(_, v)| v == previous)
            .map(|(index, _)| index + 1)
            .unwrap_or(0);
        history[turn] = time_since_last;
        time_since_last
    })
}
