use crate::Error;

/// Place in the source: byte offset, and line and column counted from 1
///
/// ```rust
/// use kuku::*;
///
/// let mut scanner = Scanner::new("가\nb");
/// scanner.advance();
/// scanner.advance();
///
/// assert_eq!(scanner.position(), Position { offset: 4, line: 2, column: 1 });
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    source: &'a str,
    position: Position,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Self {
            source,
            position: Position {
                offset: 0,
                line: 1,
                column: 1,
            },
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn offset(&self) -> usize {
        self.position.offset
    }

    pub fn line(&self) -> usize {
        self.position.line
    }

    pub fn column(&self) -> usize {
        self.position.column
    }

    pub fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    pub fn peek_at(&self, at: usize) -> Option<char> {
        self.rest().chars().nth(at)
    }

    pub fn advance(&mut self) -> Option<char> {
        let char = self.peek()?;

        self.position.offset += char.len_utf8();
        if char == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }

        Some(char)
    }

    pub fn rest(&self) -> &'a str {
        &self.source[self.position.offset..]
    }

    /// Source consumed since the given position
    ///
    /// ```rust
    /// use kuku::*;
    ///
    /// let mut scanner = Scanner::new("let x");
    /// let start = scanner.position();
    /// string::parse("let")(&mut scanner).unwrap();
    ///
    /// assert_eq!(scanner.since(start), "let");
    /// ```
    pub fn since(&self, start: Position) -> &'a str {
        &self.source[start.offset..self.position.offset]
    }

    pub fn branch(&self) -> Position {
        self.position
    }

    pub fn restore(&mut self, position: Position) {
        self.position = position;
    }

    /// Error at the current position, with the next character as what was found
    ///
    /// ```rust
    /// use kuku::*;
    ///
    /// let scanner = Scanner::new("x");
    ///
    /// assert_eq!(scanner.expected("digit").to_string(), "1:1: expected digit, got 'x'");
    /// ```
    pub fn expected(&self, expected: impl Into<String>) -> Error {
        Error::expected(self.position, expected, self.found())
    }

    /// Error at the current position with a message of its own
    ///
    /// ```rust
    /// use kuku::*;
    ///
    /// let scanner = Scanner::new("x");
    ///
    /// assert_eq!(scanner.error("no x here").to_string(), "1:1: no x here");
    /// ```
    pub fn error(&self, message: impl Into<String>) -> Error {
        Error::message(self.position, message)
    }

    pub fn found(&self) -> String {
        match self.peek() {
            Some(char) => format!("'{char}'"),
            None => "end of input".into(),
        }
    }
}
