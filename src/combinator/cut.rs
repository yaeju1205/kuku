use crate::{Error, Parser, Scanner};

/// Make every error of target fatal, so choice, many, optional and separated
/// pass it on instead of trying something else
///
/// Put it after the part that decides which way to go, so a mistake inside
/// is reported where it is, not as a failed alternative
///
/// ```rust
/// use kuku::*;
///
/// let item = choice((
///     map(sequence((string::parse("+"), cut(uint::parse))), |(_, value)| value),
///     uint::parse,
/// ));
/// let items = many(token(item));
///
/// assert_eq!(parse("1 +2 3", &items), Ok(vec![1, 2, 3]));
/// assert_eq!(
///     parse("1 +x", &items).unwrap_err().to_string(),
///     "1:4: expected unsigned integer, got 'x'",
/// );
/// ```
pub fn cut<T>(target: impl Parser<T>) -> impl Parser<T> {
    move |scanner: &mut Scanner| target(scanner).map_err(Error::fatal)
}
