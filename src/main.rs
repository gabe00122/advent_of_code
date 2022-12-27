mod challenge_input;
mod challenge_result;
mod year2021;
mod year2022;

use std::env;
use std::fmt;
use std::error;
use std::time::Instant;
use crate::challenge_result::ChallengeResult;

struct Args {
    year: u16,
    day: u8,
}

#[derive(Debug, Copy, Clone)]
enum ArgsError {
    NoYear,
    NonIntYear,
    NoDay,
    NonIntDay,
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgsError::NoYear => write!(f, "Please pass a year in."),
            ArgsError::NonIntYear => write!(f, "A year must be a number."),
            ArgsError::NoDay => write!(f, "Please pass a day in."),
            ArgsError::NonIntDay => write!(f, "A day must be a number."),
        }
    }
}

impl error::Error for ArgsError {}

fn main() {
    match parse_args() {
        Ok(Args { year, day }) => match challenge_input::get("input", year, day) {
            Ok(input) => {
                let start = Instant::now();
                match run_challenge(&input, year, day) {
                    Ok(result) => {
                        let duration = start.elapsed();
                        println!("{} : {}\nDuration: {:?}", result.part1, result.part2, duration);
                    }
                    Err(e) => {
                        eprintln!("Challenge Error: {}", e);
                    }
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
