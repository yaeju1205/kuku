# kuku

A small parser combinator library for Rust, inspired by mrow, a parser combinator library for [Luau](https://luau.org).

A parser is just a function of the form `Fn(&mut Scanner) -> Result<T, Error>`, so plain functions and closures work as parsers.

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

## Errors

An `Error` knows where it happened: its `position` holds the byte offset, line and column.
When every branch of a `choice` fails, the error is the one that got furthest, and errors
that stopped at the same place are joined.

```rust
use kuku::*;

let boolean = choice((string::parse("true"), string::parse("false")));

assert_eq!(
    parse("maybe", boolean).unwrap_err().to_string(),
    "1:1: expected 'true' or 'false', got 'm'",
);
```

`many`, `optional`, `separated` and `choice` try something else when a parser fails. Once the
input has decided which way to go, wrap the rest in `cut`, and its errors are passed on as they
are instead of being taken for a failed alternative.

```rust
use kuku::*;

let statement = choice((
    map(
        sequence((token(string::parse("let")), cut(token(uint::parse)))),
        |(_, value)| value,
    ),
    token(uint::parse),
));

assert_eq!(
    parse("let 1 let x", many(statement)).unwrap_err().to_string(),
    "1:11: expected unsigned integer, got 'x'",
);
```
