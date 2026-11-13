use std::fmt;

use nom::error::{ContextError, ErrorKind};

use crate::parsers::Span;

/// Maximum number of characters of the remaining input kept in a [`ParseError`].
const EXCERPT_CHARS: usize = 40;

/// An error returned by [`Parser::from_str`](crate::parsers::Parser::from_str).
///
/// Describes where parsing stopped and which input was found there. The
/// excerpt is bounded, so formatting the error never echoes the whole
/// remaining input.
///
/// ## Example
/// ```
/// # use pddl::{Domain, Parser};
/// let error = Domain::from_str("(define (domain d)\n  (:bogus))").unwrap_err();
/// assert_eq!((error.line(), error.column()), (2, 3));
/// assert_eq!(error.found(), Some("(:bogus))"));
/// assert_eq!(error.reason(), Some("unknown section"));
/// assert_eq!(error.to_string(), "line 2, column 3: unknown section: unexpected `(:bogus))`");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    offset: usize,
    line: usize,
    column: usize,
    found: Option<String>,
    keyword: Option<String>,
    reason: Option<String>,
}

impl ParseError {
    /// The byte offset into the input at which parsing stopped.
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// The 1-based line at which parsing stopped.
    pub const fn line(&self) -> usize {
        self.line
    }

    /// The 1-based column (in characters) at which parsing stopped.
    pub const fn column(&self) -> usize {
        self.column
    }

    /// A short excerpt of the input at which parsing stopped,
    /// or `None` if the input ended early.
    pub fn found(&self) -> Option<&str> {
        self.found.as_deref()
    }

    /// The PDDL keyword of the innermost form in which parsing stopped, if known,
    /// e.g. `:precondition` for a mistake inside an action's precondition.
    ///
    /// Made safe for display like [`found`](Self::found): characters outside
    /// printable ASCII are escaped, and long values are cut and end in `...`.
    pub fn keyword(&self) -> Option<&str> {
        self.keyword.as_deref()
    }

    /// Why parsing stopped, if known, e.g. `duplicate section`, ``missing `:goal` ``
    /// or ``expected `)` `` for a form that isn't closed.
    ///
    /// Made safe for display like [`found`](Self::found): characters outside
    /// printable ASCII are escaped, and long values are cut and end in `...`.
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Converts a nom error for `input` into a [`ParseError`].
    pub(crate) fn from_nom(input: &str, error: nom::Err<NomError<'_>>) -> Self {
        let (mut at, keyword, reason) = match error {
            nom::Err::Error(e) | nom::Err::Failure(e) => (e.input, e.keyword, e.context),
            // The parsers are complete, but report the end of input if one ever is not.
            nom::Err::Incomplete(_) => (
                nom::Input::take_from(&Span::new(input), input.len()),
                None,
                None,
            ),
        };
        // Point at the next token rather than at whitespace or comments before it.
        if let Ok((token, ())) = crate::parsers::utilities::skip_whitespace_and_comments(at) {
            at = token;
        }
        let mut error = Self::at(at);
        error.keyword = keyword.map(sanitize);
        error.reason = reason.map(sanitize);
        error
    }

    fn at(span: Span<'_>) -> Self {
        let rest = span.fragment();
        let found = if rest.is_empty() {
            None
        } else {
            Some(sanitize(rest))
        };
        Self {
            offset: span.location_offset(),
            line: span.location_line() as usize,
            column: span.get_utf8_column(),
            found,
            keyword: None,
            reason: None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}: ", self.line, self.column)?;
        if let Some(keyword) = &self.keyword {
            write!(f, "in `{keyword}`: ")?;
        }
        if let Some(reason) = &self.reason {
            write!(f, "{reason}: ")?;
        }
        match &self.found {
            Some(found) => write!(f, "unexpected `{found}`"),
            None => f.write_str("unexpected end of input"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Makes text that may come from untrusted input safe for an error message.
///
/// Printable ASCII is kept, ASCII whitespace becomes a space, and every other
/// character is shown as an escape such as `\u{202e}`. Nothing can then alter
/// or hide what a terminal or log shows (ANSI escapes, bidi overrides,
/// zero-width or tag characters, ...); PDDL itself is pure ASCII. The result
/// covers at most [`EXCERPT_CHARS`] characters of `text`, followed by `...`
/// if it was cut.
fn sanitize(text: &str) -> String {
    let mut safe = String::new();
    for c in text.chars().take(EXCERPT_CHARS) {
        if c.is_ascii_graphic() || c == ' ' {
            safe.push(c);
        } else if c.is_ascii_whitespace() {
            safe.push(' ');
        } else {
            safe.extend(c.escape_unicode());
        }
    }
    if text.chars().nth(EXCERPT_CHARS).is_some() {
        safe.push_str("...");
    }
    safe
}

/// The error type of the individual `parse_*` combinators.
///
/// [`Parser::from_str`](crate::parsers::Parser::from_str) converts it into a [`ParseError`].
/// Like `nom::error::Error`, it records where (`input`) and in which combinator
/// (`code`) parsing failed. It additionally carries the PDDL `keyword` of the
/// innermost form in which parsing failed (e.g. `:action`) and a `context`
/// describing why (e.g. `duplicate section`), if known.
#[derive(Debug, Clone, PartialEq)]
pub struct NomError<'a> {
    /// The remaining input at the position of the error.
    pub input: Span<'a>,
    /// The nom combinator that failed.
    pub code: ErrorKind,
    /// The PDDL keyword of the innermost form in which parsing failed, if known.
    pub keyword: Option<&'a str>,
    /// Why parsing failed, if known.
    pub context: Option<&'a str>,
}

impl<'a> NomError<'a> {
    /// Creates an error at `input` without a context.
    pub const fn new(input: Span<'a>, code: ErrorKind) -> Self {
        Self {
            input,
            code,
            keyword: None,
            context: None,
        }
    }

    /// Creates an error at `input` with the given reason.
    pub(crate) const fn with_context(input: Span<'a>, context: &'a str) -> Self {
        Self {
            input,
            code: ErrorKind::Verify,
            keyword: None,
            context: Some(context),
        }
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for NomError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        Self::new(input, kind)
    }

    /// Keeps the innermost error, which points closest to the actual mistake.
    fn append(_input: Span<'a>, _kind: ErrorKind, other: Self) -> Self {
        other
    }

    /// When all alternatives failed, keeps the error that got furthest into
    /// the input: that alternative most likely was the one intended. On a tie,
    /// prefers the error that knows more about why it failed.
    fn or(self, other: Self) -> Self {
        let rank = |e: &Self| {
            let described = e.context.is_some() || e.keyword.is_some();
            (e.input.location_offset(), described)
        };
        if rank(&self) > rank(&other) {
            self
        } else {
            other
        }
    }
}

impl<'a> ContextError<Span<'a>> for NomError<'a> {
    /// Keeps the innermost context, which describes the mistake most precisely.
    fn add_context(_input: Span<'a>, context: &'static str, mut other: Self) -> Self {
        other.context.get_or_insert(context);
        other
    }
}

impl fmt::Display for NomError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let at = ParseError::at(self.input);
        if let Some(keyword) = self.keyword {
            write!(f, "in `{}`: ", sanitize(keyword))?;
        }
        match self.context {
            Some(context) => write!(f, "{} ({:?}) at {at}", sanitize(context), self.code),
            None => write!(f, "{:?} at {at}", self.code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Domain, Parser, PddlFile, Problem};

    /// Typical mistakes are reported at the mistake itself, with the enclosing
    /// keyword and a reason where known.
    #[test]
    fn typical_mistakes_are_reported_where_they_are() {
        const HEAD: &str = "(define (domain d) (:requirements :strips) (:predicates (p ?x) (q))\n";
        let action = |body: &str| format!("{HEAD}  (:action a {body}))");
        let domain = |s: &str| Domain::from_str(s).unwrap_err();
        let problem = |s: &str| Problem::from_str(s).unwrap_err();
        let file = |s: &str| PddlFile::from_str(s).unwrap_err();
        // (label, parser, input, expected start of the error message)
        type Case<'a> = (&'a str, &'a dyn Fn(&str) -> ParseError, String, &'a str);
        let cases: [Case; 11] = [
            (
                "incomplete typed parameter",
                &domain,
                action(":parameters (?x - ) :precondition (p ?x) :effect (q)"),
                "line 2, column 32: in `:parameters`: unexpected `) :precondition",
            ),
            (
                "bad element in a precondition",
                &domain,
                action(":parameters (?x) :precondition (and (p ?x) 5) :effect (q)"),
                "line 2, column 57: in `:precondition`: unexpected `5) :effect",
            ),
            (
                "misspelled action keyword",
                &domain,
                action(":parameters (?x) :precondition (p ?x) :efect (q)"),
                "line 2, column 52: in `:action`: expected `)`: unexpected `:efect (q)",
            ),
            (
                "unclosed action followed by another section",
                &domain,
                format!("{HEAD}  (:action a :parameters () :effect (q)\n  (:predicates (r)))"),
                "line 3, column 3: in `:action`: expected `)`: unexpected `(:predicates (r)",
            ),
            (
                "requirement without colon",
                &domain,
                "(define (domain d) (:requirements strips))".to_string(),
                "line 1, column 35: in `:requirements`: unexpected `strips))`",
            ),
            (
                "duplicate section",
                &domain,
                "(define (domain d) (:predicates (p)) (:predicates (q)))".to_string(),
                "line 1, column 38: duplicate section: unexpected `(:predicates (q)",
            ),
            (
                "unknown section",
                &domain,
                "(define (domain d) (:predicats (p)))".to_string(),
                "line 1, column 20: unknown section: unexpected `(:predicats",
            ),
            (
                "missing :goal",
                &problem,
                "(define (problem p) (:domain d) (:init (p a))\n)".to_string(),
                "line 2, column 1: missing `:goal`: unexpected `)`",
            ),
            (
                "section written without parentheses",
                &problem,
                "(define (problem p) (:domain d) (:init) :goal (and))".to_string(),
                "line 1, column 41: unexpected `:goal (and))`",
            ),
            (
                "bad init element",
                &problem,
                "(define (problem p) (:domain d) (:init (p a) 5) (:goal (p a)))".to_string(),
                "line 1, column 46: in `:init`: expected `)`: unexpected `5) (:goal",
            ),
            (
                "domain read as part of a file",
                &file,
                format!(
                    "(define (problem q) (:domain d) (:init) (:goal (q)))\n{}",
                    action(":parameters (?x - ) :effect (q)")
                ),
                "line 3, column 32: in `:parameters`: unexpected `) :effect",
            ),
        ];
        for (label, parse, input, expected) in cases {
            let message = parse(&input).to_string();
            assert!(
                message.starts_with(expected),
                "{label}:\n  got {message}\n want {expected}"
            );
        }
    }

    /// The structured accessors match what `Display` shows.
    #[test]
    fn keyword_and_reason_accessors() {
        let error = Domain::from_str(
            "(define (domain d) (:predicates (p)) (:action a :parameters () :effect (p)",
        )
        .unwrap_err();
        assert_eq!(error.keyword(), Some(":action"));
        assert_eq!(error.reason(), Some("expected `)`"));
        assert_eq!(error.found(), None);

        let error = Domain::from_str("(define (domain d) (:predicates (p)) (:predicates (q)))")
            .unwrap_err();
        assert_eq!(error.keyword(), None);
        assert_eq!(error.reason(), Some("duplicate section"));
    }

    /// Keywords and reasons are sanitized like the excerpt, even though the
    /// crate only ever passes literals.
    #[test]
    fn keyword_and_reason_are_sanitized() {
        let mut error = NomError::with_context(Span::new("x"), "bad\x1b[2J\nreason");
        error.keyword = Some(":act\x07ion");
        let parsed = ParseError::from_nom("x", nom::Err::Failure(error.clone()));
        assert_eq!(parsed.keyword(), Some(r":act\u{7}ion"));
        assert_eq!(parsed.reason(), Some(r"bad\u{1b}[2J reason"));
        assert!(!error.to_string().chars().any(char::is_control), "{error}");
        assert!(
            !parsed.to_string().chars().any(char::is_control),
            "{parsed}"
        );
    }

    /// Bidi overrides, isolates and zero-width characters show up as escapes instead of
    /// spoofing the message.
    #[test]
    fn invisible_format_characters_are_escaped() {
        let input = "(define (domain d)\n(:\u{202E}evil\u{2066}\u{200B}\u{FEFF}))";
        let error = Domain::from_str(input).unwrap_err();
        let found = error.found().unwrap();
        assert!(found.is_ascii(), "{found:?}");
        assert_eq!(found, r"(:\u{202e}evil\u{2066}\u{200b}\u{feff}))");
    }

    /// Characters a hand-written list would miss are escaped too.
    #[test]
    fn every_non_ascii_character_is_escaped() {
        let input = "(define (domain d)\n(:\u{E0041}\u{FFF9}\u{206A}\u{FE0F}\u{00A0}x))";
        let found = Domain::from_str(input)
            .unwrap_err()
            .found()
            .unwrap()
            .to_string();
        assert_eq!(found, r"(:\u{e0041}\u{fff9}\u{206a}\u{fe0f}\u{a0}x))");
    }

    /// Keywords and reasons are bounded like the excerpt.
    #[test]
    fn keyword_and_reason_are_bounded() {
        let long = "x".repeat(10_000);
        let mut error = NomError::with_context(Span::new("x"), &long);
        error.keyword = Some(&long);
        let parsed = ParseError::from_nom("x", nom::Err::Failure(error.clone()));
        assert_eq!(parsed.keyword().unwrap().chars().count(), EXCERPT_CHARS + 3);
        assert_eq!(parsed.reason().unwrap().chars().count(), EXCERPT_CHARS + 3);
        assert!(error.to_string().len() < 200, "{}", error.to_string().len());
    }

    /// When alternatives fail at the same position, the one with a reason wins.
    #[test]
    fn or_prefers_the_described_error_on_a_tie() {
        use nom::branch::alt;
        use nom::bytes::complete::tag;
        use nom::error::context;
        use nom::Parser as _;

        let mut parser = alt((context("first", tag::<_, _, NomError>("x")), tag("y")));
        let Err(nom::Err::Error(error)) = parser.parse(Span::new("z")) else {
            panic!("expected an error");
        };
        assert_eq!(error.context, Some("first"));
    }

    #[test]
    fn each_missing_required_problem_section_is_named() {
        for (input, reason) in [
            (
                "(define (problem p) (:init) (:goal (and)))",
                "missing `:domain`",
            ),
            (
                "(define (problem p) (:domain d) (:goal (and)))",
                "missing `:init`",
            ),
            (
                "(define (problem p) (:domain d) (:init))",
                "missing `:goal`",
            ),
        ] {
            let error = Problem::from_str(input).unwrap_err();
            assert_eq!(error.reason(), Some(reason), "{input}");
            assert_eq!(error.found(), Some(")"), "{input}");
        }
    }

    /// `NomError` works with nom's `context` combinator and keeps the innermost context.
    #[test]
    fn nom_error_context_and_display() {
        use nom::bytes::complete::tag;
        use nom::error::context;
        use nom::Parser as _;

        let mut parser = context("outer", context("inner", tag::<_, _, NomError>("a")));
        let Err(nom::Err::Error(error)) = parser.parse(Span::new("b")) else {
            panic!("expected an error");
        };
        assert_eq!(error.context, Some("inner"));
        assert_eq!(
            error.to_string(),
            "inner (Tag) at line 1, column 1: unexpected `b`"
        );

        let plain = NomError::new(Span::new(""), ErrorKind::Eof);
        assert_eq!(
            plain.to_string(),
            "Eof at line 1, column 1: unexpected end of input"
        );
    }

    /// Committing after `:keywords` must not break keywords that are also predicate names.
    #[test]
    fn at_is_still_both_a_predicate_and_a_timed_literal() {
        let problem = Problem::from_str(
            "(define (problem p) (:domain d) (:init (at a b) (at 5 (p a))) (:goal (at a b)))",
        )
        .unwrap();
        assert_eq!(problem.init().len(), 2);
    }

    #[test]
    fn reports_line_column_and_bounded_excerpt() {
        let tail = "x".repeat(10_000);
        let input = format!("(define (domain d)\n  (:bogus {tail}))");
        let error = Domain::from_str(&input).unwrap_err();
        assert_eq!(error.line(), 2);
        assert_eq!(error.column(), 3);
        assert_eq!(error.offset(), 21);
        let found = error.found().unwrap();
        assert!(found.starts_with("(:bogus xxx"));
        assert_eq!(found.chars().count(), EXCERPT_CHARS + 3);
        assert!(error.to_string().len() < 100);
    }

    #[test]
    fn excerpt_flattens_newlines() {
        let error = Domain::from_str("(define (domain d)\n(:bogus\n))").unwrap_err();
        assert_eq!(error.found(), Some("(:bogus ))"));
        assert_eq!(
            error.to_string(),
            "line 2, column 1: unknown section: unexpected `(:bogus ))`"
        );
    }

    #[test]
    fn excerpt_escapes_control_characters() {
        let error = Domain::from_str("(define (domain d)\n(:\x1b[2J\0\x07))").unwrap_err();
        let found = error.found().unwrap();
        assert!(!found.chars().any(char::is_control), "{found:?}");
        assert_eq!(found, r"(:\u{1b}[2J\u{0}\u{7}))");
    }

    #[test]
    fn incomplete_maps_to_end_of_input() {
        let input = "ab\ncd";
        let error = ParseError::from_nom(input, nom::Err::Incomplete(nom::Needed::Unknown));
        assert_eq!((error.line(), error.column(), error.offset()), (2, 3, 5));
        assert_eq!(error.found(), None);
    }
}
