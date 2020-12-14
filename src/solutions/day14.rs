use ahash::AHashMap;

const INPUT: &str = include_str!("../inputs/day14");

#[derive(Debug)]
enum Operation {
    Mask(&'static [u8]),
    Mem(usize, usize),
}

pub fn run() -> (String, String) {
    let operations: Vec<Operation> = INPUT
        .lines()
        .map(|l| match l.split_once(" = ").unwrap() {
            ("mask", mask) => Operation::Mask(mask.as_bytes()),
            (mem, val) => {
                let loc = mem
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap();
                Operation::Mem(loc, val.parse().unwrap())
            }
        })
        .collect();

    let part1: usize = part1(&operations);
    let part2: usize = part2(&operations);

    (part1.to_string(), part2.to_string())
}

fn part2(operations: &[Operation]) -> usize {
    let mut memory: AHashMap<_, _> = AHashMap::with_capacity(70979);
    let mut mask: &[u8] = b"";

    for op in operations.iter() {
        match *op {
            Operation::Mask(new_mask) => mask = new_mask,
            Operation::Mem(loc, val) => write_to_memory(&mut memory, val, loc, mask, 0),
        }
    }

    memory.values().sum()
}

fn write_to_memory(
    memory: &mut AHashMap<usize, usize>,
    value: usize,
    location: usize,
    mask: &[u8],
    index: usize,
) {
    let bit = 1 << (35 - index);
    match mask.get(index) {
        Some(b'0') => write_to_memory(memory, value, location, mask, index + 1),
        Some(b'1') => write_to_memory(memory, value, location | bit, mask, index + 1),
        Some(b'X') => {
            write_to_memory(memory, value, location, mask, index + 1);
            write_to_memory(memory, value, location ^ bit, mask, index + 1);
        }
        _ => {
            memory.insert(location, value);
        }
    }
}

fn part1(operations: &[Operation]) -> usize {
    let mut memory = [0; 100000];
    let mut mask: &[u8] = b"";

    for op in operations.iter() {
        match op {
            Operation::Mask(new_mask) => mask = new_mask,
            Operation::Mem(loc, mut val) => {
                for (index, char) in mask.iter().rev().enumerate() {
                    match char {
                        b'0' => val &= !(1 << index),
                        b'1' => val |= 1 << index,
                        _ => {}
                    }
                }
                memory[*loc] = val;
            }
        }
    }
    memory.iter().sum()
}
