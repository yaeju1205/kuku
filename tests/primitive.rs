use kuku::*;

#[test]
fn uint_reads_digits() {
    assert_eq!(parse("0042", uint::parse), Ok(42));
    assert_eq!(parse("1234", uint::parse), Ok(1234));
    assert!(parse("-1", uint::parse).is_err());
    assert!(parse("99999999999999999999999", uint::parse).is_err());
}

#[test]
fn int_reads_sign() {
    assert_eq!(parse("-42", int::parse), Ok(-42));
    assert_eq!(parse("+7", int::parse), Ok(7));
    assert_eq!(parse("10", int::parse), Ok(10));
}

#[test]
fn int_lone_sign_consumes_nothing() {
    let mut scanner = Scanner::new("-");

    assert!(int::parse(&mut scanner).is_err());
    assert_eq!(scanner.rest(), "-");
}
