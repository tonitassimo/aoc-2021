mod day_01;
mod day_02;
mod day_03;

const DAY_01: bool = false;
const DAY_02: bool = false;
const DAY_03: bool = true;

fn main() {
    if DAY_01 {
        println!("part_01: {:?}", day_01::part_01::solve());
        println!("part_02: {:?}", day_01::part_02::solve());
    }
    if DAY_02 {
        println!("part_01: {:?}", day_02::part_01::solve());
        println!("part_02: {:?}", day_02::part_02::solve());
    }
    if DAY_03 {
        println!("part_01: {:?}", day_03::part_01::solve());
        println!("part_02: {:?}", day_03::part_02::solve());
    }
}
