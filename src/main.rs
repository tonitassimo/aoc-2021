mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

const DAY_01: bool = false;
const DAY_02: bool = false;
const DAY_03: bool = false;
const DAY_04: bool = false;
const DAY_05: bool = false;
const DAY_06: bool = true;

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
    if DAY_04 {
        println!("part_01: {:?}", day_04::part_01::solve());
        println!("part_02: {:?}", day_04::part_02::solve());
    }
    if DAY_05 {
        println!("part_01: {:?}", day_05::part_01::solve());
        println!("part_02: {:?}", day_05::part_02::solve());
    }
    if DAY_06 {
        println!("part_01: {:?}", day_06::part_01::solve());
        println!("part_02: {:?}", day_06::part_02::solve());
    }
}
