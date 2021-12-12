use crate::util::ChallengeResult;

pub fn run(input: &str) -> ChallengeResult {
    let input = input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();
    ChallengeResult::new(part1(&input), part2(&input))
}

fn count_bits(input: &[u16], position: u16) -> u16 {
    input.iter().map(|x| (x >> position) & 1).sum()
}

fn part1(input: &[u16]) -> i32 {
    let length = input.len() as u16;

    let mut gamma = 0;
    for position in 0..12 {
        let count = count_bits(input, position);
        if count >= length - count {
            gamma |= 1 << position;
        }
    }

    let epsilon = !gamma & 0b111111111111;

    gamma * epsilon
}

fn part2(input: &[u16]) -> i32 {
    let ogr = life_support(input, true) as i32;
    let co2 = life_support(input, false) as i32;

    ogr * co2
}

fn life_support(input: &[u16], use_most_common_bit: bool) -> u16 {
    let mut temp = Vec::from(input);

    for position in (0..12).rev() {
        let length = temp.len() as u16;
        let bit_count = count_bits(&temp, position);

        let bit_to_keep = ((bit_count >= length - bit_count) == use_most_common_bit) as u16;
        temp.retain(|x| (*x >> position) & 1 == bit_to_keep);

        if temp.len() <= 1 {
            break;
        }
    }

    temp[0]
}
