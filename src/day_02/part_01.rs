use super::shared::*;
use std::fs;

pub fn solve() -> (i32, i32) {
    let end_position = fs::read_to_string("./src/day_02/data/puzzle.txt")
        .expect("collection of commands")
        .lines()
        .map(|s| parse(&s))
        .fold((0, 0), execute_command);
    end_position
}

fn execute_command(mut acc: (i32, i32), command: Command) -> (i32, i32) {
    match command.direction {
        Direction::Up => acc.1 -= command.units,
        Direction::Down => acc.1 += command.units,
        Direction::Forward => acc.0 += command.units,
    }
    acc
}
