use crate::{Error, Scanner};

/// Any single character
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("a", any::parse), Ok('a'));
/// assert!(parse("", any::parse).is_err());
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<char, Error> {
    scanner
        .advance()
        .ok_or_else(|| scanner.expected("any character"))
}
