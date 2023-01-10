use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter, write};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::challenge_result::{ChallengeResult, Solution};
use crate::year2022::error::ParseLineError;
use crate::year2022::point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_point(&self) -> Point<i8> {
        match self {
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ParseDirectionError;

impl Display for ParseDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "direction must be 'U', 'D', 'L' or 'R'")
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
    distance: i8,
}

#[derive(Debug, Clone)]
enum ParseMoveError {
    Direction(ParseDirectionError),
    Distance(ParseIntError),
}

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseMoveError::Direction(err) => write!(f, "{}", err),
            ParseMoveError::Distance(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ParseMoveError {}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(1);

        Ok(Move {
            direction: left.parse().map_err(ParseMoveError::Direction)?,
            distance: right[1..].parse().map_err(ParseMoveError::Distance)?,
        })
    }
}

fn make_axis_adjacent(value: i8) -> i8 {
    if value > 0 {
        value - 1
    } else if value < 0 {
        value + 1
    } else {
        0
    }
}

fn make_position_adjacent(position: Point<i8>) -> Point<i8> {
    if position.x >= -1 && position.x <= 1 && position.y >= -1 && position.y <= 1 {
        position
    } else {
        Point::new(
            make_axis_adjacent(position.x),
            make_axis_adjacent(position.y),
        )
    }
}

fn part1(moves: &[Move]) -> usize {
    let mut visited: HashSet<Point<i8>> = HashSet::new();

    let mut head = Point::new(0, 0);
    let mut relative_tail = Point::new(0, 0);

    for &Move { direction, distance } in moves {
        let direction = direction.to_point();

        for _ in 0..distance {
            head += direction;
            relative_tail = make_position_adjacent(relative_tail - direction);

            visited.insert(head + relative_tail);
        }
    }

    visited.len()
}

pub fn run(input: &str) -> ChallengeResult {
    let moves: Vec<Move> = input
        .lines()
        .enumerate()
        .map(|(i, line) | line.parse().map_err(| err |
            ParseLineError::new(i, err)
        ))
        .collect::<Result<_, _>>()?;

    Ok(Solution::from(part1(&moves), 0))
}
