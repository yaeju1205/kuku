use kuku::*;

fn failure<T: std::fmt::Debug>(result: Result<T, Error>) -> String {
    result.unwrap_err().to_string()
}

#[test]
fn choice_tries_in_order() {
    let boolean = choice((string::parse("true"), string::parse("false")));

    assert_eq!(parse("false", &boolean), Ok("false"));
    assert_eq!(
        failure(parse("maybe", &boolean)),
        "1:1: expected 'true' or 'false', got 'm'"
    );
}

#[test]
fn choice_shares_prefix() {
    let keyword = choice((string::parse("format"), string::parse("for")));

    assert_eq!(parse("for", &keyword), Ok("for"));
}

#[test]
fn choice_reports_the_furthest_error() {
    let pair = choice((
        map(
            sequence((string::parse("("), uint::parse, string::parse(")"))),
            |(_, value, _)| value,
        ),
        uint::parse,
    ));

    assert_eq!(failure(parse("(1]", &pair)), "1:3: expected ')', got ']'");
}

#[test]
fn sequence_gives_every_value() {
    let assignment = sequence((string::parse("a"), string::parse("="), uint::parse));

    assert_eq!(parse("a=1", &assignment), Ok(("a", "=", 1)));

    let mut scanner = Scanner::new("a=x");
    assert!(assignment(&mut scanner).is_err());
    assert_eq!(scanner.offset(), 0);
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
    assert_eq!(scanner.offset(), 0);
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

#[test]
fn token_with_eats_what_skip_takes() {
    let comment = sequence((string::parse("#"), many(none_of::parse("\n"))));
    let blank = many(choice((
        map(one_of::parse(" \n"), |_| ()),
        map(comment, |_| ()),
    )));
    let number = token_with(uint::parse, &blank);

    assert_eq!(parse("1 # one\n2 # two", many(&number)), Ok(vec![1, 2]));
}

fn expression(scanner: &mut Scanner) -> Result<isize, Error> {
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
fn label_replaces_errors_at_the_start() {
    let identifier = label(
        "identifier",
        some(one_of::parse("abcdefghijklmnopqrstuvwxyz")),
    );

    assert_eq!(
        failure(parse("1", &identifier)),
        "1:1: expected identifier, got '1'"
    );

    let call = label("call", sequence((&identifier, string::parse("()"))));
    assert_eq!(failure(parse("f(]", call)), "1:2: expected '()', got '(]'");
}

fn statement(scanner: &mut Scanner) -> Result<usize, Error> {
    let assignment = map(
        sequence((token(string::parse("let")), cut(token(uint::parse)))),
        |(_, value)| value,
    );

    choice((assignment, token(uint::parse)))(scanner)
}

#[test]
fn cut_keeps_the_error_where_it_happened() {
    assert_eq!(parse("let 1 2 let 3", many(statement)), Ok(vec![1, 2, 3]));
    assert_eq!(
        failure(parse("let 1 let x", many(statement))),
        "1:11: expected unsigned integer, got 'x'",
    );
    assert_eq!(
        failure(parse("let x", optional(statement))),
        "1:5: expected unsigned integer, got 'x'",
    );
    assert_eq!(
        failure(parse("1,let x", separated(statement, string::parse(",")))),
        "1:7: expected unsigned integer, got 'x'",
    );
}

#[test]
fn look_ahead_consumes_nothing() {
    let mut scanner = Scanner::new("abc");

    assert_eq!(look_ahead(string::parse("ab"))(&mut scanner), Ok("ab"));
    assert_eq!(scanner.offset(), 0);
    assert!(look_ahead(string::parse("x"))(&mut scanner).is_err());
    assert_eq!(scanner.offset(), 0);
}

#[test]
fn not_succeeds_where_target_fails() {
    let letter = satisfy::parse("letter", |char| char.is_alphabetic());
    let keyword = sequence((string::parse("for"), not(&letter)));

    assert!(parse_partial("for x", &keyword).is_ok());
    assert_eq!(
        failure(parse_partial("format", &keyword)),
        "1:4: unexpected 'm'"
    );
}

#[test]
fn recognize_gives_source() {
    let word = recognize(sequence((
        satisfy::parse("letter", |char| char.is_alphabetic()),
        many(satisfy::parse("letter or digit", |char| {
            char.is_alphanumeric()
        })),
    )));

    assert_eq!(parse("nyang2", word), Ok("nyang2".into()));
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
