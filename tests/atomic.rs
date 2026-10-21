use kuku::*;

#[test]
fn string_matches_literally() {
    assert_eq!(parse("let", string::parse("let")), Ok("let"));
    assert_eq!(parse_partial("가나다", string::parse("가나")), Ok("가나"));
}

#[test]
fn string_reports_mismatch() {
    let abc = string::parse("abc");

    assert_eq!(parse("xyz", &abc), Err("expected 'abc', got 'x'".into()));
    assert_eq!(parse("abd", &abc), Err("expected 'abc', got 'abd'".into()));
    assert_eq!(
        parse("ab", &abc),
        Err("expected 'abc', got end of input".into())
    );
}

#[test]
fn string_consumes_nothing_on_failure() {
    let mut scanner = Scanner::new("가x");

    assert!(string::parse("가나")(&mut scanner).is_err());
    assert_eq!(scanner.offset, 0);
    assert_eq!(scanner.rest(), "가x");
}

#[test]
fn string_tracks_lines() {
    let mut scanner = Scanner::new("a\nb\nc");

    assert!(string::parse("a\nb\n")(&mut scanner).is_ok());
    assert_eq!(scanner.line, 3);
    assert_eq!(scanner.offset, 4);
}

#[test]
fn satisfy_uses_predicate() {
    let upper = satisfy::parse("uppercase letter", |char| char.is_uppercase());

    assert_eq!(parse("A", &upper), Ok('A'));
    assert_eq!(
        parse("a", &upper),
        Err("expected uppercase letter, got 'a'".into())
    );
}

#[test]
fn any_consumes_one_char() {
    assert_eq!(parse("a", any::parse), Ok('a'));
    assert!(parse("", any::parse).is_err());
}

#[test]
fn one_of_and_none_of() {
    let digit = one_of::parse("0123456789");
    assert_eq!(parse("7", &digit), Ok('7'));
    assert!(parse("x", &digit).is_err());

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
