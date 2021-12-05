use itertools::Itertools;
use std::fs;

pub fn solve() -> i32 {
    let puzzle = fs::read_to_string("./src/day_04/data/puzzle.txt").expect("puzzle input");
    let first_row: Vec<_> = puzzle.lines().take(1).collect();
    println!("{:?}", first_row);
    let chunks: Vec<_> = puzzle.lines().chunks(6).into_iter().map(parse).collect();
    println!("{:?}", chunks);
    // for chunk in chunks.into_iter() {
    //     for value in chunk.into_iter() {
    //         println!("{:?}", value);
    //     }
    // }
    //println!("{:?}", chunks);
    //let chunks: Vec<Vec<_>> = puzzle.collect::<Vec<_>>().chunks(6).collect::<Vec<Vec<_>>>();
    0
}

fn parse(c: itertools::Chunk<std::str::Lines>) -> i32 {
    c.into_iter().fold(0, |acc, _| acc + 1)
}
