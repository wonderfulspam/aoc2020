const INPUT: &str = include_str!("../inputs/day2");

struct PasswordLine<'a> {
    lower: usize,
    upper: usize,
    character: char,
    password: &'a str,
}

fn filter1(p: &PasswordLine) -> bool {
    let matches = p.password.matches(p.character).count();
    matches >= p.lower && matches <= p.upper
}

fn filter2(p: &PasswordLine) -> bool {
    let password_bytes = p.password.as_bytes();
    let char_byte = p.character as u8;
    (password_bytes[p.lower - 1] == char_byte) ^ (password_bytes[p.upper - 1] == char_byte)
}

fn parse(s: &str) -> PasswordLine {
    let (first, password) = s.split_once(": ").unwrap();
    let (first, character) = first.split_once(' ').unwrap();
    let (lower, upper) = first.split_once('-').unwrap();
    PasswordLine {
        lower: lower.parse().unwrap(),
        upper: upper.parse().unwrap(),
        character: character.chars().next().unwrap(),
        password,
    }
}

pub fn run() -> (String, String) {
    let password_lines: Vec<PasswordLine> = INPUT.lines().map(parse).collect();
    let part1 = password_lines.iter().filter(|p| filter1(p)).count();
    let part2 = password_lines.iter().filter(|p| filter2(p)).count();
    (part1.to_string(), part2.to_string())
}
