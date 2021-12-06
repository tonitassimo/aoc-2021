use std::fs;

pub fn parse() -> Vec<i32> {
    fs::read_to_string("./src/day_06/data/puzzle.txt")
        .expect("puzzle input")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
