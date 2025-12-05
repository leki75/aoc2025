use std::fs;

const INPUT_PATH: &str = "inputs/day3.txt";

fn max_byte(s: &str) -> (u8, usize) {
    let (mut max_pos, mut max_byte) = (0, 0);
    for (pos, ch) in s.as_bytes().iter().enumerate() {
        if *ch > max_byte {
            max_byte = *ch;
            max_pos = pos;
        }
    }
    (max_byte - b'0', max_pos)
}

fn largest_joltage(s: &str, size: usize) -> u64 {
    let mut num = 0;
    let mut pos = 0;
    for end in (0..size).rev() {
        let (ch, idx) = max_byte(&s[pos..s.len() - end]);
        pos += idx + 1;
        num = num * 10 + ch as u64;
    }
    num
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    println!(
        "Part One: {}",
        content
            .split_whitespace()
            .map(|line| largest_joltage(line, 2))
            .sum::<u64>()
    );
    println!(
        "Part Two: {}",
        content
            .split_whitespace()
            .map(|line| largest_joltage(line, 12))
            .sum::<u64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_joltage_part1() {
        let joltage_test_cases: [(&str, u64); _] = [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];

        for test_case in joltage_test_cases {
            assert_eq!(largest_joltage(test_case.0, 2), test_case.1);
        }
    }

    #[test]
    fn test_largest_joltage_part2() {
        let joltage_test_cases: [(&str, u64); _] = [
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
        ];

        for test_case in joltage_test_cases {
            assert_eq!(largest_joltage(test_case.0, 12), test_case.1);
        }
    }
}
