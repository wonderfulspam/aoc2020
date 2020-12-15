use ahash::AHashMap as HashMap;

const INPUT: [u32; 7] = [0, 1, 5, 10, 3, 12, 19];
const PART1: u32 = 2020;
const PART2: u32 = 30_000_000;
const BOUNDARY: u32 = PART2 / 20;

pub fn run() -> (String, String) {
    let part1 = solve_map(&INPUT, PART1);
    let part2 = solve_30m(&INPUT);
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

// Split bookkeeping into vec and hash, using a vec for the common (lower) numbers
fn solve_30m(input: &[u32]) -> u32 {
    let mut vec_history: [u32; BOUNDARY as usize] = [0; BOUNDARY as usize];
    input
        .iter()
        .enumerate()
        .for_each(|(i, &v)| vec_history[v as usize] = i as u32 + 1);
    let mut hash_history: HashMap<u32, u32> = HashMap::with_capacity(1024 * 256);

    (input.len() as u32..PART2).fold(*input.last().unwrap(), |last, index| {
        if last < BOUNDARY {
            let num = &mut vec_history[last as usize];
            let current = if *num == 0 { 0 } else { index - *num };
            *num = index;
            current
        } else {
            index - hash_history.insert(last, index).unwrap_or(index)
        }
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
