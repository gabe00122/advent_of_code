use crate::challenge_result::{ChallengeResult, ChallengeSuccess};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn from_str(input: &str) -> Self {
        let mut iter = input.split(',').flat_map(str::parse);

        Self {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn from_str(input: &str) -> Self {
        let mut iter = input.split(" -> ").map(Point::from_str);

        Self {
            from: iter.next().unwrap(),
            to: iter.next().unwrap(),
        }
    }

    fn not_diagonal(&self) -> bool {
        self.to.x == self.from.x || self.to.y == self.from.y
    }
}

struct Board {
    data: Vec<u8>,
    width: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            data: vec![0; width * height],
            width,
        }
    }

    fn count_doubles(&self) -> usize {
        self.data.iter().filter(|&&a| a >= 2).count()
    }

    fn index(&self, x: i16, y: i16) -> usize {
        (y as usize) * self.width + (x as usize)
    }

    fn draw(&mut self, line: &Line) {
        let dx = compare(line.from.x, line.to.x);
        let dy = compare(line.from.y, line.to.y);

        let mut x = line.from.x;
        let mut y = line.from.y;

        while x != line.to.x || y != line.to.y {
            let index = self.index(x, y);
            self.data[index] += 1;
            x += dx;
            y += dy;
        }

        let index = self.index(x, y);
        self.data[index] += 1;
    }
}

fn compare(a: i16, b: i16) -> i16 {
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let input: Vec<Line> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(Line::from_str)
        .collect();

    Ok(ChallengeSuccess::new(part1(&input), part2(&input)))
}

fn part1(input: &[Line]) -> u64 {
    let mut board = Board::new(1000, 1000);
    for line in input {
        if line.not_diagonal() {
            board.draw(line);
        }
    }

    board.count_doubles() as u64
}

fn part2(input: &[Line]) -> u64 {
    let mut board = Board::new(1000, 1000);
    for line in input {
        board.draw(line);
    }

    board.count_doubles() as u64
}
