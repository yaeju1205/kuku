use kuku::*;

#[test]
fn choice_tries_in_order() {
    let boolean = choice((string::parse("true"), string::parse("false")));

    assert_eq!(parse("false", &boolean), Ok("false"));
    assert!(parse("maybe", &boolean).is_err());
}

#[test]
fn choice_shares_prefix() {
    let keyword = choice((string::parse("format"), string::parse("for")));

    assert_eq!(parse("for", &keyword), Ok("for"));
}

#[test]
fn sequence_runs_every_parser() {
    let assignment = sequence((string::parse("a"), string::parse("="), uint::parse));

    assert_eq!(parse("a=1", &assignment), Ok(()));

    let mut scanner = Scanner::new("a=x");
    assert!(assignment(&mut scanner).is_err());
    assert_eq!(scanner.offset, 0);
}

#[test]
fn many_collects_zero_or_more() {
    let digits = many(one_of::parse("0123456789"));

    assert_eq!(parse("123", &digits).map(|v| v.len()), Ok(3));
    assert_eq!(parse("", &digits).map(|v| v.len()), Ok(0));
    assert!(parse_partial("", many(eoi::parse)).is_ok());
}

#[test]
fn some_demands_one() {
    let digits = some(one_of::parse("0123456789"));

    assert_eq!(parse("42", &digits), Ok(vec!['4', '2']));
    assert!(parse("", &digits).is_err());
}

#[test]
fn optional_turns_failure_into_none() {
    let sign = optional(string::parse("-"));

    assert_eq!(parse_partial("-1", &sign), Ok(Some("-")));
    assert_eq!(parse_partial("1", &sign), Ok(None));
}

#[test]
fn map_transforms_value() {
    let word = map(some(one_of::parse("abc")), |chars| {
        chars.into_iter().collect::<String>()
    });

    assert_eq!(parse("cab", word), Ok("cab".into()));
}

#[test]
fn between_keeps_inner() {
    let parens = between(string::parse("("), int::parse, string::parse(")"));

    assert_eq!(parse("(-3)", &parens), Ok(-3));

    let mut scanner = Scanner::new("(1");
    assert!(parens(&mut scanner).is_err());
    assert_eq!(scanner.offset, 0);
}

#[test]
fn separated_collects_items() {
    let list = separated(int::parse, string::parse(","));

    assert_eq!(parse("1,2,3", &list), Ok(vec![1, 2, 3]));
    assert_eq!(parse("", &list), Ok(vec![]));

    let mut scanner = Scanner::new("1,2,");
    assert_eq!(list(&mut scanner), Ok(vec![1, 2]));
    assert_eq!(scanner.rest(), ",");
}

#[test]
fn separated_stops_without_progress() {
    assert!(parse_partial("abc", separated(spaces::parse, spaces::parse)).is_ok());
    assert!(parse_partial("", separated(eoi::parse, spaces::parse)).is_ok());
}

#[test]
fn token_eats_trailing_spaces() {
    let list = separated(token(int::parse), token(string::parse(",")));

    assert_eq!(parse("1, 2, 3", list), Ok(vec![1, 2, 3]));
}

fn expression(scanner: &mut Scanner) -> Result<isize, String> {
    let atom = choice((
        int::parse,
        between(string::parse("("), lazy(|| expression), string::parse(")")),
    ));

    map(separated(atom, string::parse("+")), |terms| {
        terms.iter().sum()
    })(scanner)
}

#[test]
fn lazy_ties_recursion() {
    assert_eq!(parse("1+(2+3)", expression), Ok(6));
}

#[test]
fn label_replaces_errors() {
    let identifier = label(
        "identifier",
        some(one_of::parse("abcdefghijklmnopqrstuvwxyz")),
    );

    assert_eq!(parse("1", identifier), Err("expected identifier".into()));
}

#[test]
fn readme_point() {
    let point = between(
        token(string::parse("(")),
        map(
            separated(token(int::parse), token(string::parse(","))),
            |values| (values[0], values[1]),
        ),
        token(string::parse(")")),
    );

    assert_eq!(parse("(1, 2)", point), Ok((1, 2)));
}
