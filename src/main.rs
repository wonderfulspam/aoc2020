fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .expect("Please provide an argument")
        .parse()
        .expect("Please provide a valid number");
    let (part1, part2) = match day {
        #[cfg(not(feature = "dumb"))]
        1 => aoc2020::solutions::day1::run(),
        #[cfg(feature = "dumb")]
        1 => aoc2020::solutions::day1_dumb::run(),
        2 => aoc2020::solutions::day2::run(),
        3 => aoc2020::solutions::day3::run(),
        #[cfg(feature = "dumb")]
        4 => aoc2020::solutions::day4_inlined::run(),
        #[cfg(not(feature = "dumb"))]
        4 => aoc2020::solutions::day4::run(),
        5 => aoc2020::solutions::day5::run(),
        6 => aoc2020::solutions::day6::run(),
        _ => unimplemented!("Not done yet!"),
    };
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
