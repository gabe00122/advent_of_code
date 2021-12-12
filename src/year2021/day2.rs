use crate::util::ChallengeResult;
use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}

struct MoveInstruction {
    direction: Direction,
    length: i32,
}

pub fn run(input: &str) -> ChallengeResult {
    let input: Vec<MoveInstruction> = input
        .lines()
        .map(|line| {
            let direction = Direction::from_str(&line[..line.len() - 2]).unwrap();
            let length: i32 = line[line.len() - 1..].parse().unwrap();

            MoveInstruction { direction, length }
        })
        .collect();

    ChallengeResult::new(part1(&input), part2(&input))
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
