use crate::{Parser, Scanner};

/// Value of inner, surrounded by open and close
///
/// ```rust
/// use kuku::*;
///
/// let parens = between(string::parse("("), uint::parse, string::parse(")"));
///
/// assert_eq!(parse("(1)", parens), Ok(1));
/// ```
pub fn between<O, T, C>(
    open: impl Parser<O>,
    inner: impl Parser<T>,
    close: impl Parser<C>,
) -> impl Parser<T> {
    move |scanner: &mut Scanner| {
        let start = scanner.position();

        let result = open(scanner).and_then(|_| {
            let value = inner(scanner)?;
            close(scanner)?;
            Ok(value)
        });

        if result.is_err() {
            scanner.restore(start);
        }

        result
    }
}
