//! Provides parsers for problem definitions.

use nom::branch::alt;
use nom::combinator::map;
use nom::Parser;

use crate::parsers::utilities::skip_whitespace_and_comments;
use crate::parsers::{
    failure_at, fold_sections, parse_name, prefix_expr, set_once, ws2, NomError, ParseResult, Span,
};
use crate::parsers::{
    parse_problem_constraints_def, parse_problem_goal_def, parse_problem_init_def,
    parse_problem_length_spec, parse_problem_metric_spec, parse_problem_objects_declaration,
    parse_require_def,
};
use crate::types::{
    InitElements, LengthSpec, MetricSpec, Name, Objects, Problem, ProblemConstraintsDef,
    ProblemGoalDefinition, Requirements,
};

/// Parses a problem definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_action_def, parse_problem, preamble::*};
/// # use pddl::{Name, PreconditionGoalDefinition};
/// let input = r#"(define (problem get-paid)
///         (:domain briefcase-world)
///         (:init (place home) (place office)
///                (object p) (object d) (object b)
///                (at B home) (at P home) (at D home) (in P))
///         (:goal (and (at B office) (at D office) (at P home)))
///     )"#;
///
/// let (remainder, problem) = parse_problem(input).unwrap();
///
/// assert!(remainder.is_empty());
/// assert_eq!(problem.name(), &Name::new("get-paid"));
/// assert_eq!(problem.domain(), &Name::new("briefcase-world"));
/// assert!(problem.requirements().is_empty());
/// assert_eq!(problem.init().len(), 9);
/// assert_eq!(problem.goals().len(), 3);
/// ```
pub fn parse_problem<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Problem> {
    ws2(prefix_expr("define", parse_problem_body)).parse(input.into())
}

/// A top-level part of a problem definition.
enum Section {
    Domain(Name),
    Requirements(Requirements),
    Objects(Objects),
    Init(InitElements),
    Goal(ProblemGoalDefinition),
    Constraints(ProblemConstraintsDef),
    Metric(MetricSpec),
    Length(LengthSpec),
}

/// Parses `(problem <name>)` followed by the sections in any order.
///
/// Each section may appear at most once; `:domain`, `:init` and `:goal` are required.
fn parse_problem_body(input: Span) -> ParseResult<Problem> {
    let (input, name) = prefix_expr("problem", parse_name).parse(input)?;

    let mut domain = None;
    let mut requirements = None;
    let mut objects = None;
    let mut init = None;
    let mut goal = None;
    let mut constraints = None;
    let mut metric = None;
    let mut length = None;

    let section = alt((
        map(prefix_expr(":domain", parse_name), Section::Domain),
        map(parse_require_def, Section::Requirements),
        map(parse_problem_objects_declaration, Section::Objects),
        map(parse_problem_init_def, Section::Init),
        map(parse_problem_goal_def, Section::Goal),
        map(parse_problem_constraints_def, Section::Constraints),
        // :numeric-fluents
        map(parse_problem_metric_spec, Section::Metric),
        // Deprecated since PDDL 2.1
        map(parse_problem_length_spec, Section::Length),
    ));
    let (input, ()) = fold_sections(input, section, |section, at| match section {
        Section::Domain(v) => set_once(&mut domain, v, at),
        Section::Requirements(v) => set_once(&mut requirements, v, at),
        Section::Objects(v) => set_once(&mut objects, v, at),
        Section::Init(v) => set_once(&mut init, v, at),
        Section::Goal(v) => set_once(&mut goal, v, at),
        Section::Constraints(v) => set_once(&mut constraints, v, at),
        Section::Metric(v) => set_once(&mut metric, v, at),
        Section::Length(v) => set_once(&mut length, v, at),
    })?;

    let missing = match (&domain, &init, &goal) {
        (None, _, _) => Some("missing `:domain`"),
        (_, None, _) => Some("missing `:init`"),
        (_, _, None) => Some("missing `:goal`"),
        _ => None,
    };
    if let Some(missing) = missing {
        let (at, _) = skip_whitespace_and_comments(input)?;
        // Only call a section missing where the sections really end; anything
        // else (e.g. `:goal (and)` without parentheses) is just unexpected.
        let error = if at.fragment().is_empty() || at.fragment().starts_with(')') {
            failure_at(at, missing)
        } else {
            nom::Err::Failure(NomError::new(at, nom::error::ErrorKind::Tag))
        };
        return Err(error);
    }
    let (Some(domain), Some(init), Some(goal)) = (domain, init, goal) else {
        unreachable!("all required sections are present");
    };

    let problem = Problem::new(
        name,
        domain,
        requirements.unwrap_or_default(), // TODO: Do we need to imply STRIPS if empty?
        objects.unwrap_or_default(),
        init,
        goal,
        constraints.unwrap_or_default(),
        metric,
        length,
    );
    Ok((input, problem))
}

impl crate::parsers::Parser for Problem {
    type Item = Problem;

    /// Parses a problem definitions.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Name, Parser, Problem};
    /// let input = r#"(define (problem get-paid)
    ///         (:domain briefcase-world)
    ///         (:init (place home) (place office)
    ///                (object p) (object d) (object b)
    ///                (at B home) (at P home) (at D home) (in P))
    ///         (:goal (and (at B office) (at D office) (at P home)))
    ///     )"#;
    ///
    /// let (_, problem) = Problem::parse(input).unwrap();
    ///
    /// assert_eq!(problem.name(), &Name::new("get-paid"));
    /// assert_eq!(problem.domain(), &Name::new("briefcase-world"));
    /// assert!(problem.requirements().is_empty());
    /// assert_eq!(problem.init().len(), 9);
    /// assert_eq!(problem.goals().len(), 3);
    /// ```
    ///
    /// ## See also
    /// See [`parse_problem`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{Name, Problem};

    #[test]
    fn sections_in_any_order() {
        let ordered = Problem::from_str(
            "(define (problem p) (:domain d) (:requirements :strips) (:objects a) \
             (:init (at a)) (:goal (at a)) (:metric minimize (total-time)))",
        )
        .unwrap();
        let shuffled = Problem::from_str(
            "(define (problem p) (:metric minimize (total-time)) (:goal (at a)) (:objects a) \
             (:init (at a)) (:requirements :strips) (:domain d))",
        )
        .unwrap();
        assert_eq!(ordered, shuffled);
    }

    #[test]
    fn missing_required_section_is_reported_where_the_sections_end() {
        let input = "(define (problem p) (:domain d) (:init (at a))\n)";
        let error = Problem::from_str(input).unwrap_err();
        assert_eq!((error.line(), error.column()), (2, 1));
        assert_eq!(error.found(), Some(")"));
    }

    #[test]
    fn duplicate_section_is_reported_at_the_second_occurrence() {
        let input = "(define (problem p) (:domain d) (:init) (:goal (and)) (:init))";
        let error = Problem::from_str(input).unwrap_err();
        assert_eq!(Some(error.offset()), input.rfind("(:init)"));
    }

    #[test]
    fn test_parse() {
        let input = r#"(define (problem get-paid)
                 (:domain briefcase-world)
                 (:init (place home) (place office)
                        (object p) (object d) (object b)
                        (at B home) (at P home) (at D home) (in P))
                 (:goal (and (at B office) (at D office) (at P home)))
             )"#;

        let (remainder, problem) = Problem::parse(input).unwrap();

        assert!(remainder.is_empty());
        assert_eq!(problem.name(), &Name::new("get-paid"));
        assert_eq!(problem.domain(), &Name::new("briefcase-world"));
        assert!(problem.requirements().is_empty());
        assert_eq!(problem.init().len(), 9);
        assert_eq!(problem.goals().len(), 3);
    }
}
