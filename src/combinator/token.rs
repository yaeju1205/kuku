use crate::{Parser, Scanner, parser::spaces};

/// Value of target, consuming trailing whitespace
///
/// ```rust
/// use kuku::*;
///
/// let keyword = token(string::parse("let"));
///
/// assert_eq!(parse("let  ", keyword), Ok("let"));
/// ```
pub fn token<T>(target: impl Parser<T>) -> impl Parser<T> {
    move |scanner: &mut Scanner| {
        let value = target(scanner)?;
        spaces::parse(scanner)?;
        Ok(value)
    }
}
