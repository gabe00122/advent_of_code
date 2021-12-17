mod challenge_input;
mod challenge_result;
mod year2021;

use std::env;

struct Args {
    day: u8,
}

enum ArgsError {
    NoDay,
    NonIntDay,
}

fn main() {
    let year = 2021; // hard coded

    match parse_args() {
        Ok(Args { day }) => match challenge_input::get("input", year, day) {
            Ok(input) => match year2021::run_challenge(&input, day) {
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
        Err(ArgsError::NoDay) => {
            eprintln!("Please pass in a challenge day.");
        }
        Err(ArgsError::NonIntDay) => {
            eprintln!("Please pass in a challenge day as a number.");
        }
    }
}

fn parse_args() -> Result<Args, ArgsError> {
    let mut args = env::args();
    args.next(); // program name

    if let Some(day) = args.next() {
        if let Ok(day) = day.parse() {
            Ok(Args { day })
        } else {
            Err(ArgsError::NonIntDay)
        }
    } else {
        Err(ArgsError::NoDay)
    }
}
