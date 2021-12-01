use std::fs;

pub fn solve() -> i32 {
    let numbers: Vec<_> = fs::read_to_string("./src/day_02/data/puzzle.txt")
        .expect("Something went wrong reading the file")
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    let sums: Vec<_> = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .zip(numbers.iter().skip(2))
        .map(|((l, m), r)| l + m + r)
        .collect();
    sums.iter()
        .zip(sums.iter().skip(1))
        .map(|(l, r)| if r > l { 1 } else { 0 })
        .sum()
}
