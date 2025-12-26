pub struct BatteryBank {
    batteries: Vec<u32>,
    max_joltage: u32,
}

impl BatteryBank {
    pub fn new(batteries: Vec<u32>) -> BatteryBank {
        let (max, _, _) = BatteryBank::largest_two_digits(&batteries);
        BatteryBank {
            batteries: batteries,
            max_joltage: max as u32,
        }
    }

    pub fn from_string(s: &str) -> Result<BatteryBank, Box<dyn std::error::Error>> {
        if s.chars().all(|c| c.is_digit(10)) {
            let batteries = s.chars().map(|c| c.to_digit(10).unwrap() as u32).collect();
            Ok(BatteryBank::new(batteries))
        } else {
            Err("s must be all digits".into())
        }
    }
    fn find_max_with_pos(digits: &[u32]) -> (usize, u32) {
        let find_max_inner = |(i, max), (index, digit)| {
            if digit > max {
                (index, digit)
            } else {
                (i, max)
            }
        };
        let (i, n) = digits
            .iter()
            .enumerate()
            .reduce(find_max_inner)
            .unwrap_or((0, &0));
        (i, n.to_owned())
    }

    // Returns largest two digits, and indexes of both
    pub fn largest_two_digits(digits: &[u32]) -> (u32, usize, usize) {
        let (i, max) = BatteryBank::find_max_with_pos(digits);
        let (j, max_2) = BatteryBank::find_max_with_pos(&digits[(i+1)..]);

        (max * 10 + max_2, i, j+i+1)
    }
}

#[cfg(test)]
mod tests_day3 {
    use super::*;

    #[test]
    fn test_find_max() {
        let digits: Vec<u32> = vec![2, 5, 0, 5];
        assert_eq!(
            BatteryBank::find_max_with_pos(&digits),
            (1 as usize, 5 as u32)
        );
    }

    #[test]
    fn test_largest_two() {
        let digits: Vec<u32> = vec![5, 1, 1, 5, 4];
        assert_eq!(BatteryBank::largest_two_digits(&digits), (55, 0, 3));
        let digits: Vec<u32> = vec![5, 1, 1, 6, 4];
        assert_eq!(BatteryBank::largest_two_digits(&digits), (64, 3, 4));
    }
}
