use crate::{Parser, Scanner, parser::satisfy};

/// Single character in the given characters
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("b", one_of::parse("abc")), Ok('b'));
/// assert!(parse("x", one_of::parse("abc")).is_err());
/// ```
pub fn parse(chars: &str) -> impl Parser<char> {
    move |scanner: &mut Scanner| {
        satisfy::next_if(
            scanner,
            |char| chars.contains(char),
            || format!("one of '{chars}'"),
        )
    }
}
