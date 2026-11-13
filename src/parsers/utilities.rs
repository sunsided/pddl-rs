//! Utility parsers.

use crate::parsers::{NomError, ParseResult, Span};
use nom::branch::alt;
use nom::bytes::complete::{tag_no_case, take_while};
use nom::character::complete::{char, multispace1};
use nom::combinator::{recognize, value};
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded};
use nom::Parser;

/// Recognizes a semicolon comment up to, but not including, the line ending.
/// Accepts empty comments such as `;` followed immediately by `\n` or `\r\n`.
fn eol_comment<'a>(input: Span<'a>) -> ParseResult<'a, Span<'a>> {
    recognize(pair(
        char(';'),
        take_while(|c: char| c != '\n' && c != '\r'),
    ))
    .parse(input)
}

/// Skips all leading whitespace and semicolon comments.
/// Handles multiple comment lines, including empty ones (`;\n`).
pub(crate) fn skip_whitespace_and_comments<'a>(input: Span<'a>) -> ParseResult<'a, ()> {
    value((), many0(alt((multispace1, eol_comment)))).parse(input)
}

/// Separates two tokens: whitespace and `;` comments, which may be omitted
/// before a `(` (e.g. `:parameters(?x)` or `(and(p)(q))`).
pub(crate) fn sep(input: Span) -> ParseResult<()> {
    let (rest, ()) = skip_whitespace_and_comments(input)?;
    if rest.location_offset() > input.location_offset() || rest.fragment().starts_with('(') {
        Ok((rest, ()))
    } else {
        Err(nom::Err::Error(NomError::new(
            input,
            nom::error::ErrorKind::MultiSpace,
        )))
    }
}

/// Repeatedly parses `section` (skipping whitespace and comments before each)
/// and hands every result to `apply` together with the position it started at.
///
/// Stops at the first input that does not start a new `(` form, e.g. the
/// closing `)` of the enclosing definition. A `(` form that `section` does not
/// match (an unknown or malformed section) is a non-recoverable error at that
/// form, so callers such as `alt` don't fall back to unrelated alternatives.
pub(crate) fn fold_sections<'a, O>(
    mut input: Span<'a>,
    mut section: impl Parser<Span<'a>, Output = O, Error = NomError<'a>>,
    mut apply: impl FnMut(O, Span<'a>) -> Result<(), nom::Err<NomError<'a>>>,
) -> ParseResult<'a, ()> {
    loop {
        let (start, _) = skip_whitespace_and_comments(input)?;
        match section.parse(start) {
            // A section that consumes nothing would make this loop spin forever.
            Ok((rest, _)) if rest.location_offset() == start.location_offset() => {
                return Err(failure_at(start, "section consumed no input"));
            }
            Ok((rest, value)) => {
                apply(value, start)?;
                input = rest;
            }
            Err(nom::Err::Error(_)) if start.fragment().starts_with('(') => {
                return Err(failure_at(start, "unknown section"));
            }
            Err(nom::Err::Error(_)) => return Ok((input, ())),
            Err(e) => return Err(e),
        }
    }
}

/// Stores a section's `value`, failing at `at` if that section was already given.
pub(crate) fn set_once<'a, T>(
    slot: &mut Option<T>,
    value: T,
    at: Span<'a>,
) -> Result<(), nom::Err<NomError<'a>>> {
    if slot.is_some() {
        return Err(failure_at(at, "duplicate section"));
    }
    *slot = Some(value);
    Ok(())
}

/// A non-recoverable error at `at` with the given `reason`, used for
/// structural errors such as duplicate or missing sections.
pub(crate) fn failure_at<'a>(at: Span<'a>, reason: &'a str) -> nom::Err<NomError<'a>> {
    nom::Err::Failure(NomError::with_context(at, reason))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes a leading `(name` and trailing `)`, returning the output of `inner`.
#[allow(clippy::needless_lifetimes)]
pub fn prefix_expr<'a, P, O>(
    name: &'a str,
    inner: P,
) -> impl Parser<Span<'a>, Output = O, Error = NomError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = NomError<'a>>,
{
    // A `:keyword` (e.g. `:action`) can't start anything else, so once it matched,
    // a failure inside the form is reported where it happened instead of
    // backtracking to the form's start. Other keywords such as `at` or `=`
    // must keep backtracking: `(at a b)` is also an ordinary atomic formula.
    let commits = name.starts_with(':');
    let mut open = preceded(ws(tag_no_case("(")), keyword(name));
    let mut body = (ws(inner), ws(tag_no_case(")")));
    move |input: Span<'a>| {
        let (rest, _) = open.parse(input)?;
        match body.parse(rest) {
            Ok((rest, (value, _))) => Ok((rest, value)),
            Err(nom::Err::Error(mut e) | nom::Err::Failure(mut e)) if commits => {
                e.context.get_or_insert(name);
                Err(nom::Err::Failure(e))
            }
            Err(e) => Err(e),
        }
    }
}

/// Parses `<keyword> <inner>` inside a form, e.g. `:precondition (and ...)`.
///
/// Like [`prefix_expr`] for `:keywords`, it commits once the keyword matched,
/// so a mistake in `inner` is reported where it is, tagged with the keyword.
pub(crate) fn keyword_arg<'a, P, O>(
    name: &'a str,
    inner: P,
) -> impl Parser<Span<'a>, Output = O, Error = NomError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = NomError<'a>>,
{
    let mut key = keyword(name);
    let mut body = preceded(sep, inner);
    move |input: Span<'a>| {
        let (rest, _) = key.parse(input)?;
        body.parse(rest).map_err(|e| match e {
            nom::Err::Error(mut e) | nom::Err::Failure(mut e) => {
                e.context.get_or_insert(name);
                nom::Err::Failure(e)
            }
            incomplete => incomplete,
        })
    }
}

/// Matches the keyword `name` (ignoring case) as a whole word: a keyword
/// ending in a letter or digit must not be followed by a name character, so
/// `preference` does not match the start of `preferences`.
pub(crate) fn keyword<'a>(
    name: &'a str,
) -> impl Parser<Span<'a>, Output = Span<'a>, Error = NomError<'a>> {
    let needs_boundary = name.ends_with(|c: char| c.is_ascii_alphanumeric());
    move |input: Span<'a>| {
        let (rest, matched) = tag_no_case(name).parse(input)?;
        // Use the name grammar itself, so the boundary can't drift from what a name may contain.
        let continues_name = super::name::parse_any_char(rest).is_ok();
        if needs_boundary && continues_name {
            return Err(nom::Err::Error(NomError::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        Ok((rest, matched))
    }
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes leading whitespace,
/// returning the output of `inner`.
///
/// This parser also suppresses line comments.
pub fn ws<'a, P, O>(inner: P) -> impl Parser<Span<'a>, Output = O, Error = NomError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = NomError<'a>>,
{
    preceded(skip_whitespace_and_comments, inner)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes leading
/// and trailing whitespace, returning the output of `inner`.
///
/// This parser also suppresses line comments.
pub fn ws2<'a, P, O>(inner: P) -> impl Parser<Span<'a>, Output = O, Error = NomError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = NomError<'a>>,
{
    delimited(
        skip_whitespace_and_comments,
        inner,
        skip_whitespace_and_comments,
    )
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
#[allow(dead_code)]
pub fn space_separated_list0<'a, P, O>(
    inner: P,
) -> impl Parser<Span<'a>, Output = Vec<O>, Error = NomError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = NomError<'a>>,
{
    ws(separated_list0(
        sep,
        preceded(skip_whitespace_and_comments, inner),
    ))
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
pub fn space_separated_list1<'a, P, O>(
    inner: P,
) -> impl Parser<Span<'a>, Output = Vec<O>, Error = NomError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = NomError<'a>>,
{
    ws(separated_list1(
        sep,
        preceded(skip_whitespace_and_comments, inner),
    ))
}

/// A combinator that takes a parser `inner` and produces a parser that consumes
/// surrounding parentheses, returning the outputs of `inner`.
pub fn parens<'a, P, O>(inner: P) -> impl Parser<Span<'a>, Output = O, Error = NomError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = NomError<'a>>,
{
    preceded(
        skip_whitespace_and_comments,
        delimited(char('('), ws(inner), char(')')),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{parse_name, Match};
    use crate::Name;
    use nom::multi::separated_list1;

    #[test]
    fn sep_is_optional_only_before_a_paren() {
        assert_eq!(*sep(Span::new("(p)")).unwrap().0.fragment(), "(p)");
        assert_eq!(*sep(Span::new("  x")).unwrap().0.fragment(), "x");
        assert_eq!(*sep(Span::new("; c\nx")).unwrap().0.fragment(), "x");
        // Two names still need something between them.
        assert!(sep(Span::new("x")).is_err());
        assert!(sep(Span::new("")).is_err());
    }

    #[test]
    fn fold_sections_fails_on_a_section_that_consumes_nothing() {
        let input = Span::new("  (a)");
        let result = fold_sections(input, nom::combinator::success(()), |_, _| Ok(()));
        let Err(nom::Err::Failure(e)) = result else {
            panic!("expected failure, got {result:?}");
        };
        assert_eq!(e.input.location_offset(), 2);
    }

    #[test]
    fn keyword_matches_whole_words_only() {
        let mut and = keyword("and");
        assert!(and.parse(Span::new("and (p)")).is_ok());
        assert!(and.parse(Span::new("AND(p)")).is_ok());
        assert!(and.parse(Span::new("and)")).is_ok());
        assert!(and.parse(Span::new("andy")).is_err());
        assert!(and.parse(Span::new("and-or")).is_err());
        assert!(and.parse(Span::new("and_2")).is_err());
        // Keywords ending in a symbol need no boundary.
        assert!(keyword("=").parse(Span::new("=?x")).is_ok());
    }

    #[test]
    fn parens_works() {
        let input = "(content)";
        let mut parser = parens(parse_name);
        assert!(parser.parse(Span::new(input)).is_exactly("content"));
    }

    #[test]
    fn definition_section_works() {
        let input = "(either x y)";
        let inner_parser = separated_list1(tag_no_case(" "), parse_name);
        let mut parser = prefix_expr("either", inner_parser);
        assert!(parser
            .parse(Span::new(input))
            .is_exactly(vec![Name::from("x"), Name::from("y")]));
    }

    #[test]
    fn space_separated_list0_works() {
        let mut parser = space_separated_list0(parse_name);
        assert!(parser
            .parse(Span::new("x y"))
            .is_exactly(vec![Name::from("x"), Name::from("y")]));
        assert!(parser
            .parse(Span::new("x"))
            .is_exactly(vec![Name::from("x")]));
        assert!(parser.parse(Span::new("")).is_exactly(vec![]));
    }

    #[test]
    fn space_separated_list1_works() {
        let mut parser = space_separated_list1(parse_name);
        assert!(parser
            .parse(Span::new("x y"))
            .is_exactly(vec![Name::from("x"), Name::from("y")]));
        assert!(parser
            .parse(Span::new("x"))
            .is_exactly(vec![Name::from("x")]));
        assert!(parser.parse(Span::new("")).is_err());
    }

    #[test]
    fn leading_empty_comment() {
        let input = ";\ncontent";
        let mut parser = ws(tag_no_case("content"));
        let (remainder, _) = parser.parse(Span::new(input)).unwrap();
        assert!(remainder.fragment().is_empty());
    }

    #[test]
    fn multiple_leading_comments() {
        let input = "; comment 1\n; comment 2\n; comment 3\ncontent";
        let mut parser = ws(tag_no_case("content"));
        let (remainder, _) = parser.parse(Span::new(input)).unwrap();
        assert!(remainder.fragment().is_empty());
    }

    #[test]
    fn mixed_comments_and_whitespace() {
        let input = "\n  ; comment\n\n  ; another\n\t\ncontent";
        let mut parser = ws(tag_no_case("content"));
        let (remainder, _) = parser.parse(Span::new(input)).unwrap();
        assert!(remainder.fragment().is_empty());
    }

    #[test]
    fn domain_with_leading_comments() {
        let input = r#"; This is a comment
; Another comment
(define (domain test)
    (:requirements :strips)
    (:predicates (p))
)
"#;
        let (_, domain) = crate::parsers::parse_domain(Span::new(input)).unwrap();
        assert_eq!(domain.name(), &crate::Name::new("test"));
    }
}
