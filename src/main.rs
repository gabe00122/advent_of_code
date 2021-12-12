mod challenge_input;
mod util;
mod year2021;

use std::env;
use std::io;

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let day = args.next();

    let year = 2021;
    let day = day.unwrap().parse().unwrap(); // temp

    let input = challenge_input::get("input", year, day)?;

    let result = year2021::run_challenge(&input, day);
    println!("{} : {}", result.part1, result.part2);

    Ok(())
}
