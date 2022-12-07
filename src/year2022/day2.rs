use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;
use crate::challenge_result::{ChallengeResult, Solution};

#[derive(Debug, Copy, Clone)]
struct MoveParseError;

impl Display for MoveParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to read a left and a right move")
    }
}

impl Error for MoveParseError {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn wins_to(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Scissors => Move::Paper,
            Move::Paper => Move::Rock,
        }
    }

    fn losses_to(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Scissors => Move::Rock,
            Move::Paper => Move::Scissors,
        }
    }

    fn from(symbol: char) -> Option<Move> {
        match symbol {
            'A' | 'X' => Some(Move::Rock),
            'B' | 'Y' => Some(Move::Paper),
            'C' | 'Z' => Some(Move::Scissors),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum GameOutcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl GameOutcome {
    fn from(symbol: char) -> Option<GameOutcome> {
        match symbol {
            'X' => Some(GameOutcome::Lose),
            'Y' => Some(GameOutcome::Draw),
            'Z' => Some(GameOutcome::Win),
            _ => None,
        }
    }
}

fn game(other: Move, ours: Move) -> GameOutcome {
    if other == ours {
        GameOutcome::Draw
    } else if other.wins_to() == ours {
        GameOutcome::Lose
    } else {
        GameOutcome::Win
    }
}

fn reverse_game(other: Move, outcome: GameOutcome) -> Move {
    match outcome {
        GameOutcome::Draw => other,
        GameOutcome::Win => other.losses_to(),
        GameOutcome::Lose => other.wins_to(),
    }
}

pub fn run(input: &str) -> ChallengeResult {
    Ok(Solution::from(part1(input)?, part2(input)?))
}

fn part1(input: &str) -> Result<u64, MoveParseError> {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let mut char_iter = line.chars();
        let other_move = char_iter.next().and_then(Move::from);
        char_iter.next();
        let our_move = char_iter.next().and_then(Move::from);

        if let (Some(other_move), Some(our_move)) = (other_move, our_move) {
            let outcome = game(other_move, our_move);
            sum += our_move as u64;
            sum += outcome as u64;
        } else {
            return Err(MoveParseError);
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<u64, MoveParseError> {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let mut char_iter = line.chars();
        let other_move = char_iter.next().and_then(Move::from);
        char_iter.next();
        let outcome = char_iter.next().and_then(GameOutcome::from);

        if let (Some(other_move), Some(outcome)) = (other_move, outcome) {
            let our_move = reverse_game(other_move, outcome);
            sum += our_move as u64;
            sum += outcome as u64;
        } else {
            return Err(MoveParseError);
        }
    }

    Ok(sum)
}
