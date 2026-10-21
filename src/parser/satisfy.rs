use crate::{Parser, Scanner};

/// Single character matching the predicate
///
/// ```rust
/// use kuku::*;
///
/// let digit = satisfy::parse("digit", |char| char.is_ascii_digit());
///
/// assert_eq!(parse("7", &digit), Ok('7'));
/// assert_eq!(parse("x", &digit), Err("expected digit, got 'x'".into()));
/// ```
pub fn parse(
    description: impl Into<String>,
    predicate: impl Fn(char) -> bool,
) -> impl Parser<char> {
    let description = description.into();

    move |scanner: &mut Scanner| match scanner.peek() {
        Some(char) if predicate(char) => {
            scanner.advance();
            Ok(char)
        }
        Some(char) => Err(format!("expected {description}, got '{char}'")),
        None => Err(format!("expected {description}, got end of input")),
    }
}
