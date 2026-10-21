use crate::{Parser, Scanner};

/// Succeed, consuming nothing, only where target fails
///
/// ```rust
/// use kuku::*;
///
/// let letter = satisfy::parse("letter", |char| char.is_alphabetic());
/// let keyword = sequence((string::parse("let"), not(&letter)));
///
/// assert!(parse_partial("let x", &keyword).is_ok());
/// assert_eq!(
///     parse_partial("letter", &keyword).unwrap_err().to_string(),
///     "1:4: unexpected 't'",
/// );
/// ```
pub fn not<T>(target: impl Parser<T>) -> impl Parser<()> {
    move |scanner: &mut Scanner| {
        let start = scanner.position();
        match target(scanner) {
            Ok(_) => {
                scanner.restore(start);
                Err(scanner.error(format!("unexpected {}", scanner.found())))
            }
            Err(_) => {
                scanner.restore(start);
                Ok(())
            }
        }
    }
}
