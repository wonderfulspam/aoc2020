fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .expect("Please provide an argument")
        .parse()
        .expect("Please provide a valid number");
    match day {
        1 => aoc2020::solutions::day1::run(),
        2 => aoc2020::solutions::day2::run(),
        _ => unimplemented!("Not done yet!"),
    }
}
