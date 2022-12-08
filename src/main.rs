mod challenge_input;
mod challenge_result;
mod year2021;
mod year2022;

use std::env;
use std::fmt;
use std::error;
use crate::challenge_result::ChallengeResult;

struct Args {
    year: u16,
    day: u8,
}

#[derive(Debug, Copy, Clone)]
enum ArgsError {
    NoYear,
    NonIntYear,
    OutOfRangeYear,
    NoDay,
    NonIntDay,
    OutOfRangeDay,
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsError::NoYear => write!(f, ""),
            ArgsError::NonIntYear => write!(f, ""),
            ArgsError::OutOfRangeYear => write!(f, ""),
            ArgsError::NoDay => write!(f, ""),
            ArgsError::NonIntDay => write!(f, ""),
            ArgsError::OutOfRangeDay => write!(f, ""),
        }
    }
}

impl error::Error for ArgsError {}

fn main() {
    match parse_args() {
        Ok(Args { year, day }) => match challenge_input::get("input", year, day) {
            Ok(input) => match run_challenge(&input, year, day) {
                Ok(result) => {
                    println!("{} : {}", result.part1, result.part2);
                }
                Err(e) => {
                    eprintln!("Challenge Error: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Challenge input could not be read.\nReason: {}", e);
            }
        },
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

fn run_challenge(input: &str, year: u16, day: u8) -> ChallengeResult {
    match year {
        2021 => year2021::run_challenge(input, day),
        2022 => year2022::run_challenge(input, day),
        _ => panic!("Not a valid year"),
    }
}

fn parse_args() -> Result<Args, ArgsError> {
    let mut args = env::args();
    args.next(); // program name

    if let Some(year) = args.next() {
        if let Ok(year) = year.parse() {
            if let Some(day) = args.next() {
                if let Ok(day) = day.parse() {
                    Ok(Args { year, day })
                } else {
                    Err(ArgsError::NonIntDay)
                }
            } else {
                Err(ArgsError::NoDay)
            }
        } else {
            Err(ArgsError::NonIntYear)
        }
    } else {
        Err(ArgsError::NoYear)
    }
}
