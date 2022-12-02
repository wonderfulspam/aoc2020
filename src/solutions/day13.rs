const INPUT: &str = include_str!("../inputs/day13");

pub fn run() -> (String, String) {
    let mut input = INPUT.lines();
    let departure = input.next().unwrap();
    let busses = input.next().unwrap();
    let busses: Vec<_> = busses.split(',').map(|s| s.parse().unwrap_or(1)).collect();

    let part1 = part1(departure.parse().unwrap(), &busses);
    let part2 = part2(&busses);
    (part1.to_string(), part2.to_string())
}

fn part1(departure: usize, busses: &[usize]) -> usize {
    for time in departure.. {
        if let Some(bus) = busses.iter().filter(|&&x| x != 1).find(|&b| time % b == 0) {
            return bus * (time - departure);
        }
    }
    unreachable!()
}

fn part2(busses: &[usize]) -> usize {
    let lcd: usize = busses.iter().product();

    let factors: Vec<usize> = busses
        .iter()
        .rev()
        .enumerate()
        .map(|(i, bus)| {
            let factor = lcd / bus;
            let target = i % bus;
            let j = (1..).find(|j| (j * factor) % bus == target).unwrap();
            factor * j
        })
        .collect();

    factors.iter().sum::<usize>() % lcd - busses.len() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"939
7,13,x,x,59,x,31,19
";

    #[test]
    fn test_part1() {
        let (departure, busses) = SAMPLE_INPUT.split_once('\n').unwrap();
        let busses: Vec<_> = busses.split(',').filter_map(|s| s.parse().ok()).collect();
        let result = part1(departure.parse().unwrap(), &busses);
        assert_eq!(result, 295);
    }

    #[test]
    fn test_part2() {
        let busses = [7, 13, 1, 1, 59, 1, 31, 19];
        let result = part2(&busses);
        assert_eq!(result, 1068781);

        let busses = [17, 1, 13, 19];
        let result = part2(&busses);
        assert_eq!(result, 3417);

        let busses = [67, 7, 59, 61];
        let result = part2(&busses);
        assert_eq!(result, 754018);
    }
}
