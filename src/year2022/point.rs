use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub, AddAssign, SubAssign, MulAssign, DivAssign};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Point<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Div for Point<T>
where
    T: Div<Output = T>,
{
    type Output = Point<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> AddAssign for Point<T>
    where
        T: Copy + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> SubAssign for Point<T>
where
    T: Copy + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> MulAssign for Point<T>
where
    T: Copy + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> DivAssign for Point<T>
where
    T: Copy + Div<Output = T>,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParsePointError;

impl Display for ParsePointError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse point from string")
    }
}

impl Error for ParsePointError {}

impl<T: FromStr> FromStr for Point<T> {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<T>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<T>().map_err(|_| ParsePointError)?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
