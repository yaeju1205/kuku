use std::fmt;

use crate::Position;

/// Why and where a parser failed
///
/// A fatal error is not caught by choice, many, optional or separated, see cut
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub position: Position,
    pub reason: Reason,
    pub fatal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    Expected {
        expected: Vec<String>,
        found: String,
    },
    Message(String),
}

impl Error {
    pub fn expected(
        position: Position,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self {
            position,
            reason: Reason::Expected {
                expected: vec![expected.into()],
                found: found.into(),
            },
            fatal: false,
        }
    }

    pub fn message(position: Position, message: impl Into<String>) -> Self {
        Self {
            position,
            reason: Reason::Message(message.into()),
            fatal: false,
        }
    }

    pub fn fatal(self) -> Self {
        Self {
            fatal: true,
            ..self
        }
    }

    /// The error that got further, joining what both expected when they stopped at the same place
    ///
    /// ```rust
    /// use kuku::*;
    ///
    /// let start = Scanner::new("x").position();
    /// let merged = Error::expected(start, "'a'", "'x'").merge(Error::expected(start, "'b'", "'x'"));
    ///
    /// assert_eq!(merged.to_string(), "1:1: expected 'a' or 'b', got 'x'");
    /// ```
    pub fn merge(self, other: Error) -> Error {
        if other.position.offset != self.position.offset {
            return if other.position.offset > self.position.offset {
                other
            } else {
                self
            };
        }

        match (self.reason, other.reason) {
            (
                Reason::Expected {
                    mut expected,
                    found,
                },
                Reason::Expected {
                    expected: others, ..
                },
            ) => {
                for other in others {
                    if !expected.contains(&other) {
                        expected.push(other);
                    }
                }

                Error {
                    position: self.position,
                    reason: Reason::Expected { expected, found },
                    fatal: self.fatal || other.fatal,
                }
            }
            (reason, _) => Error {
                position: self.position,
                reason,
                fatal: self.fatal,
            },
        }
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reason::Expected { expected, found } => {
                write!(f, "expected {}, got {found}", expected.join(" or "))
            }
            Reason::Message(message) => f.write_str(message),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.position.line, self.position.column, self.reason
        )
    }
}

impl std::error::Error for Error {}
