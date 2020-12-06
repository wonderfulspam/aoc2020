const INPUT: &str = include_str!("../inputs/day3");

pub fn run() -> (String, String) {
    let count = count_trees((3, 1));

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = slopes.iter().map(|&slope| count_trees(slope)).product();

    (count.to_string(), product.to_string())
}

fn count_trees(slope: (usize, usize)) -> usize {
    let (x, y) = slope;
    INPUT
        .lines()
        .step_by(y)
        .skip(1)
        .enumerate()
        .filter(|&(index, line)| line.as_bytes()[(index + 1) * x % line.len()] == b'#')
        .count()
}
