fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .expect("Please provide an argument")
        .parse()
        .expect("Please provide a valid number");
    match day {
        #[cfg(not(feature = "dumb"))]
        1 => aoc2020::solutions::day1::run(),
        #[cfg(feature = "dumb")]
        1 => aoc2020::solutions::day1_dumb::run(),
        2 => aoc2020::solutions::day2::run(),
        3 => aoc2020::solutions::day3::run(),
        4 => aoc2020::solutions::day4::run(),
        5 => aoc2020::solutions::day5::run(),
        _ => unimplemented!("Not done yet!"),
    }
}
