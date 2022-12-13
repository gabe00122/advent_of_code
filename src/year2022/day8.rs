use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::challenge_result::{ChallengeResult, Solution};

const WIDTH: usize = 99;
const HEIGHT: usize = 99;

struct Trees {
    data: Box<[i8; WIDTH * HEIGHT]>,
}

impl Trees {
    fn new() -> Trees {
        Trees { data: Box::new([0; WIDTH * HEIGHT]) }
    }
}

#[derive(Debug, Copy, Clone)]
struct TreeParseError;
impl Display for TreeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tree parse error")
    }
}
impl Error for TreeParseError {}

impl FromStr for Trees {
    type Err = TreeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Trees::new();

        let mut i = 0usize;
        for line in s.lines() {
            for tree in line.chars() {
                board.data[i] = tree.to_digit(10).unwrap() as i8;
                i += 1;
            }
        }

        Ok(board)
    }
}

struct TreeVisibility {
    data: Box<[bool; WIDTH * HEIGHT]>,
}

impl TreeVisibility {
    fn new() -> TreeVisibility {
        TreeVisibility { data: Box::new([false; WIDTH * HEIGHT]) }
    }

    fn calculate_visibility_down(&mut self, trees: &Trees) {
        for x in 0..WIDTH {
            let mut highest = -1;
            for y in 0..HEIGHT {
                let index = y * WIDTH + x;
                let current = trees.data[index];

                if current > highest {
                    highest = current;
                    self.data[index] = true;
                }
            }
        }
    }

    fn calculate_visibility_up(&mut self, trees: &Trees) {
        for x in 0..WIDTH {
            let mut highest = -1;
            for y in (0..HEIGHT).rev() {
                let index = y * WIDTH + x;
                let current = trees.data[index];

                if current > highest {
                    highest = current;
                    self.data[index] = true;
                }
            }
        }
    }

    fn calculate_visibility_left(&mut self, trees: &Trees) {
        for y in 0..HEIGHT {
            let mut highest = -1;
            for x in 0..WIDTH {
                let index = y * WIDTH + x;
                let current = trees.data[index];

                if current > highest {
                    highest = current;
                    self.data[index] = true;
                }
            }
        }
    }

    fn calculate_visibility_right(&mut self, trees: &Trees) {
        for y in 0..HEIGHT {
            let mut highest = -1;
            for x in (0..WIDTH).rev() {
                let index = y * WIDTH + x;
                let current = trees.data[index];

                if current > highest {
                    highest = current;
                    self.data[index] = true;
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.data.iter().filter(| &&visible | visible).count()
    }
}

#[derive(Copy, Clone)]
struct TreeHeightNode {
    height: i8,
    position: usize,
}

struct TreeScenicScore {
    data: Box<[i32; WIDTH * HEIGHT]>,
}

impl TreeScenicScore {
    fn new() -> TreeScenicScore {
        TreeScenicScore { data: Box::new([0; WIDTH * HEIGHT]) }
    }

    fn calculate_visibility_down(&mut self, trees: &Trees) {
        let mut heights: Vec<TreeHeightNode> = Vec::new();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let index = y * WIDTH + x;
                let current = trees.data[index];

                while matches!(heights.last(), Some(last) if last.height < current) {
                    heights.pop();
                }
            }
            heights.clear();
        }
    }
}

pub fn run(input: &str) -> ChallengeResult {
    let trees: Trees = input.parse().unwrap();
    let mut tree_visibility = TreeVisibility::new();

    tree_visibility.calculate_visibility_down(&trees);
    tree_visibility.calculate_visibility_up(&trees);
    tree_visibility.calculate_visibility_left(&trees);
    tree_visibility.calculate_visibility_right(&trees);

    Ok(Solution::from(tree_visibility.count(), 0))
}