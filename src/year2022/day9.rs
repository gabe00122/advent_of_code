use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::challenge_result::{ChallengeResult, Solution};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct ParseDirectionError;

impl Display for ParseDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseDirectionError")
    }
}

impl Error for ParseDirectionError {}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionError)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    distance: u8,
}

#[derive(Debug, Copy, Clone)]
enum ParseMoveError {
    Direction,
    Distance,
}

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseMoveError::Direction => "ParseMoveError: Direction",
            ParseMoveError::Distance => "ParseMoveError: Distance",
        })
    }
}

impl Error for ParseMoveError {}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(1);

        Ok(Move {
            direction: left.parse().map_err(|_| ParseMoveError::Direction)?,
            distance: right[1..].parse().map_err(|_| ParseMoveError::Distance)?,
        })
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let lines: Vec<Move> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("{:?}", &lines[..5]);

    Ok(Solution::from(0, 0))
}
