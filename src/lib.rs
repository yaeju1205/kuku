mod combinator;
mod error;
mod parser;
mod scanner;

pub use combinator::*;
pub use error::*;
pub use parser::*;
pub use scanner::*;

pub fn parse<T>(source: &str, target: impl Parser<T>) -> Result<T, Error> {
    let mut scanner = Scanner::new(source);

    let value = target(&mut scanner)?;
    eoi::parse(&mut scanner)?;

    Ok(value)
}

pub fn parse_partial<T>(source: &str, target: impl Parser<T>) -> Result<T, Error> {
    target(&mut Scanner::new(source))
}
