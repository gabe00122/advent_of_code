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

#[derive(Debug, Clone)]
struct Simulation<'a, I>
where
    I: Iterator<Item = &'a Command>
{
    x: i32,
    commands: I,
    pending_addx: Option<i32>,
}

impl<'a, I> Simulation<'a, I>
where
    I: Iterator<Item = &'a Command>
{
    fn new(commands: impl IntoIterator<IntoIter = I>) -> Self {
        Simulation { x: 1, commands: commands.into_iter(), pending_addx: None }
    }
}

impl<'a, I> Iterator for Simulation<'a, I>
where
    I: Iterator<Item = &'a Command>
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(amount) = self.pending_addx {
            let old_x = self.x;
            self.x += amount;
            self.pending_addx = None;
            Some(old_x)
        } else if let Some(&command) = self.commands.next() {
            match command {
                Command::NoOp => {},
                Command::AddX(amount) => {
                    self.pending_addx = Some(amount)
                }
            }
            Some(self.x)
        } else {
            None
        }
    }
}

fn part1(x_values: &[i32]) -> i32 {
    x_values.iter().zip(1..)
        .map(|(x, cycle)| x * cycle)
        .skip(19)
        .step_by(40)
        .sum()
}

fn part2(x_values: &[i32]) {
    let mut iter = x_values.iter();

    for _ in 0..6 {
        for col in 0..40 {
            if let Some(&x) = iter.next() {
                if col >= x - 1 && col <= x + 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let commands: Vec<Command> = input.lines()
        .enumerate()
        .map(|(i, line)|
            line.parse::<Command>().map_err(|error| ParseLineError::new(i, error))
        ).collect::<Result<_, _>>()?;

    let x_values: Vec<i32> = Simulation::new(&commands).collect();

    part2(&x_values);

    Ok(Solution::from(part1(&x_values), 0))
}
