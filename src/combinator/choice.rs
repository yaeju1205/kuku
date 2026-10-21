use crate::{Error, Parser, Scanner};

/// Tuple of parsers for choice
pub trait Choice<T> {
    fn choose(&self, scanner: &mut Scanner) -> Result<T, Error>;
}

macro_rules! impl_choice {
    ($($parser:ident $index:tt),+) => {
        impl<T, $($parser: Parser<T>),+> Choice<T> for ($($parser,)+) {
            fn choose(&self, scanner: &mut Scanner) -> Result<T, Error> {
                let start = scanner.position();
                let mut failure: Option<Error> = None;

                $(
                    match (self.$index)(scanner) {
                        Ok(value) => return Ok(value),
                        Err(error) => {
                            scanner.restore(start);
                            if error.fatal {
                                return Err(error);
                            }
                            failure = Some(match failure {
                                Some(failure) => failure.merge(error),
                                None => error,
                            });
                        }
                    }
                )+

                Err(failure.expect("a choice holds a parser"))
            }
        }
    };
}

impl_choice!(A 0);
impl_choice!(A 0, B 1);
impl_choice!(A 0, B 1, C 2);
impl_choice!(A 0, B 1, C 2, D 3);
impl_choice!(A 0, B 1, C 2, D 3, E 4);
impl_choice!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_choice!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_choice!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
impl_choice!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
impl_choice!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
impl_choice!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
impl_choice!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);

/// Result of the first succeeding parser, tried in order
///
/// When all fail, the error is the one that got furthest. Errors that stopped
/// at the same place are joined. A fatal error stops the choice at once
///
/// ```rust
/// use kuku::*;
///
/// let boolean = choice((string::parse("true"), string::parse("false")));
///
/// assert_eq!(parse("false", &boolean), Ok("false"));
/// assert_eq!(
///     parse("maybe", &boolean).unwrap_err().to_string(),
///     "1:1: expected 'true' or 'false', got 'm'",
/// );
/// ```
pub fn choice<T>(parsers: impl Choice<T>) -> impl Parser<T> {
    move |scanner: &mut Scanner| parsers.choose(scanner)
}
