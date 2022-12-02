use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../inputs/day16");

type TicketRule = (RangeInclusive<usize>, RangeInclusive<usize>);
type Ticket = Vec<usize>;

pub fn run() -> (String, String) {
    let mut input_groups = INPUT.split("\n\n");
    let rules: Vec<TicketRule> = parse_rules(input_groups.next().unwrap());
    let _my_ticket = parse_ticket(input_groups.next().unwrap().lines().nth(1).unwrap());
    let nearby_tickets: Vec<Ticket> = input_groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();
    let part1 = nearby_tickets.iter().fold(0, |acc, ticket| {
        ticket
            .iter()
            .filter(|field| {
                rules
                    .iter()
                    .all(|(r1, r2)| !r1.contains(field) && !r2.contains(field))
            })
            .sum::<usize>()
            + acc
    });
    let part2 = "-";
    (part1.to_string(), part2.to_string())
}

fn parse_ticket(input: &str) -> Ticket {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_rules(input: &str) -> Vec<TicketRule> {
    input
        .lines()
        .map(|l| {
            let (first, last) = l.split_once(" or ").unwrap();
            let first = first.split(' ').last().unwrap();
            let (low1, high1) = first
                .split_once('-')
                .map(|(l, h)| (l.parse().unwrap(), h.parse().unwrap()))
                .unwrap();
            let (low2, high2) = last
                .split_once('-')
                .map(|(l, h)| (l.parse().unwrap(), h.parse().unwrap()))
                .unwrap();
            (
                RangeInclusive::new(low1, high1),
                RangeInclusive::new(low2, high2),
            )
        })
        .collect()
}
