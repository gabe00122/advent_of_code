use std::error::Error;

pub type ChallengeResult = Result<Solution, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Solution {
    pub part1: String,
    pub part2: String,
}

impl Solution {
    pub fn new(part1: String, part2: String) -> Solution {
        Solution { part1, part2 }
    }

    pub fn from<T: ToString>(part1: T, part2: T) -> Solution {
        Solution {
            part1: part1.to_string(),
            part2: part2.to_string(),
        }
    }
}
