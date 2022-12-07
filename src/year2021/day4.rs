use crate::challenge_result::{ChallengeResult, Solution};
use std::num::ParseIntError;

const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct Board {
    data: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Board { data: [[0; BOARD_SIZE]; BOARD_SIZE] }
    }
}

#[derive(Debug, Clone)]
struct BoardChecks {
    data: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl BoardChecks {
    fn new() -> Self {
        BoardChecks {
            data: [[false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn call_number(&mut self, board: &Board, num: u8) -> bool {
        let mut any_flipped = false;

        for (check_row, board_row) in self.data.iter_mut().zip(board.data.iter()) {
            for (check_cell, &board_cell) in check_row.iter_mut().zip(board_row.iter()) {
                if !*check_cell && board_cell == num {
                    any_flipped = true;
                    *check_cell = true;
                }
            }
        }

        any_flipped
    }

    fn is_win(&self) -> bool {
        self.data.iter().any(|row| row.iter().all(|&cell| cell))
            || (0..BOARD_SIZE).any(|i| self.data.iter().all(|row| row[i]))
            || (0..BOARD_SIZE).all(|i| self.data[i][i])
            || (0..BOARD_SIZE).all(|i| self.data[BOARD_SIZE - i - 1][i])
    }

    fn sum_unmarked(&self, board: &Board) -> u64 {
        self.data
            .iter()
            .zip(board.data.iter())
            .map(|(check_row, num_row)| {
                check_row
                    .iter()
                    .zip(num_row.iter())
                    .filter(|(&check, _)| !check)
                    .map(|(_, &num)| num as u64)
                    .sum::<u64>()
            })
            .sum::<u64>()
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<u8> = lines[0]
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let boards = lines[1..]
        .chunks(6)
        .map(|window| parse_bingo(&window[1..]))
        .collect::<Result<Vec<Board>, ParseIntError>>()?;

    Ok(Solution::from(
        part1(&numbers, &boards),
        part2(&numbers, &boards),
    ))
}

fn part1(numbers: &[u8], boards: &[Board]) -> u64 {
    let mut check_state = Vec::with_capacity(boards.len());
    for _ in 0..check_state.capacity() {
        check_state.push(BoardChecks::new());
    }

    let mut winning_index: usize = 0;
    let mut winning_number: u64 = 0;

    'top: for &num in numbers {
        for (index, (checks, board)) in check_state.iter_mut().zip(boards.iter()).enumerate() {
            if checks.call_number(board, num) && checks.is_win() {
                winning_index = index;
                winning_number = num as u64;
                break 'top;
            }
        }
    }

    let winning_check = &check_state[winning_index];
    let winning_board = &boards[winning_index];

    winning_check.sum_unmarked(winning_board) * winning_number
}

fn part2(numbers: &[u8], boards: &[Board]) -> u64 {
    let mut check_state = vec![BoardChecks::new(); boards.len()];

    let mut winning_index: usize = 0;
    let mut winning_number: u64 = 0;
    let mut winning_check = BoardChecks::new();

    for &num in numbers {
        for (index, (checks, board)) in check_state.iter_mut().zip(boards.iter()).enumerate() {
            if !checks.is_win() && checks.call_number(board, num) && checks.is_win() {
                winning_index = index;
                winning_number = num as u64;
                winning_check = checks.clone()
            }
        }
    }

    let winning_board = &boards[winning_index];

    // print_board_with_checks(winning_board, &winning_check);

    winning_check.sum_unmarked(winning_board) * winning_number
}

fn parse_bingo(lines: &[&str]) -> Result<Board, ParseIntError> {
    let mut board = Board::new();

    for (row, &line) in board.data.iter_mut().zip(lines) {
        for (cell, str_cell) in row.iter_mut().zip(line.split_whitespace()) {
            *cell = str_cell.parse()?;
        }
    }

    Ok(board)
}

// fn create_lookup_table(boards: &[Board]) -> Vec<Vec<usize>> {
//     let mut lookup_table: Vec<Vec<usize>> = Vec::with_capacity(100);
//     for _ in 0..lookup_table.capacity() {
//         lookup_table.push(Vec::new());
//     }
//
//     for (i, board) in boards.iter().enumerate() {
//         for row in board.data {
//             for cell in row {
//                 let p = &mut lookup_table[cell as usize];
//                 if !p.last().eq(&i) {
//                     p.push(i);
//                 }
//             }
//         }
//     }
//
//     lookup_table
// }

// fn print_board(board: &Board) {
//     for row in board.data.iter() {
//         for &cell in row.iter() {
//             if cell < 10 {
//                 print!(" ");
//             }
//             print!("{} ", cell);
//         }
//         println!();
//     }
// }
//
// fn print_board_with_checks(board: &Board, checks: &BoardChecks) {
//     for (row, row_checks) in board.data.iter().zip(checks.data.iter()) {
//         for (&cell, &check) in row.iter().zip(row_checks.iter()) {
//             if !check {
//                 if cell < 10 {
//                     print!(" ");
//                 }
//                 print!("{} ", cell);
//             } else {
//                 print!(" X ");
//             }
//         }
//         println!();
//     }
// }
