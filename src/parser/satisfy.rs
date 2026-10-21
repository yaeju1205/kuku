use std::fmt::Display;

use crate::{Error, Parser, Scanner};

/// Single character matching the predicate
///
/// The description is only turned into text when the parser fails
///
/// ```rust
/// use kuku::*;
///
/// let digit = satisfy::parse("digit", |char| char.is_ascii_digit());
///
/// assert_eq!(parse("7", &digit), Ok('7'));
/// assert_eq!(parse("x", &digit).unwrap_err().to_string(), "1:1: expected digit, got 'x'");
/// ```
pub fn parse(description: impl Display, predicate: impl Fn(char) -> bool) -> impl Parser<char> {
    move |scanner: &mut Scanner| next_if(scanner, &predicate, || description.to_string())
}

pub(crate) fn next_if(
    scanner: &mut Scanner,
    predicate: impl Fn(char) -> bool,
    description: impl FnOnce() -> String,
) -> Result<char, Error> {
    match scanner.peek() {
        Some(char) if predicate(char) => {
            scanner.advance();
            Ok(char)
        }
        _ => Err(scanner.expected(description())),
    }
}
