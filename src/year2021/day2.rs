use crate::challenge_result::{ChallengeResult, ChallengeSuccess};
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
    length: i32,
}

pub fn run(input: &str) -> ChallengeResult {
    let input: Result<Vec<MoveInstruction>, Box<dyn error::Error>> = input
        .lines()
        .map(|line| {
            let direction = Direction::from_str(&line[..line.len() - 2])?;
            let length: i32 = line[line.len() - 1..].parse()?;

            Ok(MoveInstruction { direction, length })
        })
        .collect();

    let input = input?;

    Ok(ChallengeSuccess::new(part1(&input), part2(&input)))
}

fn part1(input: &[MoveInstruction]) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for MoveInstruction { direction, length } in input {
        match direction {
            Direction::Up => depth -= length,
            Direction::Down => depth += length,
            Direction::Forward => horizontal += length,
        }
    }

    horizontal * depth
}

fn part2(input: &[MoveInstruction]) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for MoveInstruction { direction, length } in input {
        match direction {
            Direction::Up => aim -= length,
            Direction::Down => aim += length,
            Direction::Forward => {
                horizontal += length;
                depth += aim * length;
            }
        }
    }

    horizontal * depth
}
