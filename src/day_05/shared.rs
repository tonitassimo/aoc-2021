use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
}

pub fn parse() -> Vec<Edge> {
    let puzzle = fs::read_to_string("./src/day_05/data/puzzle.txt").expect("puzzle input");
    puzzle.lines().map(line_to_edge).collect()
}

pub fn merge_maps(mut acc: HashMap<Point, i32>, points: Vec<Point>) -> HashMap<Point, i32> {
    points.into_iter().for_each(|p| {
        if acc.contains_key(&p) {
            let count = acc.get(&p).unwrap() + 1;
            acc.insert(p, count);
        } else {
            acc.insert(p, 1);
        }
    });
    acc
}

fn line_to_edge(line: &str) -> Edge {
    let re = Regex::new(r"([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)").unwrap();
    let mut edge = Edge {
        start: Point { x: 0, y: 0 },
        end: Point { x: 0, y: 0 },
    };
    for caps in re.captures_iter(line) {
        let start = Point {
            x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        };
        let end = Point {
            x: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            y: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        };
        edge = Edge { start, end }
    }
    edge
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}
