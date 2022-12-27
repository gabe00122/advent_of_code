use crate::challenge_result::{ChallengeResult, Solution};
use crate::year2022::grid::Grid;

fn visibility(trees: &Grid<i32>) -> Grid<bool> {
    let mut visibility = Grid::new(trees.width, trees.height, || false);

    for x in 0..trees.width {
        //down
        visibility_line(
            trees.iter_full_col(x),
            visibility.iter_full_col_mut(x)
        );
        //up
        visibility_line(
            trees.iter_full_col(x).rev(),
            visibility.iter_full_col_mut(x).rev(),
        );
    }

    for y in 0..trees.height {
        //left
        visibility_line(
            trees.iter_full_row(y),
            visibility.iter_full_row_mut(y)
        );
        //right
        visibility_line(
            trees.iter_full_row(y).rev(),
            visibility.iter_full_row_mut(y).rev(),
        );
    }

    visibility
}

fn visibility_line<'a, 'b, T, V>(trees: T, visible: V)
where
    T: Iterator<Item = &'a i32>,
    V: Iterator<Item = &'b mut bool>,
{
    trees.zip(visible).fold(-1, |highest, (&current, out)| {
        if current > highest {
            *out = true;
            current
        } else {
            highest
        }
    });
}

fn scenic_score(trees: &Grid<i32>) -> Grid<i32> {
    let mut scenic = Grid::new(trees.width, trees.height, || 1);
    let mut heights: Vec<TreeHeightNode> = Vec::new();

    for x in 0..trees.width {
        //down
        scenic_score_line(
            trees.iter_full_col(x),
            scenic.iter_full_col_mut(x),
            &mut heights,
        );
        //up
        scenic_score_line(
            trees.iter_full_col(x).rev(),
            scenic.iter_full_col_mut(x).rev(),
            &mut heights,
        );
    }

    for y in 0..trees.height {
        //left
        scenic_score_line(
            trees.iter_full_row(y),
            scenic.iter_full_row_mut(y),
            &mut heights,
        );
        //right
        scenic_score_line(
            trees.iter_full_row(y).rev(),
            scenic.iter_full_row_mut(y).rev(),
            &mut heights,
        );
    }

    scenic
}

#[derive(Copy, Clone)]
struct TreeHeightNode {
    height: i32,
    position: i32,
}

fn scenic_score_line<'a, 'b, T, S>(trees: T, scenic: S, heights: &mut Vec<TreeHeightNode>)
where
    T: Iterator<Item = &'a i32>,
    S: Iterator<Item = &'b mut i32>,
{
    for (position, (&current, out)) in trees.zip(scenic).enumerate() {
        let position = position as i32;

        while matches!(heights.last(), Some(last) if last.height < current) {
            heights.pop();
        }

        *out *= position - heights.last().map_or(0, |last| last.position);
        heights.push(TreeHeightNode {
            height: current,
            position,
        });
    }

    heights.clear();
}

pub fn run(input: &str) -> ChallengeResult {
    //Time: 600µs

    let trees: Grid<i32> = input.parse()?;

    //part1
    let visibility = visibility(&trees);
    let count = visibility.iter().cloned().filter(|&value| value).count();

    //part2
    let scenic = scenic_score(&trees);
    let max = scenic.iter().cloned().max().unwrap_or(0);

    Ok(Solution::from(count, max))
}
