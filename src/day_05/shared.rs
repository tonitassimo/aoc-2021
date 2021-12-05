use std::fs;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Edge {
    pub start: Point,
    pub end: Point,
}

pub fn parse() -> Vec<Edge> {
    let puzzle = fs::read_to_string("./src/day_05/data/puzzle.txt").expect("puzzle input");
}
