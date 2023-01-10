use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ParseLineError<T: Error + 'static> {
    line: usize,
    cause: T,
}

impl<T: Error + 'static> ParseLineError<T> {
    pub fn new(line: usize, cause: T) -> Self {
        ParseLineError { line, cause }
    }
}

impl<T : Error> Display for ParseLineError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" on line {}", self.cause, self.line)
    }
}

impl<T : Error> Error for ParseLineError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}
