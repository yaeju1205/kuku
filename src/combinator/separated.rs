use crate::{Parser, Scanner};

/// Zero or more items separated by separator
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

        let branch = scanner.branch();
        match item(scanner) {
            Ok(value) => values.push(value),
            Err(_) => {
                scanner.restore(branch);
                return Ok(values);
            }
        }

        loop {
            let branch = scanner.branch();

            let Ok(value) = separator(scanner).and_then(|_| item(scanner)) else {
                scanner.restore(branch);
                break;
            };

            values.push(value);

            if scanner.offset == branch.offset {
                break;
            }
        }

        Ok(values)
    }
}
