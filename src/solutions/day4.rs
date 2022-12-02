use ahash::AHashMap as HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

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
        "hgt" => check_hgt(v),
        "hcl" => check_hcl(v),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => check_pid(v),
        "cid" => true,
        _ => unreachable!(),
    })
}

fn check_hgt(v: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([0-9]+)(in|cm)$").unwrap();
    }
    if let Some(caps) = RE.captures(v) {
        let height = &caps[1].parse::<u32>().unwrap_or(0);
        let unit = &caps[2];
        match unit {
            "cm" => (150..=193).contains(height),
            "in" => (59..=76).contains(height),
            _ => false,
        }
    } else {
        false
    }
}
fn check_hcl(v: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#[a-f0-9]{6}$").unwrap();
    }
    RE.is_match(v)
}

fn check_pid(v: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }
    RE.is_match(v)
}

pub fn run() -> (String, String) {
    let valid_passports: Vec<HashMap<_, _>> = INPUT.split("\n\n").filter_map(parse).collect();
    let part1 = valid_passports.len();

    let actually_valid_passports = valid_passports.iter().filter(|&p| validate(p));
    let part2 = actually_valid_passports.count();

    (part1.to_string(), part2.to_string())
}
