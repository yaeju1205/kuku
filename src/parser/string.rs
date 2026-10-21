use crate::{Error, Parser, Scanner};

/// Exact string
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("hello", string::parse("hello")), Ok("hello"));
/// assert_eq!(
///     parse("help", string::parse("hello")).unwrap_err().to_string(),
///     "1:1: expected 'hello', got 'help'",
/// );
/// ```
pub fn parse(expected: &str) -> impl Parser<&str> {
    move |scanner: &mut Scanner| {
        let rest = scanner.rest();

        if rest.starts_with(expected) {
            for _ in expected.chars() {
                scanner.advance();
            }

            return Ok(expected);
        }

        let mut got = String::new();
        for (char, expected_char) in rest.chars().zip(expected.chars()) {
            got.push(char);
            if char != expected_char {
                return Err(Error::expected(
                    scanner.position(),
                    format!("'{expected}'"),
                    format!("'{got}'"),
                ));
            }
        }

        Err(Error::expected(
            scanner.position(),
            format!("'{expected}'"),
            "end of input",
        ))
    }
}
