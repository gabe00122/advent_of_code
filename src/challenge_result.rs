use std::error::Error;

pub type ChallengeResult = Result<ChallengeSuccess, Box<dyn Error>>;

#[derive(Debug, Copy, Clone)]
pub struct ChallengeSuccess {
    pub part1: i32,
    pub part2: i32,
}

impl ChallengeSuccess {
    pub fn new(part1: i32, part2: i32) -> Self {
        ChallengeSuccess { part1, part2 }
    }
}
