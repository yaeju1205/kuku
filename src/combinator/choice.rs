use crate::{Parser, Scanner};

/// Tuple of parsers for choice
pub trait Choice<T> {
    fn choose(&self, scanner: &mut Scanner) -> Result<T, String>;
}

macro_rules! impl_choice {
    ($($parser:ident $index:tt),+) => {
        impl<T, $($parser: Parser<T>),+> Choice<T> for ($($parser,)+) {
            fn choose(&self, scanner: &mut Scanner) -> Result<T, String> {
                let branch = scanner.branch();
                let mut errors = Vec::new();

                $(
                    match (self.$index)(scanner) {
                        Ok(value) => return Ok(value),
                        Err(error) => {
                            errors.push(error);
                            scanner.restore(branch.clone());
                        }
                    }
                )+

                Err(errors.join(" or "))
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
/// ```rust
/// use kuku::*;
///
/// let boolean = choice((string::parse("true"), string::parse("false")));
///
/// assert_eq!(parse("false", &boolean), Ok("false"));
/// assert!(parse("maybe", &boolean).is_err());
/// ```
pub fn choice<T>(parsers: impl Choice<T>) -> impl Parser<T> {
    move |scanner: &mut Scanner| parsers.choose(scanner)
}
