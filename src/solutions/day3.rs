const INPUT: &str = include_str!("../inputs/day3");
const SLOPES: &[(usize, usize); 4] = &[(1, 1), (5, 1), (7, 1), (1, 2)];

pub fn run() -> (String, String) {
    let count = count_trees((3, 1));

    let product: usize = SLOPES.iter().map(|&slope| count_trees(slope)).product();
    let product = product * count;

    (count.to_string(), product.to_string())
}

fn count_trees(slope: (usize, usize)) -> usize {
    let (x, y) = slope;
    INPUT
        .lines()
        .step_by(y)
        .enumerate()
        .filter(|&(index, line)| line.as_bytes()[index * x % line.len()] == b'#')
        .count()
}
