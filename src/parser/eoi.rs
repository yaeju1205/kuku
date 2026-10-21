use crate::{Error, Scanner};

/// End of input
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse_partial("", eoi::parse), Ok(()));
/// assert!(parse_partial("a", eoi::parse).is_err());
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<(), Error> {
    match scanner.peek() {
        Some(_) => Err(scanner.expected("end of input")),
        None => Ok(()),
    }
}
