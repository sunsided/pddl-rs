//! Turns a [`ParseError`] into a compile-error message.
//!
//! Stable Rust cannot point a span inside a string literal, so every error
//! underlines the whole literal. The message does the locating instead: it
//! starts with a `file:line:column` that terminals and editors can jump to,
//! and ends with the offending PDDL line and a marker under the error:
//!
//! ```text
//! domains/blocks.pddl:4:4: unknown section: unexpected `(:predicats (on ?x ?y))`
//!   = help: did you mean `:predicates`?
//!    |
//!  4 |   (:predicats (on ?x ?y))
//!    |    ^~~~~~~~~~
//! ```

use pddl::ParseError;

/// Where the PDDL of a macro call lives, for naming error locations.
#[derive(Debug, Clone)]
pub enum Origin {
    /// In a string literal of a `.rs` file.
    Inline {
        /// The `.rs` file as the compiler shows it; `None` outside the compiler.
        file: Option<String>,
        /// The 1-based line on which the literal starts; `None` outside the compiler.
        line: Option<usize>,
        /// Whether each PDDL line is a line of the `.rs` file, as in a raw
        /// string or a string without escapes. A `\n` escape or a `\` line
        /// continuation breaks that.
        lines_match: bool,
    },
    /// In a PDDL file.
    File {
        /// The path as written in the macro call.
        path: String,
    },
}

/// Formats an error as a compile-error message: location, description, help,
/// and the source line with a marker.
pub fn message(error: &ParseError, source: &str, origin: &Origin) -> String {
    let mut message = format!("{}: {}", location(error, origin), description(error));
    if let Some(help) = error.help() {
        message.push_str("\n  = help: ");
        message.push_str(help);
    }

    if let Some(snippet) = snippet(error, source) {
        message.push('\n');
        message.push_str(&snippet);
    }

    message
}

/// The location prefix: `file:line:column` for a PDDL file, the matching line
/// of the `.rs` file for a literal, the literal's line plus the PDDL line and
/// column when lines don't match, or the PDDL line and column alone.
fn location(error: &ParseError, origin: &Origin) -> String {
    let (line, column) = (error.line(), error.column());
    match origin {
        Origin::File { path } => format!("{path}:{line}:{column}"),
        // The literal's first line also holds the opening quote, so a column
        // there would be off; later lines map one to one.
        Origin::Inline {
            file: Some(file),
            line: Some(start),
            lines_match: true,
        } if line > 1 => format!("{file}:{}:{column}", start + line - 1),
        Origin::Inline {
            file: Some(file),
            line: Some(start),
            lines_match: true,
        } => format!("{file}:{start}"),
        Origin::Inline {
            file: Some(file),
            line: Some(start),
            lines_match: false,
        } => format!("{file}:{start}: line {line}, column {column}"),
        Origin::Inline { .. } => format!("line {line}, column {column}"),
    }
}

/// The error's [`Display`](std::fmt::Display) without its leading
/// `line L, column C: `, which [`location`] replaces.
fn description(error: &ParseError) -> String {
    let text = error.to_string();
    let prefix = format!("line {}, column {}: ", error.line(), error.column());

    text.strip_prefix(&prefix).unwrap_or(&text).to_string()
}

/// The source line of the error with a `^~~~` marker under the error's span,
/// clipped to that line; `None` if the line does not exist (an error at the
/// very end of the input).
fn snippet(error: &ParseError, source: &str) -> Option<String> {
    let text = source.lines().nth(error.line() - 1)?;
    let column = error.column() - 1;
    let available = text.chars().count().saturating_sub(column);
    let span_chars = source
        .get(error.span())
        .map_or(1, |span| span.chars().take_while(|&c| c != '\n').count());
    let width = span_chars.clamp(1, available.max(1));

    let number = error.line().to_string();
    let gutter = " ".repeat(number.len());
    let marker = format!("{}^{}", indent(text, column), "~".repeat(width - 1));

    Some(format!(
        "{gutter} |\n{number} | {text}\n{gutter} | {}",
        marker.trim_end()
    ))
}

/// Blank padding as wide as the first `column` characters of `text`: tabs
/// stay tabs so the marker lines up however wide the terminal shows them.
fn indent(text: &str, column: usize) -> String {
    text.chars()
        .chain(std::iter::repeat(' '))
        .take(column)
        .map(|c| if c == '\t' { '\t' } else { ' ' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pddl::{Domain, Parser};

    /// Parses a broken domain and formats its error.
    fn format(source: &str, origin: &Origin) -> String {
        message(&Domain::from_str(source).unwrap_err(), source, origin)
    }

    #[test]
    fn file_errors_start_with_path_line_and_column() {
        let origin = Origin::File {
            path: "d.pddl".to_string(),
        };
        let message = format("(define (domain d)\n  (:bogus))", &origin);
        assert_eq!(
            message,
            "d.pddl:2:3: unknown section: unexpected `(:bogus))`\n  |\n2 |   (:bogus))\n  |   ^~~~~~~"
        );
    }

    #[test]
    fn inline_errors_map_to_the_rust_line() {
        let origin = Origin::Inline {
            file: Some("src/main.rs".to_string()),
            line: Some(10),
            lines_match: true,
        };
        let message = format("\n(define (domain d)\n  (:bogus))", &origin);
        assert!(
            message.starts_with("src/main.rs:12:3: unknown section"),
            "{message}"
        );

        let message = format("(define (domain d) (:bogus))", &origin);
        assert!(
            message.starts_with("src/main.rs:10: unknown section"),
            "{message}"
        );
    }

    #[test]
    fn escaped_literals_keep_the_pddl_position() {
        let origin = Origin::Inline {
            file: Some("src/main.rs".to_string()),
            line: Some(10),
            lines_match: false,
        };
        let message = format("(define (domain d)\n  (:bogus))", &origin);
        assert!(
            message.starts_with("src/main.rs:10: line 2, column 3: unknown section"),
            "{message}"
        );
    }

    #[test]
    fn marker_keeps_tabs() {
        let origin = Origin::File {
            path: "d.pddl".to_string(),
        };
        let message = format("(define (domain d)\n\t(:bogus))", &origin);
        assert!(message.ends_with("\n  | \t^~~~~~~"), "{message:?}");
    }

    #[test]
    fn error_at_end_of_input_has_no_snippet() {
        let origin = Origin::Inline {
            file: None,
            line: None,
            lines_match: true,
        };
        let message = format("(define (domain d)", &origin);
        assert!(message.starts_with("line 1, column "), "{message}");
    }
}
