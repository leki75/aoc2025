pub struct Dial {
    position: i32,
    clicks_to_zero: i32,
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            position: 50,
            clicks_to_zero: 0,
        }
    }

    pub fn rotate(&mut self, rotation: &str) {
        let prev_position = self.position;
        let rotation = match rotation.as_bytes()[0] {
            b'L' => -rotation[1..].parse::<i32>().unwrap(),
            b'R' => rotation[1..].parse::<i32>().unwrap(),
            _ => panic!("invalid rotation direction"),
        };
        self.clicks_to_zero += rotation.abs() / 100;
        self.position += rotation % 100;

        let (pos_adj, click_adj) = match self.position {
            ..0 => match prev_position {
                0 => (100, 0),
                _ => (100, 1),
            },
            0 => (0, 1),
            100.. => (-100, 1),
            _ => (0, 0),
        };
        self.clicks_to_zero += click_adj;
        self.position += pos_adj;
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn clicks_to_zero(&self) -> i32 {
        self.clicks_to_zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: [&str; 10] = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn rotation() {
        let mut dial = Dial::new();
        assert_eq!(dial.position, 50);

        let positions = [82, 52, 0, 95, 55, 0, 99, 0, 14, 32];
        let test_cases = TEST_CASES.iter().zip(positions.iter());
        for tc in test_cases {
            dial.rotate(tc.0);
            assert_eq!(dial.position(), *tc.1, "Failed on rotation {}", tc.0);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    // 0x434C49434B == "CLICK"
    fn password_method_0x434C49434B() {
        let mut dial = Dial::new();
        assert_eq!(dial.position, 50);

        for tc in TEST_CASES {
            dial.rotate(tc);
        }
        assert_eq!(dial.clicks_to_zero(), 6);

        dial = Dial::new();
        dial.rotate("R150");
        assert_eq!(dial.position(), 0);
        assert_eq!(dial.clicks_to_zero(), 2);
        dial.rotate("L150");
        assert_eq!(dial.position(), 50);
        assert_eq!(dial.clicks_to_zero(), 3);
    }
}
