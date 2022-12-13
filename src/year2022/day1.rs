use crate::challenge_result::{ChallengeResult, Solution};

pub fn run(input: &str) -> ChallengeResult {
    let mut largest_sums: [i32; 3] = [0, 0, 0];
    let mut local_sum: i32 = 0;

    for line in input.lines() {
        if let Ok(line_value) = line.parse::<i32>() {
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

    Ok(Solution::from(largest_sums[0], largest_sums.iter().sum::<i32>()))
}
