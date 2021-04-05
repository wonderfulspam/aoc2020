const INPUT: &str = include_str!("../inputs/day9");

pub fn run() -> (String, String) {
    let nums: Vec<_> = INPUT.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    let preamble_length = 25;
    let part1 = part1(&nums, preamble_length);
    let part2 = part2(&nums, part1);
    (part1.to_string(), part2.to_string())
}

fn part1(nums: &[usize], preamble_length: usize) -> usize {
    nums.iter()
        .enumerate()
        .skip(preamble_length)
        .find(|&(index, &n)| {
            let window = &nums[index - preamble_length..index];
            !window.iter().any(|&v| {
                n.checked_sub(v)
                    .map(|x| window.contains(&x))
                    .unwrap_or(false)
            })
        })
        .map(|(_, &v)| v)
        .unwrap()
}

fn part2(nums: &[usize], needle: usize) -> usize {
    let (mut i, mut j, mut sum) = (0, 0, 0);
    let (lower, upper) = loop {
        if sum < needle {
            sum += nums[i];
            i += 1;
        } else {
            sum -= nums[j];
            j += 1;
        }

        if sum == needle {
            break (j, i);
        }
    };

    let range = &nums[lower..upper];
    range.iter().min().unwrap() + range.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let nums: Vec<_> = SAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect();
        let preamble_length = 5;
        let num = part1(&nums, preamble_length);
        assert_eq!(127, num);
    }

    #[test]
    fn test_part2() {
        let nums: Vec<_> = SAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect();
        let preamble_length = 5;
        let num = part1(&nums, preamble_length);
        let num = part2(&nums, num);
        assert_eq!(62, num);
    }

    const SAMPLE_INPUT: &str = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
}
