const INPUT: &str = include_str!("../inputs/day1");
const TARGET: u32 = 2020;

pub fn run() {
    let entries: Vec<u32> = INPUT.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    'outer: for i in entries.iter() {
        for j in entries.iter() {
            if i + j == TARGET {
                println!("{}", i * j);
                break 'outer;
            }
        }
    }

    for i in entries.iter() {
        for j in entries.iter() {
            for k in entries.iter() {
                if i + j + k == TARGET {
                    println!("{}", i * j * k);
                    std::process::exit(0);
                }
            }
        }
    }
}
