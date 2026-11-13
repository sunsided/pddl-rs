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

    /// Why parsing stopped, if known, e.g. `duplicate section`, `missing `:goal``
    /// or `in `:action`` for a mistake inside an action definition.
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Converts a nom error for `input` into a [`ParseError`].
    pub(crate) fn from_nom(input: &str, error: nom::Err<NomError<'_>>) -> Self {
        let (mut at, reason) = match error {
            nom::Err::Error(e) | nom::Err::Failure(e) => (e.input, e.context),
            // The parsers are complete, but report the end of input if one ever is not.
            nom::Err::Incomplete(_) => {
                (nom::Input::take_from(&Span::new(input), input.len()), None)
            }
        };
        // Point at the next token rather than at whitespace or comments before it.
        if let Ok((token, ())) = crate::parsers::utilities::skip_whitespace_and_comments(at) {
            at = token;
        }
        let mut error = Self::at(at);
        error.reason = reason.map(|reason| {
            if reason.starts_with(':') {
                format!("in `{reason}`")
            } else {
                reason.to_string()
            }
        });
        error
    }

    fn at(span: Span<'_>) -> Self {
        let rest = span.fragment();
        let found = if rest.is_empty() {
            None
        } else {
            let mut excerpt: String = rest
                .chars()
                .take(EXCERPT_CHARS)
                // Control characters (e.g. ANSI escapes) from untrusted input must not reach terminals or logs.
                .map(|c| {
                    if c.is_whitespace() || c.is_control() {
                        ' '
                    } else {
                        c
                    }
                })
                .collect();
            if rest.chars().nth(EXCERPT_CHARS).is_some() {
                excerpt.push_str("...");
            }
            Some(excerpt)
        };
        Self {
            offset: span.location_offset(),
            line: span.location_line() as usize,
            column: span.get_utf8_column(),
            found,
            reason: None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}: ", self.line, self.column)?;
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

/// The error type of the individual `parse_*` combinators.
///
/// [`Parser::from_str`](crate::parsers::Parser::from_str) converts it into a [`ParseError`].
/// Like `nom::error::Error`, it records where (`input`) and in which combinator
/// (`code`) parsing failed; it additionally carries an optional `context`:
/// either a reason such as `duplicate section`, or the PDDL keyword of the
/// enclosing form (starting with `:`, e.g. `:action`) in which parsing failed.
#[derive(Debug, Clone, PartialEq)]
pub struct NomError<'a> {
    /// The remaining input at the position of the error.
    pub input: Span<'a>,
    /// The nom combinator that failed.
    pub code: ErrorKind,
    /// Why parsing failed, if known.
    pub context: Option<&'a str>,
}

impl<'a> NomError<'a> {
    /// Creates an error at `input` without a context.
    pub const fn new(input: Span<'a>, code: ErrorKind) -> Self {
        Self {
            input,
            code,
            context: None,
        }
    }

    /// Creates an error at `input` with the given reason.
    pub(crate) const fn with_context(input: Span<'a>, context: &'a str) -> Self {
        Self {
            input,
            code: ErrorKind::Verify,
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
    /// the input: that alternative most likely was the one intended.
    fn or(self, other: Self) -> Self {
        if self.input.location_offset() > other.input.location_offset() {
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
        match self.context {
            Some(context) => write!(f, "{context} ({:?}) at {at}", self.code),
            None => write!(f, "{:?} at {at}", self.code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Domain, Parser, PddlFile, Problem};

    /// Typical mistakes are reported at the mistake itself, with a reason.
    #[test]
    fn typical_mistakes_are_reported_where_they_are() {
        const HEAD: &str = "(define (domain d) (:requirements :strips) (:predicates (p ?x) (q))\n";
        let action = |body: &str| format!("{HEAD}  (:action a {body}))");
        let domain = |s: &str| Domain::from_str(s).unwrap_err();
        let problem = |s: &str| Problem::from_str(s).unwrap_err();
        let file = |s: &str| PddlFile::from_str(s).unwrap_err();
        type Case<'a> = (
            &'a str,
            &'a dyn Fn(&str) -> ParseError,
            String,
            (usize, usize),
            &'a str,
            &'a str,
        );
        let cases: [Case; 9] = [
            (
                "incomplete typed parameter",
                &domain,
                action(":parameters (?x - ) :precondition (p ?x) :effect (q)"),
                (2, 32),
                "in `:parameters`",
                ")",
            ),
            (
                "bad element in a precondition",
                &domain,
                action(":parameters (?x) :precondition (and (p ?x) 5) :effect (q)"),
                (2, 57),
                "in `:precondition`",
                "5)",
            ),
            (
                "misspelled action keyword",
                &domain,
                action(":parameters (?x) :precondition (p ?x) :efect (q)"),
                (2, 52),
                "in `:action`",
                ":efect",
            ),
            (
                "requirement without colon",
                &domain,
                "(define (domain d) (:requirements strips))".to_string(),
                (1, 35),
                "in `:requirements`",
                "strips",
            ),
            (
                "duplicate section",
                &domain,
                "(define (domain d) (:predicates (p)) (:predicates (q)))".to_string(),
                (1, 38),
                "duplicate section",
                "(:predicates (q)",
            ),
            (
                "unknown section",
                &domain,
                "(define (domain d) (:predicats (p)))".to_string(),
                (1, 20),
                "unknown section",
                "(:predicats",
            ),
            (
                "missing :goal",
                &problem,
                "(define (problem p) (:domain d) (:init (p a))\n)".to_string(),
                (2, 1),
                "missing `:goal`",
                ")",
            ),
            (
                "bad init element",
                &problem,
                "(define (problem p) (:domain d) (:init (p a) 5) (:goal (p a)))".to_string(),
                (1, 46),
                "in `:init`",
                "5)",
            ),
            (
                "domain read as part of a file",
                &file,
                format!(
                    "(define (problem q) (:domain d) (:init) (:goal (q)))\n{}",
                    action(":parameters (?x - ) :effect (q)")
                ),
                (3, 32),
                "in `:parameters`",
                ")",
            ),
        ];
        for (label, parse, input, (line, column), reason, found) in cases {
            let error = parse(&input);
            assert_eq!(
                (error.line(), error.column()),
                (line, column),
                "{label}: {error}"
            );
            assert_eq!(error.reason(), Some(reason), "{label}: {error}");
            assert!(
                error.found().unwrap().starts_with(found),
                "{label}: {error}"
            );
        }
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
    fn excerpt_replaces_control_characters() {
        let error = Domain::from_str("(define (domain d)\n(:\x1b[2J\0\x07))").unwrap_err();
        let found = error.found().unwrap();
        assert!(!found.chars().any(char::is_control), "{found:?}");
        assert_eq!(found, "(: [2J  ))");
    }

    #[test]
    fn incomplete_maps_to_end_of_input() {
        let input = "ab\ncd";
        let error = ParseError::from_nom(input, nom::Err::Incomplete(nom::Needed::Unknown));
        assert_eq!((error.line(), error.column(), error.offset()), (2, 3, 5));
        assert_eq!(error.found(), None);
    }
}
