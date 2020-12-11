fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .expect("Please provide an argument")
        .parse()
        .expect("Please provide a valid number");
    let (part1, part2) = match day {
        #[cfg(not(feature = "alternatives"))]
        1 => aoc2020::solutions::day1::run(),
        #[cfg(feature = "alternatives")]
        1 => aoc2020::solutions::day1_iterators_smart::run(),
        #[cfg(not(feature = "alternatives"))]
        2 => aoc2020::solutions::day2::run(),
        #[cfg(feature = "alternatives")]
        2 => aoc2020::solutions::day2_stringsplit::run(),
        3 => aoc2020::solutions::day3::run(),
        #[cfg(feature = "alternatives")]
        4 => aoc2020::solutions::day4_inlined::run(),
        #[cfg(not(feature = "alternatives"))]
        4 => aoc2020::solutions::day4::run(),
        5 => aoc2020::solutions::day5::run(),
        #[cfg(not(feature = "alternatives"))]
        6 => aoc2020::solutions::day6::run(),
        #[cfg(feature = "alternatives")]
        6 => aoc2020::solutions::day6_optimized::run(),
        7 => aoc2020::solutions::day7::run(),
        8 => aoc2020::solutions::day8::run(),
        9 => aoc2020::solutions::day9::run(),
        10 => aoc2020::solutions::day10::run(),
        11 => aoc2020::solutions::day11::run(),
        _ => unimplemented!("Not done yet!"),
    };
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
