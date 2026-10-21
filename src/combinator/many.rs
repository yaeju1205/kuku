use crate::{Parser, Scanner};

/// Zero or more repetitions
///
/// Stops at the first failing item, unless its error is fatal
///
/// ```rust
/// use kuku::*;
///
/// let digits = many(one_of::parse("0123456789"));
///
/// assert_eq!(parse("123", &digits), Ok(vec!['1', '2', '3']));
/// assert_eq!(parse("", &digits), Ok(vec![]));
/// ```
pub fn many<T>(item: impl Parser<T>) -> impl Parser<Vec<T>> {
    move |scanner: &mut Scanner| {
        let mut values = Vec::new();

        loop {
            let start = scanner.position();

            let value = match item(scanner) {
                Ok(value) => value,
                Err(error) if error.fatal => return Err(error),
                Err(_) => {
                    scanner.restore(start);
                    break;
                }
            };

            values.push(value);

            if scanner.offset() == start.offset {
                break;
            }
        }

        Ok(values)
    }
}
