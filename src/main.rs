mod challenge_input;
mod challenge_result;
mod year2021;
mod year2022;

use std::env;
use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

struct Args {
    year: u16,
    day: u8,
}

enum ArgsError {
    NoYear,
    NoIntYear,
    NoDay,
    NonIntDay,
}

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
        Err(ArgsError::NoYear) => {
            eprintln!("Please pass in a challenge year.");
        }
        Err(ArgsError::NoIntYear) => {
            eprintln!("Please pass in a challenge year as a number.");
        }
        Err(ArgsError::NoDay) => {
            eprintln!("Please pass in a challenge day.");
        }
        Err(ArgsError::NonIntDay) => {
            eprintln!("Please pass in a challenge day as a number.");
        }
    }
}

fn run_challenge(input: &str, year: u16, day: u8) -> ChallengeResult {
    match year {
        2021 => year2021::run_challenge(input, day),
        2022 => year2022::run_challenge(input, day),
        _ => Ok(ChallengeSuccess::new(0, 0)),
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
            Err(ArgsError::NoIntYear)
        }
    } else {
        Err(ArgsError::NoYear)
    }
}
