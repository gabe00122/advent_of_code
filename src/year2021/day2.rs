use crate::challenge_result::{ChallengeResult, Solution};
use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct DirectionParseError;

impl fmt::Display for DirectionParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "The direction does not match (up, down, forward)")
    }
}

impl error::Error for DirectionParseError {}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(DirectionParseError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct MoveInstruction {
    direction: Direction,
    length: u64,
}

pub fn run(input: &str) -> ChallengeResult {
    let input: Result<Vec<MoveInstruction>, Box<dyn error::Error>> = input
        .lines()
        .map(|line| {
            let direction = Direction::from_str(&line[..line.len() - 2])?;
            let length: u64 = line[line.len() - 1..].parse()?;

            Ok(MoveInstruction { direction, length })
        })
        .collect();

    let input = input?;

    Ok(Solution::from(part1(&input), part2(&input)))
}

fn part1(input: &[MoveInstruction]) -> u64 {
    use Direction::*;

    let mut horizontal: u64 = 0;
    let mut depth: u64 = 0;

    for MoveInstruction { direction, length } in input {
        match direction {
            Up => depth -= length,
            Down => depth += length,
            Forward => horizontal += length,
        }
    }

    horizontal * depth
}

fn part2(input: &[MoveInstruction]) -> u64 {
    use Direction::*;

    let mut horizontal: u64 = 0;
    let mut depth: u64 = 0;
    let mut aim: u64 = 0;

    for MoveInstruction { direction, length } in input {
        match direction {
            Up => aim -= length,
            Down => aim += length,
            Forward => {
                horizontal += length;
                depth += aim * length;
            }
        }
    }

    horizontal * depth
}
