use std::fs;

const INPUT_PATH: &str = "inputs/day4.txt";

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn new(input: &Vec<&str>) -> Self {
        let height = input.len();
        let width = if height > 0 { input[0].len() } else { 0 };
        let mut data  = Vec::with_capacity(height);
        for row in input {
            data.push(row.chars().collect());
        }
        Matrix {data, width, height}
    }

    pub fn mark_removable_paper_rolls(&mut self) -> i32 {
        let mut changed = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y][x] == '@' && self.adjacent_paper_rolls(x, y) < 4 {
                    self.data[y][x] = 'x';
                    changed += 1;
                }
            }
        }
        changed
    }

    fn cleanup_paper_rolls(&mut self) -> bool{
        let mut changed = false;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y][x] == 'x' {
                    self.data[y][x] = '.';
                    changed = true;
                }
            }
        }
        changed
    }

    fn adjacent_paper_rolls(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;
        for dy in -1_i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x < 0 && y < 0 {
                    continue;
                }

                let (x, y) = (x as usize, y as usize);
                if x < self.width && y < self.height {
                    count += match self.data[y][x] {
                        'x' | '@' => 1,
                        _ => 0,
                    };
                }
            }
        }
        count
    }
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();
    let lines = content.
        split_whitespace().
        collect::<Vec<_>>();

    let mut matrix = Matrix::new(&lines);
    println!("Part One: {:?}", matrix.mark_removable_paper_rolls());

    let mut matrix = Matrix::new(&lines);
    let mut count = matrix.mark_removable_paper_rolls();
    while matrix.cleanup_paper_rolls() {
        count += matrix.mark_removable_paper_rolls();
    }
    println!("Part Two: {:?}", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Vec<&'static str> = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];
    }

    #[test]
    fn test_mark_removable_paper_rolls_part1() {
        let mut input = Matrix::new(&TEST_INPUT);
        let output = vec![
            "..xx.xx@x.",
            "x@@.@.@.@@",
            "@@@@@.x.@@",
            "@.@@@@..@.",
            "x@.@@@@.@x",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "x.@@@.@@@@",
            ".@@@@@@@@.",
            "x.x.@@@.x.",
        ];

        assert_eq!(input.mark_removable_paper_rolls(), 13);
        assert_eq!(
            input.data.
                iter().
                map(|row| row.iter().collect::<String>()).
                collect::<Vec<_>>(), 
            output,
        );
    }  

    #[test]
    fn test_mark_removable_paper_rolls_part2() {
        let mut input = Matrix::new(&TEST_INPUT);
        let output = vec![
            "..........",
            "..........",
            "..........",
            "....@@....",
            "...@@@@...",
            "...@@@@@..",
            "...@.@.@@.",
            "...@@.@@@.",
            "...@@@@@..",
            "....@@@..."
        ];

        let mut count = input.mark_removable_paper_rolls();
        while input.cleanup_paper_rolls() {
            count += input.mark_removable_paper_rolls();
        }

        assert_eq!(count, 43);
        assert_eq!(
            input.data.
                iter().
                map(|row| row.iter().collect::<String>()).
                collect::<Vec<_>>(), 
            output,
        );
    }  
}