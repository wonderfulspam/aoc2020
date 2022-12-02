use ahash::AHashMap as HashMap;

const INPUT: &str = include_str!("../inputs/day7");

fn parse(line: &str) -> (&str, HashMap<&str, u32>) {
    let (color, rest) = line.split_once(" bags contain ").unwrap();

    let mut contents: HashMap<&str, u32> = HashMap::new();
    if !rest.contains("no other bags") {
        for rule in rest[..rest.len() - 1].split(", ") {
            let (num, color) = rule.split_once(' ').unwrap();
            let (color, _) = color.split_once(" bag").unwrap();
            let count = num.parse::<u32>().unwrap();

            contents.insert(color, count);
        }
    }
    (color, contents)
}

pub fn run() -> (String, String) {
    let bag_rules: HashMap<_, HashMap<_, _>> = INPUT.lines().map(parse).collect();
    let mut bag_cache = HashMap::new();
    bag_cache.insert("shiny gold", true);
    let part1 = bag_rules
        .keys()
        .filter(|k| contains_bag(&bag_rules, &mut bag_cache, k))
        .count();
    let part2 = bags_within(&bag_rules, "shiny gold") - 1; // Correct off-by-one error
    (part1.to_string(), part2.to_string())
}

fn bags_within(bag_rules: &HashMap<&str, HashMap<&str, u32>>, needle: &str) -> u32 {
    1 + bag_rules[needle]
        .iter()
        .map(|(k, v)| v * bags_within(bag_rules, k))
        .sum::<u32>()
}

fn contains_bag<'a>(
    bag_rules: &'a HashMap<&'a str, HashMap<&'a str, u32>>,
    bag_cache: &mut HashMap<&'a str, bool>,
    bag_name: &'a str,
) -> bool {
    if !bag_cache.contains_key(bag_name) {
        let contains = bag_rules[bag_name]
            .iter()
            .any(|(b, _)| contains_bag(bag_rules, bag_cache, b));
        bag_cache.insert(bag_name, contains);
    }
    bag_cache[bag_name]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let res = input.lines().map(parse).collect::<Vec<_>>();
        dbg!(res);
    }
}
