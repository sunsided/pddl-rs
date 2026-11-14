//! Provides parsers for `(define ...)` blocks: exactly one via [`Definition`],
//! or any number of them via [`PddlFile`].

use nom::branch::alt;
use nom::combinator::map;
use nom::Parser;

use crate::parsers::utilities::skip_whitespace_and_comments;
use crate::parsers::{parse_domain, parse_problem};
use crate::parsers::{ws2, ParseResult, Span};
use crate::types::{Definition, Domain, PddlFile, Problem};

/// Parses a single `(define ...)` block, determining whether it's a domain or problem.
fn parse_define_block<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Definition> {
    // `define` is not a `:keyword`, so a `(problem` header backtracks out of `parse_domain`.
    alt((
        map(parse_domain, Definition::Domain),
        map(parse_problem, Definition::Problem),
    ))
    .parse(input.into())
}

/// Parses zero or more `(define ...)` blocks, failing if a malformed
/// define block is encountered after valid ones.
fn parse_define_blocks<'a>(input: Span<'a>) -> ParseResult<'a, Vec<Definition>> {
    let mut blocks = Vec::new();
    let mut remaining = input;

    loop {
        (remaining, ()) = skip_whitespace_and_comments(remaining)?;
        let ahead = remaining.fragment().as_bytes();
        if !ahead
            .get(..7)
            .is_some_and(|p| p.eq_ignore_ascii_case(b"(define"))
        {
            break;
        }

        // Past `(define`, a malformed block is an error rather than the end of the list.
        let (next, block) = ws2(parse_define_block).parse(remaining)?;
        blocks.push(block);
        remaining = next;
    }

    Ok((remaining, blocks))
}

/// Parses a PDDL file containing zero or more domain and problem definitions.
///
/// This function consumes all `(define ...)` blocks in the input, returning
/// them grouped into a [`PddlFile`].
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_pddl_file, preamble::*};
/// let input = r#"
///     (define (domain briefcase-world)
///         (:requirements :strips)
///         (:predicates (at ?x))
///     )
///     (define (problem get-paid)
///         (:domain briefcase-world)
///         (:init (at home))
///         (:goal (at office))
///     )
/// "#;
///
/// let (remainder, pddl_file) = parse_pddl_file(input).unwrap();
/// assert!(remainder.is_empty());
/// assert_eq!(pddl_file.domain_count(), 1);
/// assert_eq!(pddl_file.problem_count(), 1);
/// ```
pub fn parse_pddl_file<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, PddlFile> {
    let (remainder, blocks) = parse_define_blocks(input.into())?;

    let mut domains = Vec::new();
    let mut problems = Vec::new();

    for block in blocks {
        match block {
            Definition::Domain(d) => domains.push(d),
            Definition::Problem(p) => problems.push(p),
        }
    }

    Ok((
        remainder,
        PddlFile::with_domains_and_problems(domains, problems),
    ))
}

/// Parses and returns only the [`Domain`] definitions from a PDDL file.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_domains, preamble::*};
/// let input = r#"
///     (define (domain d1) (:requirements :strips) (:predicates (p)))
///     (define (domain d2) (:requirements :strips) (:predicates (q)))
/// "#;
///
/// let (remainder, domains) = parse_domains(input).unwrap();
/// assert!(remainder.is_empty());
/// assert_eq!(domains.len(), 2);
/// ```
pub fn parse_domains<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Vec<Domain>> {
    let (remainder, blocks) = parse_define_blocks(input.into())?;

    let domains = blocks
        .into_iter()
        .filter_map(|block| match block {
            Definition::Domain(d) => Some(d),
            _ => None,
        })
        .collect();

    Ok((remainder, domains))
}

/// Parses and returns only the [`Problem`] definitions from a PDDL file.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problems, preamble::*};
/// let input = r#"
///     (define (problem p1) (:domain d) (:init) (:goal (and)))
///     (define (problem p2) (:domain d) (:init) (:goal (and)))
/// "#;
///
/// let (remainder, problems) = parse_problems(input).unwrap();
/// assert!(remainder.is_empty());
/// assert_eq!(problems.len(), 2);
/// ```
pub fn parse_problems<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Vec<Problem>> {
    let (remainder, blocks) = parse_define_blocks(input.into())?;

    let problems = blocks
        .into_iter()
        .filter_map(|block| match block {
            Definition::Problem(p) => Some(p),
            _ => None,
        })
        .collect();

    Ok((remainder, problems))
}

impl crate::parsers::Parser for Definition {
    type Item = Definition;

    /// Parses a single domain or problem definition, determining its kind
    /// from the `(define (domain ...))` / `(define (problem ...))` header.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Definition, Parser};
    /// let input = "; Uses (domain d)\n(define (problem q) (:domain d) (:init) (:goal (p)))";
    /// assert!(matches!(Definition::from_str(input), Ok(Definition::Problem(_))));
    ///
    /// // A second definition is trailing input, not part of this one.
    /// let two = "(define (domain d) (:predicates (p)))\n(define (domain e) (:predicates (q)))";
    /// assert!(Definition::from_str(two).is_err());
    /// ```
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        ws2(parse_define_block).parse(input.into())
    }
}

impl crate::parsers::Parser for PddlFile {
    type Item = PddlFile;

    /// Parses a PDDL file containing zero or more domain and problem definitions.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{PddlFile, Parser};
    /// let input = r#"
    ///     (define (domain briefcase-world)
    ///         (:requirements :strips)
    ///         (:predicates (at ?x))
    ///     )
    ///     (define (problem get-paid)
    ///         (:domain briefcase-world)
    ///         (:init (at home))
    ///         (:goal (at office))
    ///     )
    /// "#;
    ///
    /// let pddl_file = PddlFile::from_str(input).unwrap();
    /// assert_eq!(pddl_file.domain_count(), 1);
    /// assert_eq!(pddl_file.problem_count(), 1);
    /// ```
    ///
    /// ## See also
    /// See [`parse_pddl_file`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pddl_file(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Definition, Name, Parser, PddlFile};

    const DOMAIN: &str = "(define (domain d) (:predicates (p)))";
    const PROBLEM: &str = "(define (problem q) (:domain d) (:init) (:goal (p)))";

    #[test]
    fn define_is_matched_case_insensitively() {
        let file = PddlFile::from_str(&format!("{}\n{}", DOMAIN.to_uppercase(), PROBLEM)).unwrap();
        assert_eq!((file.domain_count(), file.problem_count()), (1, 1));
    }

    #[test]
    fn definition_kind_comes_from_the_header_not_comments() {
        for comment in ["", "; Uses (domain d)\n", "; Example (problem q)\n"] {
            let domain = Definition::from_str(&format!("{comment}{DOMAIN} ; end\n"));
            assert!(matches!(domain, Ok(Definition::Domain(_))), "{domain:?}");
            let problem = Definition::from_str(&format!("{comment}{PROBLEM}"));
            assert!(matches!(problem, Ok(Definition::Problem(_))), "{problem:?}");
        }
    }

    #[test]
    fn definition_rejects_empty_multiple_or_trailing_input() {
        for input in [
            String::new(),
            "; comment only\n".into(),
            format!("{DOMAIN}\n{PROBLEM}"),
            format!("{PROBLEM}\n{PROBLEM}"),
            format!("{DOMAIN}\ngarbage"),
            format!("text {DOMAIN}"),
            "(define (foo x))".into(),
            "(define (domain".into(),
        ] {
            assert!(Definition::from_str(&input).is_err(), "accepted {input:?}");
        }
    }

    /// Structural errors inside a definition must be reported where they are,
    /// not as the fallback `problem` alternative failing at `(domain`.
    #[test]
    fn section_errors_are_reported_at_the_offending_section() {
        let cases = [
            (
                "misspelled section",
                "(define (domain d)\n  (:requirement :strips)\n  (:predicates (p)))",
                "(:requirement",
            ),
            (
                "duplicate section",
                "(define (domain d)\n  (:predicates (p))\n  (:predicates (q)))",
                "(:predicates (q)",
            ),
            (
                "missing :goal",
                "(define (problem p) (:domain d) (:init)\n)",
                ")",
            ),
        ];
        for (label, input, at) in cases {
            let error = PddlFile::from_str(input).unwrap_err();
            assert_eq!(Some(error.offset()), input.rfind(at), "{label}: {error}");
        }
    }

    #[test]
    fn parse_single_domain_and_problem() {
        let input = r#"
            (define (domain briefcase-world)
                (:requirements :strips)
                (:predicates (at ?x))
            )
            (define (problem get-paid)
                (:domain briefcase-world)
                (:init (at home))
                (:goal (at office))
            )
        "#;

        let (remainder, pddl_file) = super::parse_pddl_file(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(pddl_file.domain_count(), 1);
        assert_eq!(pddl_file.problem_count(), 1);
        assert_eq!(pddl_file.domains[0].name(), &Name::new("briefcase-world"));
        assert_eq!(pddl_file.problems[0].name(), &Name::new("get-paid"));
    }

    #[test]
    fn parse_two_domains_and_one_problem() {
        let input = r#"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (domain d2)
                (:requirements :strips)
                (:predicates (q))
            )
            (define (problem p1)
                (:domain d1)
                (:init)
                (:goal (and))
            )
        "#;

        let (remainder, pddl_file) = super::parse_pddl_file(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(pddl_file.domain_count(), 2);
        assert_eq!(pddl_file.problem_count(), 1);
        assert_eq!(pddl_file.domains[0].name(), &Name::new("d1"));
        assert_eq!(pddl_file.domains[1].name(), &Name::new("d2"));
    }

    #[test]
    fn parse_one_domain_and_two_problems() {
        let input = r#"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (problem p1)
                (:domain d1)
                (:init)
                (:goal (and))
            )
            (define (problem p2)
                (:domain d1)
                (:init)
                (:goal (and))
            )
        "#;

        let (remainder, pddl_file) = super::parse_pddl_file(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(pddl_file.domain_count(), 1);
        assert_eq!(pddl_file.problem_count(), 2);
    }

    #[test]
    fn parse_mixed_order_problem_before_domain() {
        let input = r#"
            (define (problem p1)
                (:domain d1)
                (:init)
                (:goal (and))
            )
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
        "#;

        let (remainder, pddl_file) = super::parse_pddl_file(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(pddl_file.domain_count(), 1);
        assert_eq!(pddl_file.problem_count(), 1);
        // Problem comes first in the file
        assert_eq!(pddl_file.problems[0].name(), &Name::new("p1"));
        assert_eq!(pddl_file.domains[0].name(), &Name::new("d1"));
    }

    #[test]
    fn parse_file_with_leading_comments() {
        let input = r#"
            ; This is a comment
            ; Another comment
            (define (domain test)
                (:requirements :strips)
                (:predicates (p))
            )
        "#;

        let (remainder, pddl_file) = super::parse_pddl_file(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(pddl_file.domain_count(), 1);
        assert_eq!(pddl_file.domains[0].name(), &Name::new("test"));
    }

    #[test]
    fn parse_empty_file() {
        let input = "";

        let (remainder, pddl_file) = super::parse_pddl_file(input).unwrap();
        assert!(remainder.is_empty());
        assert!(pddl_file.is_empty());
        assert_eq!(pddl_file.domain_count(), 0);
        assert_eq!(pddl_file.problem_count(), 0);
    }

    #[test]
    fn parse_file_with_only_whitespace_and_comments() {
        let input = r#"; Just a comment

; Another comment
"#;

        let (remainder, pddl_file) = super::parse_pddl_file(input).unwrap();
        assert!(remainder.is_empty());
        assert!(pddl_file.is_empty());
    }

    #[test]
    fn parse_using_parser_trait() {
        let input = r#"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (problem p1)
                (:domain d1)
                (:init)
                (:goal (and))
            )
        "#;

        let pddl_file = PddlFile::from_str(input).unwrap();
        assert_eq!(pddl_file.domain_count(), 1);
        assert_eq!(pddl_file.problem_count(), 1);
    }

    #[test]
    fn parse_domains_only() {
        let input = r#"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (domain d2)
                (:requirements :strips)
                (:predicates (q))
            )
        "#;

        let (remainder, domains) = super::parse_domains(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(domains.len(), 2);
        assert_eq!(domains[0].name(), &Name::new("d1"));
        assert_eq!(domains[1].name(), &Name::new("d2"));
    }

    #[test]
    fn parse_problems_only() {
        let input = r#"
            (define (problem p1)
                (:domain d1)
                (:init)
                (:goal (and))
            )
            (define (problem p2)
                (:domain d1)
                (:init)
                (:goal (and))
            )
        "#;

        let (remainder, problems) = super::parse_problems(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(problems.len(), 2);
    }

    #[test]
    fn parse_rejects_malformed_define_block_after_valid_ones() {
        let input = r#"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (domain d2)
                (:requirements
            )
        "#;

        let result = super::parse_pddl_file(input);
        assert!(
            result.is_err(),
            "Should reject file with malformed define block after valid ones"
        );
    }
}
