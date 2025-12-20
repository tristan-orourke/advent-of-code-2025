use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Safe {
    dial: i32,
    zero_count: u32,
}

impl Safe {
    fn new() -> Safe {
        Safe {
            dial: 50,
            zero_count: 0,
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
    fn run_cmd(&self, cmd: &str) -> Safe {
        let value = (self.dial + Safe::parse_cmd_str(cmd)) % 100;
        Safe {
            dial: match value >= 0 {
                true => value,
                false => 100 + value,
            },
            zero_count: match value {
                0 => self.zero_count + 1,
                _ => self.zero_count,
            },
        }
    }
}

fn main() {
    let day1_str = fs::read_to_string("./src/data/day1.txt").unwrap();
    let day1 = day1_str.trim().split("\n");
    let safe = day1.fold(Safe::new(), |acc, cmd| acc.run_cmd(cmd));
    println!("Resulting state is {safe:?}");
}
