use crate::{Scanner, parser::uint};

/// Signed integer with an optional '+' or '-' sign
///
/// ```rust
/// use kuku::*;
///
/// assert_eq!(parse("-42", int::parse), Ok(-42));
/// assert_eq!(parse("+42", int::parse), Ok(42));
/// ```
pub fn parse(scanner: &mut Scanner) -> Result<isize, String> {
    let branch = scanner.branch();

    let negative = match scanner.peek() {
        Some('-') => {
            scanner.advance();
            true
        }
        Some('+') => {
            scanner.advance();
            false
        }
        _ => false,
    };

    let digits = match uint::parse(scanner) {
        Ok(digits) => digits,
        Err(error) => {
            scanner.restore(branch);
            return Err(error);
        }
    };

    let value = if negative {
        0isize.checked_sub_unsigned(digits)
    } else {
        isize::try_from(digits).ok()
    };

    value.ok_or_else(|| {
        scanner.restore(branch);
        "integer is out of range".into()
    })
}
