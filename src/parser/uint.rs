use crate::Scanner;

/// Unsigned integer
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("42", uint::parse), Ok(42));
/// assert!(parse("-42", uint::parse).is_err());
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<usize, String> {
    let branch = scanner.branch();
    let mut sum: usize = 0;
    let mut length = 0;

    while let Some(digit) = scanner.peek().and_then(|char| char.to_digit(10)) {
        scanner.advance();
        length += 1;

        let Some(next) = sum
            .checked_mul(10)
            .and_then(|sum| sum.checked_add(digit as usize))
        else {
            scanner.restore(branch);
            return Err("unsigned integer is too large".into());
        };

        sum = next;
    }

    if length == 0 {
        return Err(match scanner.peek() {
            Some(char) => format!("expected unsigned integer, got '{char}'"),
            None => "expected unsigned integer, got end of input".into(),
        });
    }

    Ok(sum)
}
