pub struct ChallengeResult {
    pub part1: i32,
    pub part2: i32,
}

impl ChallengeResult {
    pub fn new(part1: i32, part2: i32) -> Self {
        ChallengeResult { part1, part2 }
    }
}
