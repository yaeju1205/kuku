use crate::{Parser, Scanner};

/// Source text that target consumed, in place of its value
///
/// ```rust
/// use kuku::*;
///
/// let name = recognize(some(satisfy::parse("letter", |char| char.is_alphabetic())));
///
/// assert_eq!(parse("nyang", name), Ok("nyang".into()));
/// ```
pub fn recognize<T>(target: impl Parser<T>) -> impl Parser<String> {
    move |scanner: &mut Scanner| {
        let start = scanner.position();
        target(scanner)?;
        Ok(scanner.since(start).to_string())
    }
}
