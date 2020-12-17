use itertools::iproduct;
use std::collections::HashSet;
use std::hash::Hash;

const INPUT: &str = include_str!("../inputs/day17");
const CYCLES: usize = 6;

type Cube = (i32, i32, i32);
type Hypercube = (i32, i32, i32, i32);

pub fn run() -> (String, String) {
    let initial_state: HashSet<Cube> = parse1(INPUT);
    let part1 = solve(initial_state, find_3d_neighbours, retain_3d);
    dbg!(part1);

    let initial_state2: HashSet<Hypercube> = parse2(INPUT);
    let part2 = solve(initial_state2, find_4d_neighbours, retain_4d);

    (part1.to_string(), part2.to_string())
}

fn solve<C, F, R>(mut state: HashSet<C>, neighbour_fn: F, retain_fn: R) -> usize
where
    C: PartialEq + Eq + Hash,
    F: Fn(&HashSet<C>) -> HashSet<C>,
    R: Fn(&C, &HashSet<C>) -> bool,
{
    for _ in 0..CYCLES {
        let mut neighbours = neighbour_fn(&state);
        neighbours.retain(|cube| retain_fn(cube, &state));
        std::mem::swap(&mut neighbours, &mut state);
    }
    state.len()
}

const OFFSETS: [i32; 3] = [-1, 0, 1];

fn find_3d_neighbours(state: &HashSet<Cube>) -> HashSet<Cube> {
    let mut new_state = HashSet::<Cube>::new();
    state.iter().for_each(|cube| {
        new_state.extend(
            iproduct!(&OFFSETS, &OFFSETS, &OFFSETS)
                .map(move |(mx, my, mz)| (cube.0 + mx, cube.1 + my, cube.2 + mz)),
        )
    });
    new_state
}

fn retain_3d(cube: &Cube, state: &HashSet<Cube>) -> bool {
    let mut neighbours: HashSet<Cube> = iproduct!(&OFFSETS, &OFFSETS, &OFFSETS)
        .map(move |(mx, my, mz)| (cube.0 + mx, cube.1 + my, cube.2 + mz))
        .collect();
    let active = state.contains(cube);
    neighbours.remove(cube);
    let active_neighbours = &neighbours & state;
    if active {
        if (2..=3).contains(&active_neighbours.len()) {
            return true;
        } else {
            return false;
        }
    } else {
        return active_neighbours.len() == 3;
    }
}

fn find_4d_neighbours(state: &HashSet<Hypercube>) -> HashSet<Hypercube> {
    let mut new_state = HashSet::<Hypercube>::new();
    state.iter().for_each(|cube| {
        new_state.extend(
            iproduct!(&OFFSETS, &OFFSETS, &OFFSETS, &OFFSETS)
                .map(move |(mx, my, mz, mæ)| (cube.0 + mx, cube.1 + my, cube.2 + mz, cube.3 + mæ)),
        )
    });
    new_state
}

fn retain_4d(cube: &Hypercube, state: &HashSet<Hypercube>) -> bool {
    let mut neighbours: HashSet<Hypercube> = iproduct!(&OFFSETS, &OFFSETS, &OFFSETS, &OFFSETS)
        .map(move |(mx, my, mz, mæ)| (cube.0 + mx, cube.1 + my, cube.2 + mz, cube.3 + mæ))
        .collect();
    let active = state.contains(cube);
    neighbours.remove(cube);
    let active_neighbours = &neighbours & state;
    if active {
        if (2..=3).contains(&active_neighbours.len()) {
            return true;
        } else {
            return false;
        }
    } else {
        return active_neighbours.len() == 3;
    }
}

fn parse2(input: &str) -> HashSet<Hypercube> {
    input
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as i32, y as i32, 0, 0)),
                _ => None,
            })
        })
        .collect()
}

fn parse1(input: &str) -> HashSet<Cube> {
    input
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as i32, y as i32, 0)),
                _ => None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r".#.
..#
###
";

    #[test]
    fn test_part1() {
        let state = parse1(SAMPLE_INPUT);
        dbg!(state);
    }
}
