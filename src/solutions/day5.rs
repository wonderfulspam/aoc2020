const INPUT: &str = include_str!("../inputs/day5");

pub fn run() {
    let max = seat_iter().max().unwrap();
    println!("{}", max);

    let mut seats = seat_iter().collect::<Vec<u32>>();
    seats.sort();
    let seat = seats
        .windows(2)
        // Find first pair where difference in seat ID isn't 1
        .find(|&seat_pair| seat_pair[0] + 1 != seat_pair[1])
        .unwrap();
    println!("{}", seat[0] + 1);
}

fn seat_iter() -> impl Iterator<Item = u32> {
    INPUT.lines().map(|l| {
        // Convert strings to u32 by bit shifting
        // No need to multiply by 8; adding the last three digits
        // will automatically take care of it
        l.chars().fold(0, |acc, c| match c {
            'F' | 'L' => (acc << 1) | 0,
            'B' | 'R' => (acc << 1) | 1,
            _ => unreachable!(),
        })
    })
}
