use crate::util::ChallengeResult;

pub fn run(input: &str) -> ChallengeResult {
    let input: Vec<i32> = input.lines().map(str::parse).flatten().collect();
    ChallengeResult::new(part1(&input), part2(&input))
}

fn part1(input: &[i32]) -> i32 {
    input
        .iter()
        .zip(input[1..].iter())
        .map(|(a, b)| (a < b) as i32)
        .sum()
}

fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .zip(input[1..].iter())
        .zip(input[2..].iter())
        .zip(input[3..].iter())
        .map(|(((a, b), c), d)| (a + b + c < b + c + d) as i32)
        .sum()
}
