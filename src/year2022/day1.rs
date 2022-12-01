use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

pub fn run(input: &str) -> ChallengeResult {
    let mut top3 = [0, 0, 0];
    let mut local_sum = 0;

    for line in input.lines() {
        if let Ok(line_value) = line.parse::<i32>() {
            local_sum += line_value;
        } else {
            for m in &mut top3 {
                if local_sum > *m {
                    *m = local_sum;
                    break;
                }
            }

            local_sum = 0;
        }
    }

    Ok(ChallengeSuccess::new(top3[0], top3.iter().sum()))
}
