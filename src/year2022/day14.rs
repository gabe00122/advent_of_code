use crate::challenge_result::{ChallengeResult, Solution};
use crate::year2022::grid::Grid;
use crate::year2022::point::Point;
use std::fmt::{Display, Formatter, Write};

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

fn draw_line(map: &mut Grid<Cell>, from: Point<usize>, to: Point<usize>) {
    if from.x == to.x {
        let (start, end) = order(from.y, to.y);
        for cell in map.iter_col_mut(from.x, start, end + 1) {
            *cell = Cell::Wall;
        }
    } else if from.y == to.y {
        let (start, end) = order(from.x, to.x);
        for cell in map.iter_row_mut(from.y, start, end + 1) {
            *cell = Cell::Wall;
        }
    } else {
        panic!("can only draw a line where one axis aligns")
    }
}

fn order(from: usize, to: usize) -> (usize, usize) {
    if from < to {
        (from, to)
    } else {
        (to, from)
    }
}

fn parse_paths(input: &str) -> Vec<Vec<Point<usize>>> {
    input
        .lines()
        .map(|line| line.split(" -> ").map(|p| p.parse().unwrap()).collect())
        .collect()
}

fn parse_map(paths: &Vec<Vec<Point<usize>>>) -> Grid<Cell> {
    let max_y = paths
        .iter()
        .flatten()
        .map(|point| point.y)
        .max()
        .unwrap_or(0);

    let mut map: Grid<Cell> = Grid::new(1000, max_y + 2, Cell::default);
    for path in paths.iter() {
        for (&from, &to) in path.iter().zip(&path[1..]) {
            draw_line(&mut map, from, to);
        }
    }

    map
}

fn drop_sand(map: &mut Grid<Cell>, start: Point<usize>) -> Point<usize> {
    let directions = [map.width, map.width - 1, map.width + 1];
    let last_row = map.data.len() - map.width;

    let mut current = start.y * map.width + start.x;

    'outer: loop {
        if current < last_row {
            for direction in directions {
                let next = current + direction;
                if map[next] == Cell::Empty {
                    current = next;
                    continue 'outer;
                }
            }
        }
        map[current] = Cell::Sand;
        break Point::new(current % map.width, current / map.width);
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let mut paths = parse_paths(input);
    let mut map = parse_map(&paths);

    let start = Point::new(500, 0);

    let mut count = 0;
    while drop_sand(&mut map, start).y < map.height - 1 {
        count += 1;
    }
    let part1 = count;

    while drop_sand(&mut map, start) != start {
        count += 1;
    }

    // println!("{}", map);

    Ok(Solution::from(part1, count + 2))
}
