use std::fmt;
use termion::color;

/// Error: message, line and column
pub struct JSONError {
    msg: String,
    line: usize,
    col: usize,
}

impl fmt::Debug for JSONError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}:{}{} - {}",
            color::Fg(color::Red),
            self.line,
            self.col,
            color::Fg(color::Reset),
            self.msg
        )
    }
}

impl JSONError {
    /// Create a new json error
    pub fn new(msg: String, line: usize, col: usize) -> Self {
        Self { msg, line, col }
    }
}
