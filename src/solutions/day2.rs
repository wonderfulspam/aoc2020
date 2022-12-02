use parse_display::FromStr;

const INPUT: &str = include_str!("../inputs/day2");

#[derive(FromStr)]
#[display("{lower}-{upper} {character}: {password}")]
struct PasswordLine {
    lower: usize,
    upper: usize,
    character: char,
    password: String,
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

pub fn run() -> (String, String) {
    let part1 = INPUT
        .lines()
        .map(|l| l.parse::<PasswordLine>().unwrap())
        .filter(filter1)
        .count();

    let part2 = INPUT
        .lines()
        .map(|l| l.parse::<PasswordLine>().unwrap())
        .filter(filter2)
        .count();
    (part1.to_string(), part2.to_string())
}
