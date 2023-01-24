use crate::challenge_result::{ChallengeResult, Solution};
use crate::year2022::grid::{FromChar, Grid};

#[derive(Debug, Copy, Clone)]
enum Cell {
    Start,
    End,
    Height(i8),
}

impl FromChar for Cell {
    type Error = ();

    fn from_char(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            o if 'a' <= o && o <= 'z' => Ok(Cell::Height(o as i8 - 'a' as i8)),
            _ => Err(()),
        }
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let grid: Grid<Cell> = input.parse()?;
    Ok(Solution::from(grid.width, grid.height + grid.data.len()))
}
