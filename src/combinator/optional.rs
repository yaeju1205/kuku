use crate::{Parser, Scanner};

/// Some on success, None on failure
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
        let branch = scanner.branch();

        match item(scanner) {
            Ok(value) => Ok(Some(value)),
            Err(_) => {
                scanner.restore(branch);
                Ok(None)
            }
        }
    }
}
