use crate::{Parser, Scanner};

/// Transform the value
///
/// ```rust
/// use kuku::*;
///
/// let double = map(uint::parse, |value| value * 2);
///
/// assert_eq!(parse("21", double), Ok(42));
/// ```
pub fn map<T, U>(target: impl Parser<T>, transform: impl Fn(T) -> U) -> impl Parser<U> {
    move |scanner: &mut Scanner| target(scanner).map(&transform)
}
