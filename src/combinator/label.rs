use crate::{Error, Parser, Scanner};

/// Replace the error with 'expected description' when target failed where it started
///
/// An error from further in, or a fatal one, is kept, since it says more
///
/// ```rust
/// use kuku::*;
///
/// let digit = label("digit", one_of::parse("0123456789"));
///
/// assert_eq!(parse("x", digit).unwrap_err().to_string(), "1:1: expected digit, got 'x'");
/// ```
pub fn label<T>(description: impl Into<String>, target: impl Parser<T>) -> impl Parser<T> {
    let description = description.into();

    move |scanner: &mut Scanner| {
        let start = scanner.position();

        target(scanner).map_err(|error| {
            scanner.restore(start);
            if error.fatal || error.position.offset > start.offset {
                error
            } else {
                Error::expected(start, description.clone(), scanner.found())
            }
        })
    }
}
