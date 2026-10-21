use std::cell::OnceCell;

use crate::{Parser, Scanner};

/// Build the parser on first use, for recursive grammars
///
/// ```rust
/// use kuku::*;
///
/// fn nested(scanner: &mut Scanner) -> Result<usize, Error> {
///     choice((
///         map(between(string::parse("("), lazy(|| nested), string::parse(")")), |depth| depth + 1),
///         map(string::parse(""), |_| 0),
///     ))(scanner)
/// }
///
/// assert_eq!(parse("((()))", nested), Ok(3));
/// ```
pub fn lazy<T, P: Parser<T>>(make: impl Fn() -> P) -> impl Parser<T> {
    let made = OnceCell::new();

    move |scanner: &mut Scanner| made.get_or_init(&make)(scanner)
}
