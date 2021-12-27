use std::error::Error;

pub type ChallengeResult = Result<ChallengeSuccess, Box<dyn Error>>;

#[derive(Debug, Copy, Clone)]
pub struct ChallengeSuccess {
    pub part1: u64,
    pub part2: u64,
}

impl ChallengeSuccess {
    pub fn new(part1: u64, part2: u64) -> Self {
        ChallengeSuccess { part1, part2 }
    }
}
