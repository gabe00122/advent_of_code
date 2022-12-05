use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

#[derive(Copy, Clone)]
struct Range {
    start: u32,
    finish: u32,
}

impl Range {
    fn new(start: u32, finish: u32) -> Range {
        Range { start, finish }
    }

    fn contains(&self, other: Range) -> bool {
        self.start <= other.start && self.finish >= other.finish
    }

    fn overlaps(&self, other: Range) -> bool {
        (self.start <= other.start && other.start <= self.finish) ||
            (other.start <= self.start && self.start <= other.finish)
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let data: Vec<_> = input.lines().map(|line| {
        let mut ranges = line.split(",").map(| range | {
            let mut parts = range.split("-").flat_map(| part | part.parse::<u32>().ok());
            Range::new(parts.next().unwrap(), parts.next().unwrap())
        });

        (ranges.next().unwrap(), ranges.next().unwrap())
    }).collect();

    let part1 = data.iter().filter(|(first, second)| {
        first.contains(*second) || second.contains(*first)
    }).count();

    let part2 = data.iter().filter(|(first, second)| {
        first.overlaps(*second)
    }).count();

    Ok(ChallengeSuccess::new(part1 as u64, part2 as u64))
}
