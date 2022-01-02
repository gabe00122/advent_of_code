use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

pub fn run(input: &str) -> ChallengeResult {
    let first_line = input.lines().next().unwrap();

    let mut fish = [0u64; 9];
    for fish_age in first_line.split(',').map(|num| num.parse::<usize>().unwrap()) {
        fish[fish_age] += 1;
    }

    Ok(ChallengeSuccess::new(part1(&fish), part2(&fish)))
}

fn part1(fish: &[u64; 9]) -> u64 {
    let mut fish = *fish;

    for _ in 0..80 {
        run_simulation(&mut fish);
    }

    fish.iter().sum::<u64>()
}

fn part2(fish: &[u64; 9]) -> u64 {
    let mut fish = *fish;

    for _ in 0..256 {
        run_simulation(&mut fish);
    }

    fish.iter().sum::<u64>()
}

fn run_simulation(fish: &mut [u64; 9]) {
    let new_fish = fish[0];

    for i in 0..8 {
        fish[i] = fish[i + 1];
    }

    fish[6] += new_fish;
    fish[8] = new_fish;
}