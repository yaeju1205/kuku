use crate::{Parser, Scanner};

/// Value of target, without consuming anything
///
/// ```rust
/// use kuku::*;
///
/// let mut scanner = Scanner::new("let");
///
/// assert_eq!(look_ahead(string::parse("let"))(&mut scanner), Ok("let"));
/// assert_eq!(scanner.rest(), "let");
/// ```
pub fn look_ahead<T>(target: impl Parser<T>) -> impl Parser<T> {
    move |scanner: &mut Scanner| {
        let start = scanner.position();
        let result = target(scanner);
        scanner.restore(start);
        result
    }
}
