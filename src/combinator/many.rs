use crate::{Parser, Scanner};

/// Zero or more repetitions
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
            let branch = scanner.branch();

            let Ok(value) = item(scanner) else {
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
