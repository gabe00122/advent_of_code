use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

pub fn run(input: &str) -> ChallengeResult {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    let mut stacks = parse_stacks(&lines[..8]);
    let moves = parse_moves(&lines[9..]);

    for m in moves {
        let new_size = stacks[m.from].len().saturating_sub(m.amount);
        let transfer: Vec<_> = stacks[m.from].drain(..new_size).collect();
        stacks[m.to].extend(&transfer);
    }

    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!();


    Ok(ChallengeSuccess::new(0, 0))
}



fn parse_stacks(input: &[&str]) -> Vec<Vec<char>> {
    let lines: Vec<Vec<_>> = input.iter().rev().map(| line | line.chars().collect()).collect();

    (0..9usize).map(|i| {
        let letter_position = 1 + i * 4;
        lines.iter().map(|line| {
            line[letter_position]
        }).collect()
    }).collect()
}

fn parse_moves(input: &[&str]) -> Vec<Move> {
    input.iter().map(|line| {
        let mut data = line.split(' ')
            .skip(1).step_by(2).map(| x | x.parse::<usize>().unwrap());

        Move {
            amount: data.next().unwrap(),
            from: data.next().unwrap() - 1,
            to: data.next().unwrap() - 1,
        }
    }).collect()
}