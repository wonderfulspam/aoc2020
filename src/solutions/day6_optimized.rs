/// Adapted from https://github.com/benediktwerner/AdventOfCode/blob/master/2020/optimized/src/days/day06.rs

const BYTES: &[u8] = include_str!("../inputs/day6").as_bytes();

pub fn run() -> (String, String) {
    let mut output = (0, 0);
    let mut i = 0;
    loop {
        let mut any = 0;
        let mut all = 0xffff_ffff;

        loop {
            let mut answers = 0;

            loop {
                answers |= 1_u32.wrapping_shl(BYTES[i] as u32);
                i += 1;
                if BYTES[i] == b'\n' {
                    break;
                }
            }

            any |= answers;
            all &= answers;

            i += 1;

            let is_end = i >= BYTES.len();
            if is_end || BYTES[i] == b'\n' {
                output.0 += any.count_ones();
                output.1 += all.count_ones();

                if is_end {
                    return (output.0.to_string(), output.1.to_string());
                } else {
                    i += 1;
                    break;
                }
            }
        }
    }
}
