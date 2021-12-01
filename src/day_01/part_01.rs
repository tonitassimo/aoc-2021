use std::fs;

pub fn solve() -> i16 {
    let numbers: Vec<_> = fs::read_to_string("./src/day_01/data/puzzle.txt")
        .expect("Something went wrong reading the file")
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(l, r)| if r > l { 1 } else { 0 })
        .sum()
}
