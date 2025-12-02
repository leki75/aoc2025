use std::path::Path;
use std::{fs, ops::RangeInclusive};

const INPUT_PATH: &str = "inputs/day2.txt";

pub fn main() {
    let product_ids = product_ids_in_file(INPUT_PATH);

    // count sum of invalid product IDs
    println!(
        "Part One: {}",
        product_ids
            .iter()
            .filter(|id| invalid_part1(**id))
            .sum::<u64>(),
    );
    println!(
        "Part Two: {}",
        product_ids
            .iter()
            .filter(|id| invalid_part2(**id))
            .sum::<u64>(),
    );
}

fn product_ids_in_file<P: AsRef<Path>>(path: P) -> Vec<u64> {
    let content = fs::read_to_string(path).unwrap();
    content
        .trim()
        .split(',')
        .filter_map(|s| match s.find('-') {
            Some(idx) => {
                let start = s[..idx].parse::<u64>().unwrap();
                let end = s[idx + 1..].parse::<u64>().unwrap();
                Some(RangeInclusive::new(start, end))
            }
            None => None,
        })
        .flatten()
        .collect::<Vec<u64>>()
}

pub fn invalid_part1<T: ToString>(id: T) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        return false;
    }
    let half = id_str.len() / 2;
    if id_str[..half] == id_str[half..] {
        return true;
    }
    false
}

pub fn invalid_part2<T: ToString>(id: T) -> bool {
    let id_str = id.to_string();
    let half = id_str.len() / 2;
    for i in 1..=half {
        if id_str[..i].repeat(id_str.len() / i) == id_str {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::RangeInclusive;

    static PRODUCT_ID_TEST_RANGES: [RangeInclusive<u64>; 11] = [
        11..=22,
        95..=115,
        998..=1012,
        1188511880..=1188511890,
        222220..=222224,
        1698522..=1698528,
        446443..=446449,
        38593856..=38593862,
        565653..=565659,
        824824821..=824824827,
        2121212118..=2121212124,
    ];

    fn invalid_product_id(invalid_ids: &[&[u64]], func: fn(u64) -> bool) {
        let test_cases = PRODUCT_ID_TEST_RANGES.iter().zip(invalid_ids.iter());

        for tc in test_cases {
            assert_eq!(
                tc.0.clone()
                    .filter(|x| func(*x))
                    .collect::<Vec<u64>>()
                    .as_slice(),
                *tc.1
            );
        }
    }

    #[test]
    fn test_invalid_part1() {
        let invalid_ids: [&[u64]; 11] = [
            &[11, 22],
            &[99],
            &[1010],
            &[1188511885],
            &[222222],
            &[],
            &[446446],
            &[38593859],
            &[],
            &[],
            &[],
        ];
        invalid_product_id(&invalid_ids, invalid_part1);
    }

    #[test]
    fn test_invalid_part2() {
        let invalid_ids: [&[u64]; 11] = [
            &[11, 22],
            &[99, 111],
            &[999, 1010],
            &[1188511885],
            &[222222],
            &[],
            &[446446],
            &[38593859],
            &[565656],
            &[824824824],
            &[2121212121],
        ];
        invalid_product_id(&invalid_ids, invalid_part2);
    }
}
