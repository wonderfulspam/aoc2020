use ahash::AHashMap as HashMap;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day4");
const MANDATORY_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse(s: &str) -> Option<HashMap<&str, &str>> {
    let map: HashMap<_, _> = s
        .split_whitespace()
        .flat_map(|p| p.split(':'))
        .tuples()
        .collect();
    if MANDATORY_KEYS.iter().all(|k| map.contains_key(k)) {
        return Some(map);
    }
    None
}

fn validate(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => (1920..=2002).contains(&v.parse().unwrap_or_default()),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap_or_default()),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap_or_default()),
        "hgt" => {
            let height = &v[0..v.len() - 2].parse().unwrap_or(0);
            let unit = &v[v.len() - 2..];
            match unit {
                "cm" => (150..=193).contains(height),
                "in" => (59..=76).contains(height),
                _ => false,
            }
        }
        "hcl" => {
            v.len() == 7
                && v.starts_with('#')
                && v[1..v.len()].chars().all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => v.len() == 9 && v.chars().all(char::is_numeric),
        "cid" => true,
        _ => unreachable!(),
    })
}

pub fn run() -> (String, String) {
    let valid_passports: Vec<HashMap<_, _>> = INPUT.split("\n\n").filter_map(parse).collect();
    let part1 = valid_passports.len();

    let actually_valid_passports = valid_passports.iter().filter(|&p| validate(p));
    let part2 = actually_valid_passports.count();

    (part1.to_string(), part2.to_string())
}
