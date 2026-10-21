use crate::{Parser, Scanner, parser::spaces};

/// Value of target, consuming trailing whitespace
///
/// ```rust
/// use kuku::*;
///
/// let keyword = token(string::parse("let"));
///
/// assert_eq!(parse("let  ", keyword), Ok("let"));
/// ```
pub fn token<T>(target: impl Parser<T>) -> impl Parser<T> {
    move |scanner: &mut Scanner| {
        let value = target(scanner)?;
        spaces::skip(scanner);
        Ok(value)
    }
}

/// Value of target, consuming whatever skip takes after it, such as spaces and comments
///
/// ```rust
/// use kuku::*;
///
/// let comment = sequence((string::parse("//"), many(none_of::parse("\n"))));
/// let blank = many(choice((map(one_of::parse(" \n"), |_| ()), map(comment, |_| ()))));
/// let number = token_with(uint::parse, &blank);
///
/// assert_eq!(parse("1 // one\n2", many(&number)), Ok(vec![1, 2]));
/// ```
pub fn token_with<T, S>(target: impl Parser<T>, skip: impl Parser<S>) -> impl Parser<T> {
    move |scanner: &mut Scanner| {
        let value = target(scanner)?;
        skip(scanner)?;
        Ok(value)
    }
}
