//! Provides parsers for multi-definition PDDL files.

use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::Parser;

use crate::parsers::{parse_domain, parse_problem};
use crate::parsers::{ws2, ParseResult, Span};
use crate::types::{Domain, PddlFile, Problem};

/// Intermediate representation for a single define block.
enum DefineBlock {
    Domain(Domain),
    Problem(Problem),
}

/// Parses a single `(define ...)` block, determining whether it's a domain or problem.
fn parse_define_block<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, DefineBlock> {
    let input = input.into();

    // First, skip whitespace/comments, then parse `(define`
    // Then peek at the next keyword to determine domain vs problem
    // Then dispatch to the appropriate parser

    // We need to consume `(define` then look at the next token
    // Both parse_domain and parse_problem expect the full `(define (domain|problem ...))` structure
    // So we use alt with both parsers - but we need to ensure backtracking works

    // The trick: both parsers start with ws2(prefix_expr("define", ...))
    // alt will try parse_domain first; if it fails (because the keyword is "problem"),
    // it backtracks and tries parse_problem

    alt((
        map(parse_domain, DefineBlock::Domain),
        map(parse_problem, DefineBlock::Problem),
    ))
    .parse(input)
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
    let (remainder, blocks) = ws2(many0(ws2(parse_define_block))).parse(input.into())?;

    let mut domains = Vec::new();
    let mut problems = Vec::new();

    for block in blocks {
        match block {
            DefineBlock::Domain(d) => domains.push(d),
            DefineBlock::Problem(p) => problems.push(p),
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
    let (remainder, blocks) = ws2(many0(ws2(parse_define_block))).parse(input.into())?;

    let domains = blocks
        .into_iter()
        .filter_map(|block| match block {
            DefineBlock::Domain(d) => Some(d),
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
    let (remainder, blocks) = ws2(many0(ws2(parse_define_block))).parse(input.into())?;

    let problems = blocks
        .into_iter()
        .filter_map(|block| match block {
            DefineBlock::Problem(p) => Some(p),
            _ => None,
        })
        .collect();

    Ok((remainder, problems))
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
    use crate::{Name, Parser, PddlFile};

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
}
