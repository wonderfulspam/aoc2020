const INPUT: &str = include_str!("../inputs/day5");

pub fn run() -> (String, String) {
    let max = seat_iter().max().unwrap();

    let mut seats = seat_iter().collect::<Vec<u32>>();
    seats.sort_unstable();
    let seat = seats
        .windows(2)
        // Find first pair where difference in seat ID isn't 1
        .find(|&seat_pair| seat_pair[0] + 1 != seat_pair[1])
        .unwrap();
    let seat = seat[0] + 1;
    (max.to_string(), seat.to_string())
}

fn seat_iter() -> impl Iterator<Item = u32> {
    INPUT.lines().map(|l| {
        // Convert strings to u32 by bit shifting
        // No need to multiply by 8; adding the last three digits
        // will automatically take care of it
        l.chars().fold(0, |acc, c| match c {
            'F' | 'L' => (acc << 1),
            'B' | 'R' => (acc << 1) | 1,
            _ => unreachable!(),
        })
    })
}
