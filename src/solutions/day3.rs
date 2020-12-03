const INPUT: &str = include_str!("../inputs/day3");

pub fn run() {
    let count = count_trees((3, 1));
    println!("{}", count);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = slopes.iter().map(|&slope| count_trees(slope)).product();
    println!("{}", product);
}

fn count_trees(slope: (usize, usize)) -> usize {
    INPUT
        .lines()
        .step_by(slope.1)
        .enumerate()
        .filter(|&(index, line)| line.as_bytes()[index * slope.0 % line.len()] == '#' as u8)
        .count()
}
