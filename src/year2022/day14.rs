use std::fmt::{Display, Formatter, Write};
use crate::challenge_result::{ChallengeResult, Solution};
use crate::year2022::grid::Grid;
use crate::year2022::point::Point;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Sand,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Empty => '.',
            Cell::Sand => 'O',
            Cell::Wall => '#',
        })
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let paths: Vec<Vec<Point<i32>>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|p| p.parse())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Line failed to parse");

    let mut grid: Grid<Cell> = Grid::new(10, 10);

    for cell in grid.iter_col_mut(5, 2, 8) {
        *cell = Cell::Sand;
    }

    println!("{}", grid);

    Ok(Solution::from(0, 0))
}
