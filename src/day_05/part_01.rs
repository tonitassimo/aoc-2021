use super::shared::*;
use std::collections::HashMap;

pub fn solve() -> i32 {
    parse()
        .into_iter()
        .map(calculate_visited_points)
        .fold(HashMap::new(), merge_maps)
        .iter()
        .filter(|kvp| kvp.1 > &1)
        .count() as i32
}

fn calculate_visited_points(edge: Edge) -> Vec<Point> {
    if edge.start.x == edge.end.x {
        let r = create_range_from_borders(edge.start.y, edge.end.y);
        return r
            .into_iter()
            .map(|val| Point {
                x: edge.start.x,
                y: val,
            })
            .collect();
    } else if edge.start.y == edge.end.y {
        let r = create_range_from_borders(edge.start.x, edge.end.x);
        return r
            .into_iter()
            .map(|val| Point {
                x: val,
                y: edge.start.y,
            })
            .collect();
    }
    vec![]
}

fn create_range_from_borders(a: i32, b: i32) -> std::ops::Range<i32> {
    if a < b {
        return a..b + 1;
    }
    return b..a + 1;
}
