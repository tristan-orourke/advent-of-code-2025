use regex::Regex;

#[derive(Debug)]
pub struct Safe {
    pub dial: i32,
    pub zero_stops: u32,
    pub zero_passes: u32,
}

impl Safe {
    pub fn new() -> Safe {
        Safe {
            dial: 50,
            zero_stops: 0,
            zero_passes: 0,
        }
    }
    fn parse_cmd_str(cmd: &str) -> i32 {
        let re = Regex::new(r"(\D)(\d+)").unwrap();
        let Some((_, [direction, value_str])) = re.captures(cmd).map(|caps| caps.extract()) else {
            panic!("Cannot parse cmd: {cmd}")
        };
        let value: i32 = str::parse(value_str).unwrap_or(0);
        match direction {
            "L" => -value,
            _ => value,
        }
    }
    pub fn run_cmd(&self, cmd: &str) -> Safe {
        let turn = Safe::parse_cmd_str(cmd);
        let value = self.dial + turn;
        let new_dial = ((value % 100) + 100) % 100;
        Safe {
            dial: new_dial,
            zero_stops: match new_dial {
                0 => self.zero_stops + 1,
                _ => self.zero_stops,
            },
            zero_passes: match (value, self.dial) {
                (..0, 0) => self.zero_passes + (value / -100).abs() as u32,
                (..0, _) => self.zero_passes + 1 + (value / -100).abs() as u32,
                (0, _) => self.zero_passes + 1,
                (_, _) => self.zero_passes + (value / 100).abs() as u32,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn print_return<T: std::fmt::Debug>(x: T) -> T {
        println!("{x:?}");
        x
    }

    #[test]
    fn day1_test_input() {
        let day1_str = fs::read_to_string("./src/data/day1_test1.txt").unwrap();
        let day1 = day1_str.trim().split("\n");
        let safe = day1.fold(Safe::new(), |acc, cmd| print_return(acc.run_cmd(cmd)));
        assert_eq!(safe.zero_stops, 3);
        assert_eq!(safe.zero_passes, 6);
    }

    #[test]
    fn day1_big_rotation() {
        let safe = Safe::new().run_cmd("R1000");
        assert_eq!(safe.zero_passes, 10);
        assert_eq!(Safe::new().run_cmd("L1000").zero_passes, 10);
        assert_eq!(Safe::new().run_cmd("R50").zero_passes, 1);
        assert_eq!(print_return(Safe::new().run_cmd("R150")).zero_passes, 2);
    }

    #[test]
    fn i_understand_division() {
        assert_eq!(6 / 10, 0);
        assert_eq!(12 / 10, 1);
    }
}
