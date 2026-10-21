use crate::{Parser, Scanner};

/// Replace the error with 'expected description'
///
/// ```rust
/// use kuku::*;
///
/// let digit = label("digit", one_of::parse("0123456789"));
///
/// assert_eq!(parse("x", digit), Err("expected digit".into()));
/// ```
pub fn label<T>(description: impl Into<String>, target: impl Parser<T>) -> impl Parser<T> {
    let description = description.into();

    move |scanner: &mut Scanner| {
        let branch = scanner.branch();

        target(scanner).map_err(|_| {
            scanner.restore(branch);
            format!("expected {description}")
        })
    }
}
