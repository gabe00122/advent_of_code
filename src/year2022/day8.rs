use crate::challenge_result::{ChallengeResult, Solution};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const SIZE: usize = 99;

struct Trees {
    data: Box<[i8; SIZE * SIZE]>,
}

impl Trees {
    fn new() -> Trees {
        Trees {
            data: Box::new([0; SIZE * SIZE]),
        }
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
    data: Box<[bool; SIZE * SIZE]>,
}

impl TreeVisibility {
    fn new() -> TreeVisibility {
        TreeVisibility {
            data: Box::new([false; SIZE * SIZE]),
        }
    }

    fn calculate_visibility<F>(&mut self, trees: &Trees, index_fn: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        for i in 0..SIZE {
            let mut highest = -1;
            for j in 0..SIZE {
                let index = index_fn(i, j);
                let current = trees.data[index];

                if current > highest {
                    highest = current;
                    self.data[index] = true;
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.data.iter().filter(|&&visible| visible).count()
    }
}

#[derive(Copy, Clone)]
struct TreeHeightNode {
    height: i8,
    position: i32,
}

struct ScenicScore {
    data: Box<[i32; SIZE * SIZE]>,
}

impl ScenicScore {
    fn new() -> ScenicScore {
        ScenicScore {
            data: Box::new([1; SIZE * SIZE]),
        }
    }

    fn calculate_score<F>(&mut self, trees: &Trees, index_fn: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        let mut heights: Vec<TreeHeightNode> = Vec::new();

        for i in 0..SIZE {
            for j in 0..SIZE {
                let index = index_fn(i, j);
                let current_height = trees.data[index];
                let current_position = j as i32;

                while matches!(heights.last(), Some(last) if last.height < current_height) {
                    heights.pop();
                }

                self.data[index] *=
                    current_position - heights.last().map_or(0, |last| last.position);
                heights.push(TreeHeightNode {
                    height: current_height,
                    position: current_position,
                })
            }
            heights.clear();
        }
    }

    fn max(&self) -> i32 {
        self.data.iter().cloned().max().unwrap()
    }
}

fn index_down(i: usize, j: usize) -> usize {
    j * SIZE + i
}

fn index_up(i: usize, j: usize) -> usize {
    (SIZE - j - 1) * SIZE + i
}

fn index_left(i: usize, j: usize) -> usize {
    i * SIZE + j
}

fn index_right(i: usize, j: usize) -> usize {
    i * SIZE + (SIZE - j - 1)
}

pub fn run(input: &str) -> ChallengeResult {
    let trees: Trees = input.parse().unwrap();

    let mut tree_visibility = TreeVisibility::new();
    tree_visibility.calculate_visibility(&trees, index_down);
    tree_visibility.calculate_visibility(&trees, index_up);
    tree_visibility.calculate_visibility(&trees, index_left);
    tree_visibility.calculate_visibility(&trees, index_right);

    let mut scenic_score = ScenicScore::new();
    scenic_score.calculate_score(&trees, index_down);
    scenic_score.calculate_score(&trees, index_up);
    scenic_score.calculate_score(&trees, index_left);
    scenic_score.calculate_score(&trees, index_right);

    Ok(Solution::from(tree_visibility.count(), senic_score.max()))
}
