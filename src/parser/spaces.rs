use crate::{Error, Scanner};

/// Zero or more whitespace characters
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse(" \t\n", spaces::parse), Ok(" \t\n".into()));
/// assert_eq!(parse("", spaces::parse), Ok("".into()));
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<String, Error> {
    let start = scanner.position();
    skip(scanner);
    Ok(scanner.since(start).to_string())
}

pub(crate) fn skip(scanner: &mut Scanner) {
    while let Some(' ' | '\t' | '\n' | '\r') = scanner.peek() {
        scanner.advance();
    }
}
