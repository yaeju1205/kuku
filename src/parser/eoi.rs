use crate::Scanner;

/// End of input
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse_partial("", eoi::parse), Ok(()));
/// assert!(parse_partial("a", eoi::parse).is_err());
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<(), String> {
    if let Some(char) = scanner.peek() {
        Err(format!("expected end of input, got '{char}'"))
    } else {
        Ok(())
    }
}
