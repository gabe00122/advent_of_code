use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

pub fn run(input: &str) -> ChallengeResult {
    let mut largest_sums: [u64; 3] = [0, 0, 0];
    let mut local_sum: u64 = 0;

    for line in input.lines() {
        if let Ok(line_value) = line.parse::<u64>() {
            local_sum += line_value;
        } else {
            if local_sum > largest_sums[0] {
                largest_sums[2] = largest_sums[1];
                largest_sums[1] = largest_sums[0];
                largest_sums[0] = local_sum;
            } else if local_sum > largest_sums[1] {
                largest_sums[2] = largest_sums[1];
                largest_sums[1] = local_sum;
            } else if local_sum > largest_sums[2] {
                largest_sums[2] = local_sum;
            }

            local_sum = 0;
        }
    }

    Ok(ChallengeSuccess::new(largest_sums[0], largest_sums.iter().sum()))
}
