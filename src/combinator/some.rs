use crate::{Parser, Scanner, combinator::many};

/// One or more repetitions
///
/// ```rust
/// use kuku::*;
///
/// let digits = some(one_of::parse("0123456789"));
///
/// assert_eq!(parse("123", &digits), Ok(vec!['1', '2', '3']));
/// assert!(parse("", &digits).is_err());
/// ```
pub fn some<T>(item: impl Parser<T>) -> impl Parser<Vec<T>> {
    move |scanner: &mut Scanner| {
        let first = item(scanner)?;

        let mut values = vec![first];
        values.extend(many(&item)(scanner)?);

        Ok(values)
    }
}
