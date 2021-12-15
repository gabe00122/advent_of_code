mod day1;
mod day2;
mod day3;
mod day4;

use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

pub fn run_challenge(input: &str, day: u8) -> ChallengeResult {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        _ => Ok(ChallengeSuccess::new(0, 0)),
    }
}
