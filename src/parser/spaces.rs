use crate::Scanner;

/// Zero or more whitespace characters
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse(" \t\n", spaces::parse), Ok(" \t\n".into()));
/// assert_eq!(parse("", spaces::parse), Ok("".into()));
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<String, String> {
    let rest = scanner.rest();
    let mut length = 0;

    while let Some(char @ (' ' | '\t' | '\n' | '\r')) = scanner.peek() {
        scanner.advance();
        length += char.len_utf8();
    }

    Ok(rest[..length].to_string())
}
