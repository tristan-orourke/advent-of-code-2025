use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Safe {
    dial: i32,
    zero_stops: u32,
    zero_passes: u32,
}

impl Safe {
    fn new() -> Safe {
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
    fn run_cmd(&self, cmd: &str) -> Safe {
        let turn = Safe::parse_cmd_str(cmd);
        let value = self.dial + turn;
        let new_dial = ((value % 100) + 100) % 100;
        Safe {
            dial: new_dial,
            zero_stops: match new_dial {
                0 => self.zero_stops + 1,
                _ => self.zero_stops,
            },
            zero_passes: (if value.abs() > 100 {
                self.zero_passes
                    + (value.abs() as u32 / 100)
                    + (if (value < 0 && self.dial != 0) { 1 } else { 0 })
            } else if new_dial == 0 {
                self.zero_passes + 1
            } else if value >= 100 {
                self.zero_passes + 1
            } else if value <= 0 && self.dial != 0 {
                self.zero_passes + 1
            } else {
                self.zero_passes
            }),
        }
    }
}

fn main() {
    let day1_str = fs::read_to_string("./src/data/day1.txt").unwrap();
    let day1 = day1_str.trim().split("\n");
    let safe = day1.fold(Safe::new(), |acc, cmd| acc.run_cmd(cmd));
    println!("Resulting state is {safe:?}");
}
