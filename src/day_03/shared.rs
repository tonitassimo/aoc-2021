#[derive(Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Command {
    pub direction: Direction,
    pub units: i32,
}

pub fn parse(s: &str) -> Command {
    let parts: Vec<_> = s.split(' ').collect();
    let direction = match parts[0] {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => panic!("unkown direction was provided"),
    };
    Command {
        direction,
        units: parts[1].parse::<i32>().unwrap(),
    }
}
