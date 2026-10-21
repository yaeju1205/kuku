pub mod any;
pub mod eoi;
pub mod int;
pub mod none_of;
pub mod one_of;
pub mod satisfy;
pub mod spaces;
pub mod string;
pub mod uint;

use crate::{Error, Scanner};

/// A function that consumes input from a scanner and returns a value
///
/// ```rust
/// use kuku::*;
///
/// fn digit(scanner: &mut Scanner) -> Result<char, Error> {
///     one_of::parse("0123456789")(scanner)
/// }
///
/// assert_eq!(parse("7", digit), Ok('7'));
/// ```
pub trait Parser<T>: Fn(&mut Scanner) -> Result<T, Error> {}

impl<T, F> Parser<T> for F where F: Fn(&mut Scanner) -> Result<T, Error> {}
