use std::str::FromStr;

const INPUT: &str = include_str!("../inputs/day2");

struct PasswordLine {
    min: usize,
    max: usize,
    character: String,
    password: String,
}

impl FromStr for PasswordLine {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, password) = s.split_once(": ").unwrap();
        let (min_max, character) = first.split_once(" ").unwrap();
        let (min, max) = min_max.split_once("-").unwrap();
        Ok(PasswordLine {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
            character: character.to_string(),
            password: password.to_string()
        })
    }
}

pub fn run() {
    let entries: Vec<PasswordLine> = INPUT
        .lines()
        .map(|l| l.parse::<PasswordLine>().unwrap())
        .collect();
    let mut counter = 0;
    for entry in entries.iter() {
        let count = entry.password.matches(&entry.character).count();
        if count >= entry.min && count <= entry.max {
            counter += 1;
        }
    }
    println!("{}", counter);

    counter = 0;
    for entry in entries.iter() {
        let first_match = entry.password.chars().nth(entry.min - 1).unwrap().to_string() == entry.character;
        let second_match = entry.password.chars().nth(entry.max - 1).unwrap().to_string() == entry.character;
        if first_match ^ second_match {
            counter += 1;
        }
    }

    println!("{}", counter);
}
