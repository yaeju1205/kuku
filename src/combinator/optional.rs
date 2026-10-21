use crate::{Parser, Scanner};

/// Some on success, None on failure, unless the error is fatal
///
/// ```rust
/// use kuku::*;
///
/// let sign = optional(string::parse("-"));
///
/// assert_eq!(parse("-", &sign), Ok(Some("-")));
/// assert_eq!(parse("", &sign), Ok(None));
/// ```
pub fn optional<T>(item: impl Parser<T>) -> impl Parser<Option<T>> {
    move |scanner: &mut Scanner| {
        let start = scanner.position();

        match item(scanner) {
            Ok(value) => Ok(Some(value)),
            Err(error) if error.fatal => Err(error),
            Err(_) => {
                scanner.restore(start);
                Ok(None)
            }
        }
    }
}
