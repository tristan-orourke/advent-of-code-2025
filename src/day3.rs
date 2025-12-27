#[derive(Clone)]
pub struct BatteryBank {
    pub batteries: Vec<u64>,
    pub max_joltage: u64,
}

impl BatteryBank {
    pub fn new(batteries: Vec<u64>) -> BatteryBank {
        let (max, _, _) = BatteryBank::largest_two_digits(&batteries);
        BatteryBank {
            batteries: batteries,
            max_joltage: max as u64,
        }
    }

    fn string_to_digits(s: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        if s.chars().all(|c| c.is_digit(10)) {
            Ok(s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        } else {
            Err("s must be all digits".into())
        }
    }

    pub fn from_string(s: &str) -> Result<BatteryBank, Box<dyn std::error::Error>> {
        let batteries = BatteryBank::string_to_digits(s)?;
        Ok(BatteryBank::new(batteries))
    }

    fn find_max_with_pos(digits: &[u64]) -> (usize, u64) {
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
    pub fn largest_two_digits(digits: &[u64]) -> (u64, usize, usize) {
        let (i, max) = BatteryBank::find_max_with_pos(&digits[..digits.len() - 1]);
        let (j, max_2) = BatteryBank::find_max_with_pos(&digits[(i + 1)..]);

        (max * 10 + max_2, i, j + i + 1)
    }

    pub fn largest_n_digits(n: u64, digits: &[u64]) -> Vec<u64> {
        let (i, max) = BatteryBank::find_max_with_pos(&digits[..digits.len() - (n - 1) as usize]);
        if n == 1 {
            vec![max]
        } else {
            let mut max_digits = vec![max];
            max_digits.append(&mut BatteryBank::largest_n_digits(
                n - 1,
                &digits[(i + 1)..],
            ));
            max_digits
        }
    }
    pub fn get_max_joltage(&self, n: u64) -> u64 {
        BatteryBank::largest_n_digits(n, &self.batteries)
            .into_iter()
            .reduce(|acc, d| acc * 10 + d)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests_day3 {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_max() {
        let digits: Vec<u64> = vec![2, 5, 0, 5];
        assert_eq!(
            BatteryBank::find_max_with_pos(&digits),
            (1 as usize, 5 as u64)
        );
    }

    #[test]
    fn test_largest_two() {
        let digits: Vec<u64> = vec![5, 1, 1, 5, 4];
        assert_eq!(BatteryBank::largest_two_digits(&digits), (55, 0, 3));
        let digits: Vec<u64> = vec![5, 1, 1, 6, 4];
        assert_eq!(BatteryBank::largest_two_digits(&digits), (64, 3, 4));
        let digits: Vec<u64> = vec![5, 1, 1, 6, 7];
        assert_eq!(BatteryBank::largest_two_digits(&digits), (67, 3, 4));
    }

    #[test]
    fn test_largest_n_digits() {
        assert_eq!(
            BatteryBank::largest_n_digits(3, &vec![5, 1, 1, 5, 4]),
            vec![5, 5, 4]
        );
    }

    #[test]
    fn test_string_to_digits() {
        assert_eq!(BatteryBank::string_to_digits("215").unwrap(), vec![2, 1, 5]);
    }

    #[test]
    fn test_12_joltage_sum() {
        let day3_str = fs::read_to_string("./src/data/day3_test.txt").unwrap();
        let banks = day3_str
            .trim()
            .split("\n")
            .map(|s| BatteryBank::from_string(s));
        let joltage_sum_12: u64 = banks.map(|b| b.unwrap().get_max_joltage(12)).sum();
        assert_eq!(joltage_sum_12, 3121910778619);
    }
}
