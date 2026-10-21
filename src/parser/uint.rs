use crate::{Error, Scanner};

/// Unsigned integer
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("42", uint::parse), Ok(42));
/// assert!(parse("-42", uint::parse).is_err());
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<usize, Error> {
    let start = scanner.position();
    let mut sum: usize = 0;

    while let Some(digit) = scanner.peek().and_then(|char| char.to_digit(10)) {
        scanner.advance();

        let Some(next) = sum
            .checked_mul(10)
            .and_then(|sum| sum.checked_add(digit as usize))
        else {
            scanner.restore(start);
            return Err(scanner.error("unsigned integer is too large"));
        };

        sum = next;
    }

    if scanner.offset() == start.offset {
        return Err(scanner.expected("unsigned integer"));
    }

    Ok(sum)
}
