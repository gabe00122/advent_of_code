use hashbrown::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
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

fn find_adjacent_axis(value: i8) -> i8 {
    if value > 0 {
        -1
    } else if value < 0 {
        1
    } else {
        0
    }
}

fn find_adjacent_position(position: Point<i8>) -> Point<i8> {
    if position.x >= -1 && position.x <= 1 && position.y >= -1 && position.y <= 1 {
        Point::new(0, 0)
    } else {
        Point::new(
            find_adjacent_axis(position.x),
            find_adjacent_axis(position.y),
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
            relative_tail += find_adjacent_position(relative_tail - direction);
            visited.insert(head + relative_tail);
        }
    }

    visited.len()
}

fn part2(moves: &[Move]) -> usize {
    let mut visited: HashSet<Point<i8>> = HashSet::new();
    visited.insert(Point::new(0, 0));

    let mut head = Point::new(0, 0);
    let mut tails: [Point<i8>; 9] = Default::default();

    for &Move { direction, distance } in moves {
        let direction = direction.to_point();

        'step_loop: for _ in 0..distance {
            head += direction;
            let mut position = head;
            let mut delta = direction;

            for tail in tails.iter_mut() {
                let unadjacent = *tail - delta;
                delta = find_adjacent_position(unadjacent);
                *tail = unadjacent + delta;

                if delta.x == 0 && delta.y == 0 {
                    continue 'step_loop;
                }

                position += *tail;
            }

            visited.insert(position);
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

    Ok(Solution::from(part1(&moves), part2(&moves)))
}
