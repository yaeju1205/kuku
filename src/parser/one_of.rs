use crate::{Parser, parser::satisfy};

/// Single character in the given characters
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("b", one_of::parse("abc")), Ok('b'));
/// assert!(parse("x", one_of::parse("abc")).is_err());
/// ```
pub fn parse(chars: &str) -> impl Parser<char> {
    satisfy::parse(format!("one of '{chars}'"), move |char| {
        chars.contains(char)
    })
}
