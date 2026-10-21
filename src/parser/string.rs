use crate::{Parser, Scanner};

/// Exact string
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("hello", string::parse("hello")), Ok("hello"));
/// assert!(parse("help", string::parse("hello")).is_err());
/// ```
pub fn parse(expected: &str) -> impl Parser<&str> {
    move |scanner: &mut Scanner| {
        let rest = scanner.rest();

        if let Some(stripped) = rest.strip_prefix(expected) {
            let length = rest.len() - stripped.len();
            while scanner.rest().len() > rest.len() - length {
                scanner.advance();
            }

            return Ok(expected);
        }

        let mut got = String::new();
        for (char, expected_char) in rest.chars().zip(expected.chars()) {
            got.push(char);
            if char != expected_char {
                return Err(format!("expected '{expected}', got '{got}'"));
            }
        }

        Err(format!("expected '{expected}', got end of input"))
    }
}
