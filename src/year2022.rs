mod day1;

use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

pub fn run_challenge(input: &str, day: u8) -> ChallengeResult {
    match day {
        1 => day1::run(input),
        _ => Ok(ChallengeSuccess::new(0, 0)),
    }
}
