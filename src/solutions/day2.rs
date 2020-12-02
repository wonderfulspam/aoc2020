use parse_display::FromStr;

const INPUT: &str = include_str!("../inputs/day2");

#[derive(FromStr)]
#[display("{min}-{max} {character}: {password}")]
struct PasswordLine {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

fn filter1(entry: &PasswordLine) -> bool {
    let matches = entry.password.matches(entry.character).count();
    matches >= entry.min && matches <= entry.max
}

fn filter2(entry: &PasswordLine) -> bool {
    let first_match = entry.password.chars().nth(entry.min - 1).unwrap() == entry.character;
    let second_match = entry.password.chars().nth(entry.max - 1).unwrap() == entry.character;
    first_match ^ second_match
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
