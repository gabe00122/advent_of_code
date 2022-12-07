use crate::challenge_result::{ChallengeResult, Solution};

#[derive(Clone, Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

pub fn run(input: &str) -> ChallengeResult {
    let lines: Vec<&str> = input.lines().collect();

    let stacks = parse_stacks(&lines[..8]);
    let moves = parse_moves(&lines[10..]);


    Ok(Solution::new(part1(&stacks, &moves), part2(&stacks, &moves)))
}

fn part1(stacks: &Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();

    let mut transfer = Vec::new();
    for current_move in moves.iter() {
        let from = &mut stacks[current_move.from];
        let new_size = from.len().saturating_sub(current_move.amount);
        transfer.extend(from.drain(new_size..).rev());

        stacks[current_move.to].append(&mut transfer);
    }

    stacks.iter().flat_map(| stack | stack.last()).collect()
}

fn part2(stacks: &Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();

    let mut transfer = Vec::new();
    for current_move in moves.iter() {
        let from = &mut stacks[current_move.from];
        let new_size = from.len().saturating_sub(current_move.amount);
        transfer.extend(from.drain(new_size..));

        stacks[current_move.to].append(&mut transfer);
    }

    stacks.iter().flat_map(| stack | stack.last()).collect()
}

fn parse_stacks(input: &[&str]) -> Vec<Vec<char>> {
    let lines: Vec<Vec<_>> = input
        .iter()
        .rev()
        .map(|line| line.chars().skip(1).step_by(4).collect())
        .collect();

    (0..9)
        .map(|i| {
            lines
                .iter()
                .map(|line| line[i])
                .filter(|&c| c != ' ')
                .collect()
        })
        .collect()
}

fn parse_moves(input: &[&str]) -> Vec<Move> {
    input
        .iter()
        .map(|line| {
            let mut data = line
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap());

            Move {
                amount: data.next().unwrap(),
                from: data.next().unwrap() - 1,
                to: data.next().unwrap() - 1,
            }
        })
        .collect()
}
