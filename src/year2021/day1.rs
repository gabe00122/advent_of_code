use crate::challenge_result::{ChallengeResult, Solution};

pub fn run(input: &str) -> ChallengeResult {
    let input: Vec<u64> = input.lines().map(str::parse).flatten().collect();
    Ok(Solution::from(part1(&input), part2(&input)))
}

fn part1(input: &[u64]) -> u64 {
    input
        .iter()
        .zip(input[1..].iter())
        .map(|(a, b)| (a < b) as u64)
        .sum()
}

fn part2(input: &[u64]) -> u64 {
    input
        .iter()
        .zip(input[1..].iter())
        .zip(input[2..].iter())
        .zip(input[3..].iter())
        .map(|(((a, _), _), d)| (a < d) as u64)
        .sum()
}
