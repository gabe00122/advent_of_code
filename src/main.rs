mod challenge_input;
mod util;
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
        Ok(Args { day }) => {
            if let Ok(input) = challenge_input::get("input", year, day) {
                let result = year2021::run_challenge(&input, day);
                println!("{} : {}", result.part1, result.part2);
            } else { // capture details
                eprintln!("Challenge input could not be read.");
            }
        }
        Err(ArgsError::NoDay) => {
            eprintln!("Please pass in a challenge day.")
        }
        Err(ArgsError::NonIntDay) => {
            eprintln!("Please pass in a challenge day as a number.")
        }
    }
}

fn parse_args() -> Result<Args, ArgsError> {
    let mut args = env::args();
    args.next(); // program name
    let raw_day = args.next();

    if let Some(day) = raw_day {
        if let Ok(day) = day.parse() {
            Ok(Args {
                day,
            })
        } else {
            Err(ArgsError::NonIntDay)
        }
    } else {
        Err(ArgsError::NoDay)
    }
}