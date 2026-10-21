use kuku::*;

fn failure<T: std::fmt::Debug>(result: Result<T, Error>) -> String {
    result.unwrap_err().to_string()
}

#[test]
fn string_matches_literally() {
    assert_eq!(parse("let", string::parse("let")), Ok("let"));
    assert_eq!(parse_partial("가나다", string::parse("가나")), Ok("가나"));
}

#[test]
fn string_reports_mismatch() {
    let abc = string::parse("abc");

    assert_eq!(failure(parse("xyz", &abc)), "1:1: expected 'abc', got 'x'");
    assert_eq!(
        failure(parse("abd", &abc)),
        "1:1: expected 'abc', got 'abd'"
    );
    assert_eq!(
        failure(parse("ab", &abc)),
        "1:1: expected 'abc', got end of input"
    );
}

#[test]
fn string_consumes_nothing_on_failure() {
    let mut scanner = Scanner::new("가x");

    assert!(string::parse("가나")(&mut scanner).is_err());
    assert_eq!(scanner.offset(), 0);
    assert_eq!(scanner.rest(), "가x");
}

#[test]
fn scanner_tracks_lines_and_columns() {
    let mut scanner = Scanner::new("a\nb가c");

    assert!(string::parse("a\nb가")(&mut scanner).is_ok());
    assert_eq!(
        scanner.position(),
        Position {
            offset: 6,
            line: 2,
            column: 3
        }
    );
    assert_eq!(scanner.rest(), "c");
}

#[test]
fn errors_carry_their_position() {
    let error = parse("1\n22x", many(token(uint::parse))).unwrap_err();

    assert_eq!(
        error.position,
        Position {
            offset: 4,
            line: 2,
            column: 3
        }
    );
    assert_eq!(error.to_string(), "2:3: expected end of input, got 'x'");
}

#[test]
fn satisfy_uses_predicate() {
    let upper = satisfy::parse("uppercase letter", |char| char.is_uppercase());

    assert_eq!(parse("A", &upper), Ok('A'));
    assert_eq!(
        failure(parse("a", &upper)),
        "1:1: expected uppercase letter, got 'a'"
    );
}

#[test]
fn any_consumes_one_char() {
    assert_eq!(parse("a", any::parse), Ok('a'));
    assert_eq!(
        failure(parse("", any::parse)),
        "1:1: expected any character, got end of input"
    );
}

#[test]
fn one_of_and_none_of() {
    let digit = one_of::parse("0123456789");
    assert_eq!(parse("7", &digit), Ok('7'));
    assert_eq!(
        failure(parse("x", &digit)),
        "1:1: expected one of '0123456789', got 'x'"
    );

    let text_char = none_of::parse("\"\\");
    assert_eq!(parse("a", &text_char), Ok('a'));
    assert!(parse("\"", &text_char).is_err());
}

#[test]
fn spaces_never_fails() {
    assert_eq!(parse("  \n", spaces::parse), Ok("  \n".into()));
    assert_eq!(parse_partial("a", spaces::parse), Ok("".into()));
}

#[test]
fn eoi_consumes_nothing() {
    let mut scanner = Scanner::new("a");

    assert!(eoi::parse(&mut scanner).is_err());
    assert_eq!(scanner.rest(), "a");
    assert_eq!(parse("", eoi::parse), Ok(()));
}

#[test]
fn since_gives_consumed_source() {
    let mut scanner = Scanner::new("가나다");
    let start = scanner.position();

    scanner.advance();
    scanner.advance();

    assert_eq!(scanner.since(start), "가나");
}
