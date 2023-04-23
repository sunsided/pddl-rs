//! Provides parsers for problem definitions.

use crate::parsers::{parse_name, prefix_expr, ws};
use crate::parsers::{
    parse_problem_constraints_def, parse_problem_goal_def, parse_problem_init_def,
    parse_problem_length_spec, parse_problem_metric_spec, parse_problem_objects_declaration,
    parse_require_def,
};
use crate::types::Requirements;
use crate::types::{Objects, PrefConGD, Problem};
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a problem definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_action_def, parse_problem};
/// # use pddl::types::{Name, PreGD};
///
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
/// assert_eq!(remainder, "");
/// assert_eq!(problem.name(), &Name::new("get-paid"));
/// assert_eq!(problem.domain(), &Name::new("briefcase-world"));
/// assert!(problem.requirements().is_empty());
/// assert_eq!(problem.init().len(), 9);
/// assert!(matches! { problem.goal(), PreGD::And(_) });
/// ```
pub fn parse_problem(input: &str) -> IResult<&str, Problem> {
    map(
        ws(prefix_expr(
            "define",
            tuple((
                prefix_expr("problem", parse_name),
                preceded(multispace1, prefix_expr(":domain", parse_name)),
                opt(preceded(multispace1, parse_require_def)),
                opt(preceded(multispace1, parse_problem_objects_declaration)),
                preceded(multispace1, parse_problem_init_def),
                preceded(multispace1, parse_problem_goal_def),
                // :constraints
                opt(preceded(multispace1, parse_problem_constraints_def)),
                // :numeric-fluents
                opt(preceded(multispace1, parse_problem_metric_spec)),
                // Deprecated since PDDL 2.1
                opt(preceded(multispace1, parse_problem_length_spec)),
            )),
        )),
        |(name, domain, reqs, objects, init, goal, constraints, metric, length)| {
            Problem::new(
                name,
                domain,
                reqs.unwrap_or(Requirements::new([])), // TODO: Do we need to imply STRIPS if empty?
                objects.unwrap_or(Objects::default()),
                init,
                goal,
                constraints.unwrap_or(PrefConGD::default()),
                metric,
                length,
            )
        },
    )(input)
}