# Changelog

## 0.2.0

### Breaking

- Parsers return `Result<T, Error>` instead of `Result<T, String>`. `Error` holds a `Position`,
  a `Reason` (what was expected and what was found, or a message) and whether it is fatal.
- `Scanner` keeps its source and a `Position`. `offset` is a byte offset, and there is a column.
  The fields are private: use `position()`, `offset()`, `line()`, `column()`.
  `branch()` returns a `Position`, which `restore()` takes back.
- `sequence` gives the tuple of every parser's value instead of `()`.
- `choice` gives the error that got furthest, joining errors at the same place, instead of
  joining every message with "or".
- `label` replaces the error only when the target failed where it started, and adds what was found.

### Added

- `cut`: makes errors fatal, so `choice`, `many`, `optional` and `separated` pass them on.
- `look_ahead`, `not`, `recognize`, `token_with`.
- `Scanner::since`, `Scanner::expected`, `Scanner::error`, `Scanner::found`, `Error::merge`.

### Changed

- `satisfy`, `one_of` and `none_of` describe themselves only when they fail, so building them
  no longer allocates.
- `token` no longer allocates the spaces it skips.
