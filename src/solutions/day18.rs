use lalrpop_util::lalrpop_mod;

const INPUT: &str = include_str!("../inputs/day18");

lalrpop_mod!(
    #[allow(clippy::all)]
    day18_mod
);
lalrpop_mod!(
    #[allow(clippy::all)]
    day18_mod_p2
);

pub fn run() -> (String, String) {
    let parser = day18_mod::ExprParser::new();
    let part1 = INPUT.lines().map(|l| parser.parse(l).unwrap()).sum::<u64>();
    let parser = day18_mod_p2::ExprParser::new();
    let part2 = INPUT.lines().map(|l| parser.parse(l).unwrap()).sum::<u64>();

    (part1.to_string(), part2.to_string())
}
