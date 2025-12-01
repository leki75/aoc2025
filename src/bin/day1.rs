use aoc2025::Dial;

pub fn main() {
    use std::fs;

    let mut dial = Dial::new();
    let zeroes = fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .split_whitespace()
        .filter_map(|x| {
            dial.rotate(x);
            match dial.position() {
                0 => Some(b'0'),
                _ => None,
            }
        })
        .count();
    println!("Part One: {zeroes}");

    let mut dial = Dial::new();
    for rotation in fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .split_whitespace()
    {
        dial.rotate(rotation);
    }
    println!("Part Two: {}", dial.clicks_to_zero());
}
