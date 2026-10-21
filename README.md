# kuku

A small parser combinator library for Rust, inspired by mrow, a parser combinator library for [Luau](https://luau.org).

A parser is just a function of the form `Fn(&mut Scanner) -> Result<T, String>`, so plain functions and closures work as parsers.

## Example

```rust
use kuku::*;

let point = between(
    token(string::parse("(")),
    map(
        separated(token(int::parse), token(string::parse(","))),
        |values| (values[0], values[1]),
    ),
    token(string::parse(")")),
);

assert_eq!(parse("(1, 2)", point), Ok((1, 2)));
```
