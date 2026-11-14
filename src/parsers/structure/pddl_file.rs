//! Provides parsers for files with several domain and problem definitions.

use crate::parsers::error::R;
use crate::parsers::sexpr::{self, Node};
use crate::parsers::structure::domain::parse_domain_recovering;
use crate::parsers::structure::problem::parse_problem_recovering;
use crate::parsers::{alt, parse_domain, parse_nodes, parse_problem, ParseError, Parser};
use crate::types::{Definition, PddlFile};

/// Parses a single `(define ...)` block, determining whether it's a domain or problem.
pub(crate) fn parse_definition(node: &Node) -> R<Definition> {
    alt(
        node,
        &[&|n| parse_domain(n).map(Definition::Domain), &|n| {
            parse_problem(n).map(Definition::Problem)
        }],
    )
}

/// Parses every leading `(define ...)` node of `nodes` and returns how many it read.
pub(crate) fn parse_pddl_file(nodes: &[Node]) -> R<(PddlFile, usize)> {
    let mut domains = Vec::new();
    let mut problems = Vec::new();
    let mut read = 0;
    for node in nodes {
        if !node
            .head()
            .is_some_and(|h| h.eq_ignore_ascii_case("define"))
        {
            break;
        }
        match parse_definition(node)? {
            Definition::Domain(d) => domains.push(d),
            Definition::Problem(p) => problems.push(p),
        }
        read += 1;
    }
    let file = PddlFile::with_domains_and_problems(domains, problems);
    Ok((file, read))
}

crate::parsers::impl_parser!(Definition => parse_definition);

/// Parses `input` like [`PddlFile::from_str`](crate::Parser::from_str), but
/// returns every error it finds instead of stopping at the first one.
///
/// After an error, checking resumes at the next section of the same domain or
/// problem, or at the next top-level form. So it reports at most one error per
/// section, action or top-level form, in input order. The result is empty
/// exactly when [`PddlFile::from_str`](crate::Parser::from_str) succeeds.
///
/// ## Example
/// ```
/// let input = "(define (domain d)\n  (:predicats (p))\n  (:action a :parameters (?x -)))";
/// let errors = pddl::parsers::check(input);
/// assert_eq!(errors.len(), 2);
/// assert_eq!(errors[0].help(), Some("did you mean `:predicates`?"));
/// assert_eq!(errors[1].line(), 3);
/// ```
#[must_use]
pub fn check(input: &str) -> Vec<ParseError> {
    let forest = match sexpr::read(input) {
        Ok(forest) => forest,
        Err(error) => return vec![ParseError::new(input, &error)],
    };
    let mut errors = Vec::new();
    for node in &forest.nodes {
        if !node
            .head()
            .is_some_and(|h| h.eq_ignore_ascii_case("define"))
        {
            errors.push(node.error());
            continue;
        }
        let kind = node.children().and_then(|c| c.get(1)).and_then(Node::head);
        let result = match kind.map(str::to_ascii_lowercase).as_deref() {
            Some("domain") => parse_domain_recovering(node, Some(&mut errors)).map(drop),
            Some("problem") => parse_problem_recovering(node, Some(&mut errors)).map(drop),
            _ => parse_definition(node).map(drop),
        };
        errors.extend(result.err());
    }
    errors.extend(forest.unbalanced);
    errors.iter().map(|e| ParseError::new(input, e)).collect()
}

impl Parser for PddlFile {
    type Item = PddlFile;

    /// Parses zero or more `(define ...)` blocks, in any order.
    fn from_str(input: &str) -> Result<Self::Item, ParseError> {
        parse_nodes(input, |nodes, _| parse_pddl_file(nodes))
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use crate::*;

    const DOMAIN: &str = "(define (domain d) (:predicates (p)))";
    const PROBLEM: &str = "(define (problem q) (:domain d) (:init) (:goal (p)))";

    #[test]
    fn define_is_matched_case_insensitively() {
        let file = PddlFile::from_str(&format!("{}\n{}", DOMAIN.to_uppercase(), PROBLEM)).unwrap();
        check!((file.domain_count(), file.problem_count()) == (1, 1));
    }

    #[test]
    fn definition_kind_comes_from_the_header_not_comments() {
        for comment in ["", "; Uses (domain d)\n", "; Example (problem q)\n"] {
            let domain = Definition::from_str(&format!("{comment}{DOMAIN} ; end\n"));
            check!(let Ok(Definition::Domain(_)) = domain, "{domain:?}");
            let problem = Definition::from_str(&format!("{comment}{PROBLEM}"));
            check!(let Ok(Definition::Problem(_)) = problem, "{problem:?}");
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
            check!(Definition::from_str(&input).is_err(), "accepted {input:?}");
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
            check!(Some(error.offset()) == input.rfind(at), "{label}: {error}");
        }
    }

    #[test]
    fn check_reports_one_error_per_section_and_form() {
        let input = "(define (domain d)\n  (:predicats (p))\n  (:types t)\n  (:types u)\n  \
                     (:action a :parameters (?x -) :effect (p))\n  \
                     (:action b :parameters () :effect (p)))\n\
                     stray\n\
                     (define (problem q) (:domain d) (:init (p) 1) (:goal (p)))\n\
                     (define (problem r) (:domain d) (:init (p)))\n\
                     (define (problem s) (:domain d) (:goal (p) (q)))\n\
                     (";
        let errors = parsers::check(input);
        let found: Vec<_> = errors
            .iter()
            .map(|e| (e.line(), e.reason().unwrap_or_default()))
            .collect();
        check!(
            found
                == [
                    (2, "unknown section"),
                    (4, "duplicate section"),
                    (5, ""),
                    (7, ""),
                    (8, ""),
                    (9, "missing `:goal`"),
                    (10, "expected `)`"),
                    (11, ""),
                    (11, "expected `)`"),
                ]
        );
        // A failed `:init` is not reported again as missing.
        check!(errors.iter().all(|e| e.reason() != Some("missing `:init`")));
    }

    #[test]
    fn check_reports_sections_that_are_not_keyword_lists() {
        let input = "(define (domain d) word (()) (:predicates (p)))\n\
                     (define (problem q) (:domain d) (()) (:init) (:goal (p)))";
        let errors = parsers::check(input);
        let found: Vec<_> = errors
            .iter()
            .map(|e| {
                let excerpt = e.found().unwrap_or_default();
                (
                    &excerpt[..excerpt.find(' ').unwrap()],
                    e.reason().unwrap_or_default(),
                )
            })
            .collect();
        check!(
            found
                == [
                    ("word", ""),
                    ("(())", "unknown section"),
                    ("(())", "unknown section")
                ]
        );
        check!(errors.iter().all(|e| e.help().is_none()));
    }

    #[test]
    fn check_agrees_with_from_str() {
        let inputs = [
            String::new(),
            format!("{DOMAIN}\n{PROBLEM}"),
            format!("{DOMAIN})"),
            "(define (foo x))".into(),
            "(define (domain".into(),
            ") (define (domain d))".into(),
            "(define (domain d) (:requirement :strips))".into(),
            "(define (problem p) (:domain d) (:init (p) 1) (:goal (p)))".into(),
            "(define (problem p) (:domain d) (:init))".into(),
            "(".repeat(100),
        ];
        for input in inputs {
            let first = PddlFile::from_str(&input).err();
            let all = parsers::check(&input);
            check!(all.first() == first.as_ref(), "{input:?}");
        }
    }

    #[test]
    fn parse_single_domain_and_problem() {
        let input = r"
            (define (domain briefcase-world)
                (:requirements :strips)
                (:predicates (at ?x))
            )
            (define (problem get-paid)
                (:domain briefcase-world)
                (:init (at home))
                (:goal (at office))
            )
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 1);
        check!(pddl_file.problem_count() == 1);
        check!(pddl_file.domains[0].name() == &Name::new("briefcase-world"));
        check!(pddl_file.problems[0].name() == &Name::new("get-paid"));
    }

    #[test]
    fn parse_two_domains_and_one_problem() {
        let input = r"
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
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 2);
        check!(pddl_file.problem_count() == 1);
        check!(pddl_file.domains[0].name() == &Name::new("d1"));
        check!(pddl_file.domains[1].name() == &Name::new("d2"));
    }

    #[test]
    fn parse_one_domain_and_two_problems() {
        let input = r"
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
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 1);
        check!(pddl_file.problem_count() == 2);
    }

    #[test]
    fn parse_mixed_order_problem_before_domain() {
        let input = r"
            (define (problem p1)
                (:domain d1)
                (:init)
                (:goal (and))
            )
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 1);
        check!(pddl_file.problem_count() == 1);
        // Problem comes first in the file
        check!(pddl_file.problems[0].name() == &Name::new("p1"));
        check!(pddl_file.domains[0].name() == &Name::new("d1"));
    }

    #[test]
    fn parse_file_with_leading_comments() {
        let input = r"
            ; This is a comment
            ; Another comment
            (define (domain test)
                (:requirements :strips)
                (:predicates (p))
            )
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 1);
        check!(pddl_file.domains[0].name() == &Name::new("test"));
    }

    #[test]
    fn parse_empty_file() {
        let input = "";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.is_empty());
        check!(pddl_file.domain_count() == 0);
        check!(pddl_file.problem_count() == 0);
    }

    #[test]
    fn parse_file_with_only_whitespace_and_comments() {
        let input = r"; Just a comment

; Another comment
";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.is_empty());
    }

    #[test]
    fn parse_using_parser_trait() {
        let input = r"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (problem p1)
                (:domain d1)
                (:init)
                (:goal (and))
            )
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 1);
        check!(pddl_file.problem_count() == 1);
    }

    #[test]
    fn parse_domains_only() {
        let input = r"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (domain d2)
                (:requirements :strips)
                (:predicates (q))
            )
        ";

        let domains = PddlFile::from_str(input).unwrap().domains;
        check!(domains.len() == 2);
        check!(domains[0].name() == &Name::new("d1"));
        check!(domains[1].name() == &Name::new("d2"));
    }

    #[test]
    fn parse_problems_only() {
        let input = r"
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
        ";

        let problems = PddlFile::from_str(input).unwrap().problems;
        check!(problems.len() == 2);
    }

    #[test]
    fn parse_rejects_malformed_define_block_after_valid_ones() {
        let input = r"
            (define (domain d1)
                (:requirements :strips)
                (:predicates (p))
            )
            (define (domain d2)
                (:requirements
            )
        ";

        let result = PddlFile::from_str(input);
        check!(
            result.is_err(),
            "Should reject file with malformed define block after valid ones"
        );
    }

    #[test]
    fn doc_example_0() {
        let input = r"
            (define (domain briefcase-world)
                (:requirements :strips)
                (:predicates (at ?x))
            )
            (define (problem get-paid)
                (:domain briefcase-world)
                (:init (at home))
                (:goal (at office))
            )
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 1);
        check!(pddl_file.problem_count() == 1);
    }

    #[test]
    fn doc_example_1() {
        let input = r"
            (define (domain d1) (:requirements :strips) (:predicates (p)))
            (define (domain d2) (:requirements :strips) (:predicates (q)))
        ";

        let domains = PddlFile::from_str(input).unwrap().domains;
        check!(domains.len() == 2);
    }

    #[test]
    fn doc_example_2() {
        let input = r"
            (define (problem p1) (:domain d) (:init) (:goal (and)))
            (define (problem p2) (:domain d) (:init) (:goal (and)))
        ";

        let problems = PddlFile::from_str(input).unwrap().problems;
        check!(problems.len() == 2);
    }

    #[test]
    fn doc_example_3() {
        let input = "; Uses (domain d)\n(define (problem q) (:domain d) (:init) (:goal (p)))";
        check!(let Ok(Definition::Problem(_)) = Definition::from_str(input));

        // A second definition is trailing input, not part of this one.
        let two = "(define (domain d) (:predicates (p)))\n(define (domain e) (:predicates (q)))";
        check!(Definition::from_str(two).is_err());
    }

    #[test]
    fn doc_example_4() {
        let input = r"
            (define (domain briefcase-world)
                (:requirements :strips)
                (:predicates (at ?x))
            )
            (define (problem get-paid)
                (:domain briefcase-world)
                (:init (at home))
                (:goal (at office))
            )
        ";

        let pddl_file = PddlFile::from_str(input).unwrap();
        check!(pddl_file.domain_count() == 1);
        check!(pddl_file.problem_count() == 1);
    }
}
