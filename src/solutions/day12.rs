const INPUT: &str = include_str!("../inputs/day12");

type Instruction = (char, isize);

pub fn run() -> (String, String) {
    let instructions: Vec<Instruction> = INPUT
        .lines()
        .map(|l| {
            let (command, amount) = l.split_at(1);
            let amount = amount.parse().unwrap();
            (command.chars().next().unwrap(), amount)
        })
        .collect();

    let part1 = part1(&instructions);
    let part2 = part2(&instructions);
    (part1.to_string(), part2.to_string())
}

fn part2(instructions: &[Instruction]) -> isize {
    let (mut waypoint_x, mut waypoint_y) = (10, 1);
    let (mut x, mut y) = (0, 0);

    for (command, amount) in instructions.iter() {
        match command {
            'N' => waypoint_y += amount,
            'S' => waypoint_y -= amount,
            'E' => waypoint_x += amount,
            'W' => waypoint_x -= amount,
            'L' => (waypoint_x, waypoint_y) = rotate(waypoint_x, waypoint_y, *amount),
            'R' => (waypoint_x, waypoint_y) = rotate(waypoint_x, waypoint_y, 360 - amount),
            'F' => {
                x += amount * waypoint_x;
                y += amount * waypoint_y;
            }
            _ => unreachable!(),
        }
    }
    x.abs() + y.abs()
}

fn rotate(x: isize, y: isize, angle: isize) -> (isize, isize) {
    match angle {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => unreachable!(),
    }
}

fn part1(instructions: &[Instruction]) -> isize {
    let (mut y, mut x, mut angle) = (0isize, 0isize, 90);
    for (command, amount) in instructions.iter() {
        match command {
            'N' => y += amount,
            'S' => y -= amount,
            'E' => x += amount,
            'W' => x -= amount,
            'L' => angle -= amount,
            'R' => angle += amount,
            'F' => match angle.rem_euclid(360) {
                0 => y += amount,
                90 => x += amount,
                180 => y -= amount,
                270 => x -= amount,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    x.abs() + y.abs()
}
