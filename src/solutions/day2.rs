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

fn filter1(entry: &PasswordLine) -> bool {
    let matches = entry.password.matches(entry.character).count();
    matches >= entry.lower && matches <= entry.upper
}

fn filter2(entry: &PasswordLine) -> bool {
    let password_bytes = entry.password.as_bytes();
    let character_byte = entry.character as u8;
    (password_bytes[entry.lower - 1] == character_byte) ^ (password_bytes[entry.upper - 1] == character_byte)
}

pub fn run() {
    let count = INPUT
        .lines()
        .map(|l| l.parse::<PasswordLine>().unwrap())
        .filter(|entry| filter1(entry)).count();
    println!("{}", count);

    let count = INPUT
        .lines()
        .map(|l| l.parse::<PasswordLine>().unwrap())
        .filter(|entry| filter2(entry)).count();
    println!("{}", count);

}
