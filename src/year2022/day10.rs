use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::challenge_result::{ChallengeResult, Solution};
use crate::year2022::error::ParseLineError;

#[derive(Debug, Copy, Clone)]
enum Command {
    NoOp,
    AddX(i32),
}

#[derive(Debug, Copy, Clone)]
struct ParseCommandError;

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "command must be noop or addx")
    }
}

impl Error for ParseCommandError {}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, arg) = s.split_at(4);

        match command {
            "noop" => Ok(Command::NoOp),
            "addx" => Ok(Command::AddX(
                arg.trim_start()
                    .parse()
                    .map_err(|_| ParseCommandError)?
            )),
            _ => Err(ParseCommandError),
        }
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let input: Vec<Command> = input.lines()
        .enumerate()
        .map(|(i, line)|
            line.parse::<Command>().map_err(|error| ParseLineError::new(i, error))
        ).collect::<Result<_, _>>()?;

    println!("{:?}", &input[..7]);

    Ok(Solution::from(0, 0))
}
