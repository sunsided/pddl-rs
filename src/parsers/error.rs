//! The public [`ParseError`] and the grammar's internal [`Error`].

use std::borrow::Cow;
use std::fmt;
use std::ops::Range;

/// Maximum number of characters of the remaining input kept in a [`ParseError`].
const EXCERPT_CHARS: usize = 40;

/// An error returned by [`Parser::from_str`](crate::parsers::Parser::from_str).
///
/// Describes where parsing stopped and which input was found there. The
/// excerpt is bounded, so formatting the error never echoes the whole
/// remaining input.
///
/// With the `miette` feature, `ParseError` implements `miette::Diagnostic`.
/// Attach the parsed text to render the error with the offending input marked:
/// `miette::Report::new(error).with_source_code(input)`.
///
/// ## Example
/// ```
/// # use pddl::{Domain, Parser};
/// let error = Domain::from_str("(define (domain d)\n  (:bogus))").unwrap_err();
/// assert_eq!((error.line(), error.column()), (2, 3));
/// assert_eq!(error.span(), 21..28);
/// assert_eq!(error.found(), Some("(:bogus))"));
/// assert_eq!(error.reason(), Some("unknown section"));
/// assert_eq!(error.to_string(), "line 2, column 3: unknown section: unexpected `(:bogus))`");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Byte offset of the error in the input.
    offset: usize,
    /// Length in bytes of the input the error covers; see [`ParseError::span`].
    len: usize,
    /// 1-based line of `offset`.
    line: usize,
    /// 1-based column of `offset`, in characters.
    column: usize,
    /// Sanitized excerpt of the input from `offset`, or `None` at the end of the input.
    found: Option<String>,
    /// Sanitized `:keyword` of the innermost form containing the error.
    keyword: Option<String>,
    /// Sanitized description of why parsing stopped.
    reason: Option<String>,
    /// Byte offset of the `(` that is never closed, for an error at the end of the input.
    #[cfg_attr(not(feature = "miette"), allow(dead_code))]
    unclosed: Option<usize>,
}

impl ParseError {
    /// The byte offset into the input at which parsing stopped.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// The byte range of the input that parsing stopped at: an atom, the start
    /// of a list up to its first atom (e.g. `(:bogus`), a single parenthesis,
    /// or an empty range at the end of the input.
    #[must_use]
    pub const fn span(&self) -> Range<usize> {
        self.offset..self.offset + self.len
    }

    /// The 1-based line at which parsing stopped.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// The 1-based column (in characters) at which parsing stopped.
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }

    /// A short excerpt of the input at which parsing stopped,
    /// or `None` if the input ended early.
    #[must_use]
    pub fn found(&self) -> Option<&str> {
        self.found.as_deref()
    }

    /// The PDDL keyword of the innermost form in which parsing stopped, if known,
    /// e.g. `:precondition` for a mistake inside an action's precondition.
    ///
    /// Made safe for display like [`found`](Self::found): characters outside
    /// printable ASCII are escaped, and long values are cut and end in `...`.
    #[must_use]
    pub fn keyword(&self) -> Option<&str> {
        self.keyword.as_deref()
    }

    /// Why parsing stopped, if known, e.g. `duplicate section`, ``missing `:goal` ``
    /// or ``expected `)` `` for a form that isn't closed.
    ///
    /// Made safe for display like [`found`](Self::found): characters outside
    /// printable ASCII are escaped, and long values are cut and end in `...`.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Describes `error` within `input`.
    pub(crate) fn new(input: &str, error: &Error) -> Self {
        let offset = error.offset.min(input.len());
        let before = &input[..offset];
        let line_start = before.rfind('\n').map_or(0, |i| i + 1);
        let rest = &input[offset..];
        Self {
            offset,
            len: error.len.min(rest.len()),
            line: before.matches('\n').count() + 1,
            column: before[line_start..].chars().count() + 1,
            found: (!rest.is_empty()).then(|| sanitize(rest)),
            keyword: error.keyword.as_deref().map(sanitize),
            reason: error.reason.map(sanitize),
            unclosed: error.unclosed,
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

#[cfg(feature = "miette")]
#[cfg_attr(docsrs, doc(cfg(feature = "miette")))]
impl miette::Diagnostic for ParseError {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new("pddl::parse"))
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        let text = self.reason.clone().unwrap_or_else(|| match &self.found {
            Some(_) => "unexpected".to_string(),
            None => "unexpected end of input".to_string(),
        });
        let label = miette::LabeledSpan::new_primary_with_span(Some(text), self.span());
        // An error at the end of the input has an empty span, which a report can't mark;
        // the `(` that is never closed shows where the missing `)` belongs.
        let unclosed = self.unclosed.map(|offset| {
            miette::LabeledSpan::new_with_span(
                Some("this `(` is never closed".to_string()),
                offset..offset + 1,
            )
        });
        Some(Box::new(std::iter::once(label).chain(unclosed)))
    }
}

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

/// A grammar error: where it happened and, if known, in which form and why.
///
/// [`Parser::from_str`](crate::parsers::Parser::from_str) turns it into a [`ParseError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Error {
    /// Byte offset of the error in the input.
    pub offset: usize,
    /// Length in bytes of the input the error covers.
    pub len: usize,
    /// The `:keyword` of the innermost form the error is in, such as `:action`.
    pub keyword: Option<Cow<'static, str>>,
    /// Why parsing failed, such as `duplicate section`.
    pub reason: Option<&'static str>,
    /// A fatal error ends parsing; otherwise the grammar may try another alternative.
    /// Errors inside a `:keyword` form are fatal: nothing else can start with it.
    pub fatal: bool,
    /// Byte offset of the `(` that is never closed, for an error at the end of the input.
    pub unclosed: Option<usize>,
}

/// The result of a grammar rule.
pub(crate) type R<T> = Result<T, Error>;

impl Error {
    /// An error covering `len` bytes at `offset`.
    pub const fn at(offset: usize, len: usize) -> Self {
        Self {
            offset,
            len,
            keyword: None,
            reason: None,
            fatal: false,
            unclosed: None,
        }
    }

    /// Sets why parsing failed.
    pub fn reason(mut self, reason: &'static str) -> Self {
        self.reason = Some(reason);
        self
    }

    /// Sets the `:keyword` of the form the error is in.
    pub fn keyword(mut self, keyword: impl Into<Cow<'static, str>>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// Makes the error fatal, so no alternative is tried after it.
    pub const fn fatal(mut self) -> Self {
        self.fatal = true;
        self
    }

    /// When all alternatives failed, keeps the error that got furthest into
    /// the input: that alternative most likely was the one intended. On a tie,
    /// prefers the error that knows more about why it failed, then the later one.
    pub fn or(self, other: Self) -> Self {
        let rank = |e: &Self| {
            let described = e.reason.is_some() || e.keyword.is_some();
            (e.offset, described)
        };
        if rank(&self) > rank(&other) {
            self
        } else {
            other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Domain, Parser, PddlFile, Problem};
    use assert2::check;

    /// Typical mistakes are reported at the mistake itself, with the enclosing
    /// keyword and a reason where known.
    #[test]
    fn typical_mistakes_are_reported_where_they_are() {
        // (label, parser, input, expected start of the error message)
        type Case<'a> = (&'a str, &'a dyn Fn(&str) -> ParseError, String, &'a str);
        const HEAD: &str = "(define (domain d) (:requirements :strips) (:predicates (p ?x) (q))\n";
        let action = |body: &str| format!("{HEAD}  (:action a {body}))");
        let domain = |s: &str| Domain::from_str(s).unwrap_err();
        let problem = |s: &str| Problem::from_str(s).unwrap_err();
        let file = |s: &str| PddlFile::from_str(s).unwrap_err();

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
                "line 1, column 46: in `:init`: unexpected `5) (:goal",
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
            check!(
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
        check!(error.keyword() == Some(":action"));
        check!(error.reason() == Some("expected `)`"));
        check!(error.found() == None);

        let error = Domain::from_str("(define (domain d) (:predicates (p)) (:predicates (q)))")
            .unwrap_err();
        check!(error.keyword() == None);
        check!(error.reason() == Some("duplicate section"));
    }

    /// Keywords and reasons are sanitized like the excerpt.
    #[test]
    fn keyword_and_reason_are_sanitized() {
        let error = Error::at(0, 1)
            .reason("bad\x1b[2J\nreason")
            .keyword(":act\x07ion");
        let parsed = ParseError::new("x", &error);
        check!(parsed.keyword() == Some(r":act\u{7}ion"));
        check!(parsed.reason() == Some(r"bad\u{1b}[2J reason"));
        check!(
            !parsed.to_string().chars().any(char::is_control),
            "{parsed}"
        );
    }

    /// The keyword of an unclosed form comes from the input, so it is sanitized too.
    #[test]
    fn keyword_of_an_unclosed_form_is_sanitized() {
        let input = "(:\u{202E}x";
        let unclosed = crate::parsers::sexpr::read(input)
            .unwrap()
            .unbalanced
            .unwrap();
        let error = ParseError::new(input, &unclosed);
        check!(error.keyword() == Some(r":\u{202e}x"));
    }

    /// Bidi overrides, isolates and zero-width characters show up as escapes instead of
    /// spoofing the message.
    #[test]
    fn invisible_format_characters_are_escaped() {
        let input = "(define (domain d)\n(:\u{202E}evil\u{2066}\u{200B}\u{FEFF}))";
        let error = Domain::from_str(input).unwrap_err();
        let found = error.found().unwrap();
        check!(found.is_ascii(), "{found:?}");
        check!(found == r"(:\u{202e}evil\u{2066}\u{200b}\u{feff}))");
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
        check!(found == r"(:\u{e0041}\u{fff9}\u{206a}\u{fe0f}\u{a0}x))");
    }

    /// Keywords and reasons are bounded like the excerpt.
    #[test]
    fn keyword_and_reason_are_bounded() {
        let long: &'static str = "x".repeat(10_000).leak();
        let parsed = ParseError::new("x", &Error::at(0, 1).reason(long).keyword(long));
        check!(parsed.keyword().unwrap().chars().count() == EXCERPT_CHARS + 3);
        check!(parsed.reason().unwrap().chars().count() == EXCERPT_CHARS + 3);
    }

    /// When alternatives fail at the same position, the one with a reason wins.
    #[test]
    fn or_prefers_the_furthest_then_the_described_error() {
        let plain = Error::at(3, 1);
        let described = Error::at(3, 1).reason("why");
        check!(plain.clone().or(described.clone()) == described);
        check!(described.clone().or(plain.clone()) == described);
        check!(described.or(Error::at(4, 1)) == Error::at(4, 1));
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
            check!(error.reason() == Some(reason), "{input}");
            check!(error.found() == Some(")"), "{input}");
        }
    }

    #[test]
    fn reports_line_column_and_bounded_excerpt() {
        let tail = "x".repeat(10_000);
        let input = format!("(define (domain d)\n  (:bogus {tail}))");
        let error = Domain::from_str(&input).unwrap_err();
        check!(error.line() == 2);
        check!(error.column() == 3);
        check!(error.offset() == 21);
        check!(error.span() == (21..28));
        let found = error.found().unwrap();
        check!(found.starts_with("(:bogus xxx"));
        check!(found.chars().count() == EXCERPT_CHARS + 3);
        check!(error.to_string().len() < 100);
    }

    #[test]
    fn excerpt_flattens_newlines() {
        let error = Domain::from_str("(define (domain d)\n(:bogus\n))").unwrap_err();
        check!(error.found() == Some("(:bogus ))"));
        check!(error.to_string() == "line 2, column 1: unknown section: unexpected `(:bogus ))`");
    }

    #[test]
    fn excerpt_escapes_control_characters() {
        let error = Domain::from_str("(define (domain d)\n(:\x1b[2J\0\x07))").unwrap_err();
        let found = error.found().unwrap();
        check!(!found.chars().any(char::is_control), "{found:?}");
        check!(found == r"(:\u{1b}[2J\u{0}\u{7}))");
    }

    #[test]
    fn end_of_input_has_no_excerpt() {
        let error = Domain::from_str("ab\n").unwrap_err();
        check!(error.found() == Some("ab "));
        let error = ParseError::new("ab\ncd", &Error::at(5, 0));
        check!((error.line(), error.column(), error.offset()) == (2, 3, 5));
        check!(error.found() == None);
        check!(error.span() == (5..5));
    }

    #[cfg(feature = "miette")]
    #[test]
    fn diagnostic_labels_the_span() {
        use miette::Diagnostic;
        let error = Domain::from_str("(define (domain d) (:bogus))").unwrap_err();
        let labels: Vec<_> = error.labels().unwrap().collect();
        check!(labels.len() == 1);
        check!((labels[0].offset(), labels[0].len()) == (19, 7));
        check!(labels[0].label() == Some("unknown section"));
        check!(error.code().unwrap().to_string() == "pddl::parse");
    }

    #[test]
    fn column_counts_characters() {
        let error = ParseError::new("ä(x", &Error::at(3, 1));
        check!(error.column() == 3);
    }
}
