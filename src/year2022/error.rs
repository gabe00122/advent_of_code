use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ParseLineError<T: Error + 'static> {
    line: usize,
    cause: T,
}

impl<T: Error> ParseLineError<T> {
    pub fn new(cause: T, line: usize) -> Self {
        ParseLineError {
            cause,
            line,
        }
    }
}

impl<T : Error> Display for ParseLineError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Temp")
    }
}

impl<T : Error> Error for ParseLineError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}
