use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day11");
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn run() -> (String, String) {
    let mut grid = parse_input(INPUT);
    let mut grid_clone = grid.clone();
    let part1 = solve(&mut grid, 4, part1_neighbour_map);
    let part2 = solve(&mut grid_clone, 5, part2_neighbour_map);
    (part1.to_string(), part2.to_string())
}

pub fn test_neighbour_map1() {
    let grid = parse_input(INPUT);
    let _ = part1_neighbour_map(&grid);
}

pub fn test_neighbour_map2() {
    let grid = parse_input(INPUT);
    let _ = part2_neighbour_map(&grid);
}

fn part2_neighbour_map(grid: &[Vec<char>]) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    grid.iter()
        .enumerate()
        .map(|(x_index, l)| {
            l.iter().enumerate().map(move |(y_index, _)| {
                (
                    (x_index, y_index),
                    part2_neighbour_positions(grid, (x_index, y_index)),
                )
            })
        })
        .flatten()
        .collect()
}

fn part2_neighbour_positions(grid: &[Vec<char>], position: (usize, usize)) -> Vec<(usize, usize)> {
    DIRECTIONS
        .iter()
        .filter_map(|&(dy, dx)| {
            let (mut x, mut y) = (position.0 as isize, position.1 as isize);
            loop {
                x += dx;
                y += dy;
                match grid.get(x as usize).and_then(|l| l.get(y as usize)) {
                    Some('.') => {}
                    Some(_) => break Some((x as usize, y as usize)),
                    None => break None,
                }
            }
        })
        .collect()
}

fn part1_neighbour_map(grid: &[Vec<char>]) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    grid.iter()
        .enumerate()
        .map(|(x_index, l)| {
            l.iter().enumerate().map(move |(y_index, _)| {
                (
                    (x_index, y_index),
                    part1_neighbour_positions(grid, (x_index, y_index)),
                )
            })
        })
        .flatten()
        .collect()
}

fn part1_neighbour_positions(grid: &[Vec<char>], position: (usize, usize)) -> Vec<(usize, usize)> {
    DIRECTIONS
        .iter()
        .map(|&(dy, dx)| (position.0 as isize + dy, position.1 as isize + dx))
        .filter(|&(x, y)| {
            grid.get(x as usize)
                .and_then(|v| v.get(y as usize))
                .is_some()
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn solve<F>(grid: &mut Vec<Vec<char>>, occupied_threshold: usize, neigbour_fn: F) -> usize
where
    F: Fn(&[Vec<char>]) -> HashMap<(usize, usize), Vec<(usize, usize)>>,
{
    let mut next_grid = grid.clone();
    let neighbour_map = neigbour_fn(&grid);
    loop {
        let mut has_changed = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let seat = grid[i][j];
                if seat == '.' {
                    continue;
                }
                let neighbours = &neighbour_map[&(i, j)];
                let should_flip = match seat {
                    'L' => neighbours
                        .iter()
                        .all(|&(x, y)| grid[x as usize][y as usize] != '#'),
                    '#' => {
                        neighbours
                            .iter()
                            .filter(|&&(x, y)| grid[x as usize][y as usize] == '#')
                            .count()
                            >= occupied_threshold
                    }
                    _ => unreachable!(),
                };
                let c = match (seat, should_flip) {
                    ('L', true) => '#',
                    ('#', true) => 'L',
                    _ => seat,
                };
                next_grid[i][j] = c;
                has_changed |= c != seat;
            }
        }

        std::mem::swap(grid, &mut next_grid);
        if !has_changed {
            break;
        }
    }
    grid.iter()
        .flat_map(|l| l.iter().filter(|&c| *c == '#'))
        .count()
}

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn test_part1() {
        let mut grid = parse_input(SAMPLE_INPUT);
        let part1 = solve(&mut grid, 4, part1_neighbour_map);
        assert_eq!(part1, 37);
    }
}
