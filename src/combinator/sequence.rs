use crate::{Error, Parser, Scanner};

/// Tuple of parsers for sequence
pub trait Sequence<T> {
    fn run(&self, scanner: &mut Scanner) -> Result<T, Error>;
}

macro_rules! impl_sequence {
    ($($parser:ident $value:ident $index:tt),+) => {
        impl<$($value, $parser: Parser<$value>),+> Sequence<($($value,)+)> for ($($parser,)+) {
            fn run(&self, scanner: &mut Scanner) -> Result<($($value,)+), Error> {
                Ok(($((self.$index)(scanner)?,)+))
            }
        }
    };
}

impl_sequence!(A TA 0);
impl_sequence!(A TA 0, B TB 1);
impl_sequence!(A TA 0, B TB 1, C TC 2);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4, F TF 5);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4, F TF 5, G TG 6);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4, F TF 5, G TG 6, H TH 7);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4, F TF 5, G TG 6, H TH 7, I TI 8);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4, F TF 5, G TG 6, H TH 7, I TI 8, J TJ 9);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4, F TF 5, G TG 6, H TH 7, I TI 8, J TJ 9, K TK 10);
impl_sequence!(A TA 0, B TB 1, C TC 2, D TD 3, E TE 4, F TF 5, G TG 6, H TH 7, I TI 8, J TJ 9, K TK 10, L TL 11);

/// Tuple of every parser's value, run in order
///
/// ```rust
/// use kuku::*;
///
/// let assignment = sequence((string::parse("a"), string::parse("="), uint::parse));
///
/// assert_eq!(parse("a=1", &assignment), Ok(("a", "=", 1)));
/// assert!(parse("a=x", &assignment).is_err());
/// ```
pub fn sequence<T>(parsers: impl Sequence<T>) -> impl Parser<T> {
    move |scanner: &mut Scanner| {
        let start = scanner.position();

        let result = parsers.run(scanner);
        if result.is_err() {
            scanner.restore(start);
        }

        result
    }
}
