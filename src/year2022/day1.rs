use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

pub fn run(input: &str) -> ChallengeResult {
    let mut largest_sums: [u64; 3] = [0, 0, 0];

    let mut local_sum: u64 = 0;

    for line in input.lines() {
        if let Ok(line_value) = line.parse::<u64>() {
            local_sum += line_value;
        } else {
            for i in 0 .. largest_sums.len() {
                if local_sum > largest_sums[i] {
                    for j in (i + 1 .. largest_sums.len()).rev() {
                        largest_sums[j] = largest_sums[j - 1];
                    }
                    largest_sums[i] = local_sum;
                    break;
                }
            }

            local_sum = 0;
        }
    }

    Ok(ChallengeSuccess::new(largest_sums[0], largest_sums.iter().sum()))
}
