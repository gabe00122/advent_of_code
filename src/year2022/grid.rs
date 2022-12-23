use std::fmt::{Display, Formatter, Write};
use std::iter::StepBy;
use std::slice::{Iter, IterMut};
use std::str::FromStr;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T>
    where
        T: Default,
    {
        let mut data = Vec::new();
        data.resize_with(width * height, T::default);
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
        if col < self.width {
            let start_index = col + (start * self.width);
            let end_index = col + (end * self.width);
            self.data[start_index..end_index].iter().step_by(self.width)
        } else {
            panic!(
                "out of bounds. Column must be less than {:?}, but is {:?}.",
                self.width, col
            )
        }
    }

    pub fn iter_full_col(&self, col: usize) -> StepBy<Iter<T>> {
        self.iter_col(col, 0, self.height)
    }

    pub fn iter_col_mut(&mut self, col: usize, start: usize, end: usize) -> StepBy<IterMut<T>> {
        if col < self.width {
            let start_index = col + (start * self.width);
            let end_index = col + (end * self.width);
            self.data[start_index..end_index]
                .iter_mut()
                .step_by(self.width)
        } else {
            panic!(
                "out of bounds. Column must be less than {:?}, but is {:?}.",
                self.width, col
            )
        }
    }

    pub fn iter_full_col_mut(&mut self, col: usize) -> StepBy<IterMut<T>> {
        self.iter_col_mut(col, 0, self.height)
    }

    pub fn iter_row(&self, row: usize, start: usize, end: usize) -> Iter<T> {
        if row < self.height {
            let row_index = row * self.width;
            self.data[row_index + start..row_index + end].iter()
        } else {
            panic!(
                "out of bounds. Row must be less than {:?}, but is {:?}.",
                self.height, row
            )
        }
    }

    pub fn iter_full_row(&self, row: usize) -> Iter<T> {
        self.iter_row(row, 0, self.width)
    }

    pub fn iter_row_mut(&mut self, row: usize, start: usize, end: usize) -> IterMut<T> {
        if row < self.height {
            let row_index = row * self.width;
            self.data[row_index + start..row_index + end].iter_mut()
        } else {
            panic!(
                "out of bounds. Row must be less than {:?}, but is {:?}.",
                self.height, row
            )
        }
    }

    pub fn iter_full_row_mut(&mut self, row: usize) -> IterMut<T> {
        self.iter_row_mut(row, 0, self.width)
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

impl FromStr for Grid<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.chars()
            .enumerate()
            .find(|&(_, c)| c == '\n')
            .map_or(1, |(width, _)| width);

        let data: Vec<char> = s.chars().filter(|&c| c != '\n').collect();
        Ok(Grid {
            width,
            height: data.len() / width,
            data,
        })
    }
}
