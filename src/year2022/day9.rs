use std::str::FromStr;
use crate::challenge_result::{ChallengeResult, Solution};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(())
        }
    }
}

pub fn run(input: &str) -> ChallengeResult {


    Ok(Solution::from(0, 0))
}
