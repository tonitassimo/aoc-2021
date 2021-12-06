use super::shared::*;

pub fn solve() -> i32 {
    (0..80)
        .fold(parse(), |population, _| perform_iteration(population))
        .iter()
        .count() as i32
}

fn perform_iteration(population: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for entry in population {
        match entry {
            0 => {
                result.push(6);
                result.push(8);
            }
            _ => result.push(entry - 1),
        }
    }
    result
}
