
use std::collections::BTreeSet;
use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

pub fn run(input: &str) -> ChallengeResult {
    let part1 = input.lines().map(|line| {
        let half_length = line.len() / 2;
        let first = &line[..half_length];
        let second = &line[half_length..];

        if let Some(letter) = intersection_two_ways(first, second) {
            letter_to_int(letter)
        } else {
            0
        }
    }).sum();

    let lines: Vec<&str> = input.lines().collect();
    let part2 = lines.chunks(3).map(|chunk| {
        if let Some(letter) = intersection_three_ways(chunk[0], chunk[1], chunk[2]) {
            letter_to_int(letter)
        } else {
            0
        }
    }).sum();

    Ok(ChallengeSuccess::new(part1, part2))
}

fn letter_to_int(c: char) -> u64 {
    let mut value = c as u64 - 38;
    if value > 58 {
        value -= 58
    }

    value
}

fn intersection_two_ways(first: &str, second: &str) -> Option<char> {
    let first_set = BTreeSet::from_iter(first.chars());
    for c in second.chars() {
        if first_set.contains(&c) {
            return Some(c)
        }
    }

    None
}

fn intersection_three_ways(first: &str, second: &str, third: &str) -> Option<char> {
    let first_set = BTreeSet::from_iter(first.chars());
    let second_set = BTreeSet::from_iter(second.chars());

    for c in third.chars() {
        if first_set.contains(&c) && second_set.contains(&c) {
            return Some(c)
        }
    }

    None
}