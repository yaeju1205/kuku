use crate::{Parser, parser::satisfy};

/// Single character not in the given characters
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("x", none_of::parse("abc")), Ok('x'));
/// assert!(parse("a", none_of::parse("abc")).is_err());
/// ```
pub fn parse(chars: &str) -> impl Parser<char> {
    satisfy::parse(format!("none of '{chars}'"), move |char| {
        !chars.contains(char)
    })
}
