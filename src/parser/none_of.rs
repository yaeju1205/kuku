use crate::{Parser, Scanner, parser::satisfy};

/// Single character not in the given characters
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("x", none_of::parse("abc")), Ok('x'));
/// assert!(parse("a", none_of::parse("abc")).is_err());
/// ```
pub fn parse(chars: &str) -> impl Parser<char> {
    move |scanner: &mut Scanner| {
        satisfy::next_if(
            scanner,
            |char| !chars.contains(char),
            || format!("none of '{chars}'"),
        )
    }
}
