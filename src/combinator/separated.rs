use crate::{Parser, Scanner};

/// Zero or more items separated by separator
///
/// Stops before a separator that is not followed by an item, unless an error is fatal
///
/// ```rust
/// use kuku::*;
///
/// let list = separated(uint::parse, string::parse(","));
///
/// assert_eq!(parse("1,2,3", &list), Ok(vec![1, 2, 3]));
/// assert_eq!(parse("", &list), Ok(vec![]));
/// ```
pub fn separated<T, S>(item: impl Parser<T>, separator: impl Parser<S>) -> impl Parser<Vec<T>> {
    move |scanner: &mut Scanner| {
        let mut values = Vec::new();

        let start = scanner.position();
        match item(scanner) {
            Ok(value) => values.push(value),
            Err(error) if error.fatal => return Err(error),
            Err(_) => {
                scanner.restore(start);
                return Ok(values);
            }
        }

        loop {
            let start = scanner.position();

            let value = match separator(scanner).and_then(|_| item(scanner)) {
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
