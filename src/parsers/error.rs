use std::fmt;

use crate::parsers::{NomError, Span};

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
/// assert_eq!(error.to_string(), "line 2, column 3: unexpected `(:bogus))`");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    offset: usize,
    line: u32,
    column: usize,
    found: Option<String>,
}

impl ParseError {
    /// The byte offset into the input at which parsing stopped.
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// The 1-based line at which parsing stopped.
    pub const fn line(&self) -> u32 {
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

    /// Converts a nom error for `input` into a [`ParseError`].
    pub(crate) fn from_nom(input: &str, error: nom::Err<NomError<'_>>) -> Self {
        let at = match error {
            nom::Err::Error(e) | nom::Err::Failure(e) => e.input,
            // The parsers are complete, but report the end of input if one ever is not.
            nom::Err::Incomplete(_) => nom::Input::take_from(&Span::new(input), input.len()),
        };
        Self::at(at)
    }

    fn at(span: Span<'_>) -> Self {
        let rest = span.fragment();
        let found = if rest.is_empty() {
            None
        } else {
            let mut excerpt: String = rest
                .chars()
                .take(EXCERPT_CHARS)
                .map(|c| if c.is_whitespace() { ' ' } else { c })
                .collect();
            if rest.chars().nth(EXCERPT_CHARS).is_some() {
                excerpt.push_str("...");
            }
            Some(excerpt)
        };
        Self {
            offset: span.location_offset(),
            line: span.location_line(),
            column: span.get_utf8_column(),
            found,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}: ", self.line, self.column)?;
        match &self.found {
            Some(found) => write!(f, "unexpected `{found}`"),
            None => f.write_str("unexpected end of input"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Domain, Parser};

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
            "line 2, column 1: unexpected `(:bogus ))`"
        );
    }

    #[test]
    fn incomplete_maps_to_end_of_input() {
        let input = "ab\ncd";
        let error = ParseError::from_nom(input, nom::Err::Incomplete(nom::Needed::Unknown));
        assert_eq!((error.line(), error.column(), error.offset()), (2, 3, 5));
        assert_eq!(error.found(), None);
    }
}
