mod point;
mod grid;
mod math;
mod error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::challenge_result::ChallengeResult;

pub fn run_challenge(input: &str, day: u8) -> ChallengeResult {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        5 => day5::run(input),
        6 => day6::run(input),
        7 => day7::run(input),
        8 => day8::run(input),
        9 => day9::run(input),
        10 => day10::run(input),
        11 => day11::run(input),
        12 => day12::run(input),
        13 => day13::run(input),
        14 => day14::run(input),
        15 => day15::run(input),
        16 => day16::run(input),
        17 => day17::run(input),
        18 => day18::run(input),
        19 => day19::run(input),
        20 => day20::run(input),
        21 => day21::run(input),
        22 => day22::run(input),
        23 => day23::run(input),
        24 => day24::run(input),
        25 => day25::run(input),
        _ => panic!("Not a valid day."),
    }
}
