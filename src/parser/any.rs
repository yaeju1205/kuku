use crate::Scanner;

/// Any single character
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("a", any::parse), Ok('a'));
/// assert!(parse("", any::parse).is_err());
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<char, String> {
    if let Some(char) = scanner.advance() {
        Ok(char)
    } else {
        Err("expected any character, got end of input".into())
    }
}
