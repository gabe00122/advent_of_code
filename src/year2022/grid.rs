use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use std::iter::StepBy;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::str::FromStr;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new<F>(width: usize, height: usize, default: F) -> Grid<T>
    where
        F: Fn() -> T,
    {
        let mut data = Vec::new();
        data.resize_with(width * height, default);
        Grid {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(self.index(x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.index(x, y);
        self.data.get_mut(index)
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn iter_col(&self, col: usize, start: usize, end: usize) -> StepBy<Iter<T>> {
        let start_index = col + (start * self.width);
        let end_index = col + ((end - 1) * self.width);
        self.data[start_index..=end_index].iter().step_by(self.width)
    }

    pub fn iter_full_col(&self, col: usize) -> StepBy<Iter<T>> {
        self.iter_col(col, 0, self.height)
    }

    pub fn iter_col_mut(&mut self, col: usize, start: usize, end: usize) -> StepBy<IterMut<T>> {
        let start_index = col + (start * self.width);
        let end_index = col + ((end - 1) * self.width);
        self.data[start_index..=end_index]
            .iter_mut()
            .step_by(self.width)
    }

    pub fn iter_full_col_mut(&mut self, col: usize) -> StepBy<IterMut<T>> {
        self.iter_col_mut(col, 0, self.height)
    }

    pub fn iter_row(&self, row: usize, start: usize, end: usize) -> Iter<T> {
        let row_index = row * self.width;
        self.data[row_index + start..row_index + end].iter()
    }

    pub fn iter_full_row(&self, row: usize) -> Iter<T> {
        self.iter_row(row, 0, self.width)
    }

    pub fn iter_row_mut(&mut self, row: usize, start: usize, end: usize) -> IterMut<T> {
        let row_index = row * self.width;
        self.data[row_index + start..row_index + end].iter_mut()
    }

    pub fn iter_full_row_mut(&mut self, row: usize) -> IterMut<T> {
        self.iter_row_mut(row, 0, self.width)
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, cell) in self.data.iter().enumerate() {
            write!(f, "{}", cell)?;

            if index % self.width == self.width - 1 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParseGridError;

impl Display for ParseGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing grid data")
    }
}

impl Error for ParseGridError {}

impl<T> FromStr for Grid<T>
where
    T: FromStr,
{
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut found_width = false;
        let mut width = 0;
        let mut data: Vec<T> = Vec::new();

        let mut current = s;
        while !current.is_empty() {
            let (left, right) = current.split_at(1);
            current = right;

            if left != "\n" {
                let value: T = left.parse().map_err(|_| ParseGridError)?;
                data.push(value);

                if !found_width {
                    width += 1;
                }
            } else {
                found_width = true;
            }
        }

        Ok(Grid {
            width,
            height: data.len() / width,
            data,
        })
    }
}
