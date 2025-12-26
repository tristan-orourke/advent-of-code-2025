use std::fs;
mod day1;
mod day2;
mod day3;

fn main() {
    let day1_str = fs::read_to_string("./src/data/day1.txt").unwrap();
    let day1 = day1_str.trim().split("\n");
    let safe = day1.fold(day1::Safe::new(), |acc, cmd| acc.run_cmd(cmd));
    println!("Resulting state is {safe:?}");
    let day2_str = fs::read_to_string("./src/data/day2.txt").unwrap();
    let day2_result = day2::process_ids_sum_duplicates(day2_str.trim());
    println!("Day 2 duplicate sum is {day2_result:?}");
}
