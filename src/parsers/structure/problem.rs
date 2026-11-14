//! Provides parsers for problem definitions (`<problem>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    parse_name, parse_problem_constraints_def, parse_problem_goal_def, parse_problem_init_def,
    parse_problem_length_spec, parse_problem_metric_spec, parse_problem_objects_declaration,
    parse_require_def, prefix, set_once,
};
use crate::types::Problem;

/// Parses a problem definition (`<problem>`), i.e. `(define (problem <name>) <section>*)`.
///
/// The sections may appear in any order, each at most once; `:domain`, `:init`
/// and `:goal` are required.
pub(crate) fn parse_problem(node: &Node) -> R<Problem> {
    prefix(node, "define", |args| {
        let name = prefix(args.next()?, "problem", |args| parse_name(args.next()?))?;
        let mut domain = None;
        let mut requirements = None;
        let mut objects = None;
        let mut init = None;
        let mut goal = None;
        let mut constraints = None;
        let mut metric = None;
        let mut length = None;
        for section in args.rest() {
            if section.atom().is_some() {
                return Err(section.error());
            }
            let head = section.head().unwrap_or_default().to_ascii_lowercase();
            match head.as_str() {
                ":domain" => {
                    let value = prefix(section, ":domain", |args| parse_name(args.next()?))?;
                    set_once(&mut domain, value, section)?;
                }
                ":requirements" => {
                    set_once(&mut requirements, parse_require_def(section)?, section)?;
                }
                ":objects" => {
                    let value = parse_problem_objects_declaration(section)?;
                    set_once(&mut objects, value, section)?;
                }
                ":init" => set_once(&mut init, parse_problem_init_def(section)?, section)?,
                ":goal" => set_once(&mut goal, parse_problem_goal_def(section)?, section)?,
                ":constraints" => {
                    let value = parse_problem_constraints_def(section)?;
                    set_once(&mut constraints, value, section)?;
                }
                // :numeric-fluents
                ":metric" => set_once(&mut metric, parse_problem_metric_spec(section)?, section)?,
                // Deprecated since PDDL 2.1
                ":length" => set_once(&mut length, parse_problem_length_spec(section)?, section)?,
                _ => return Err(section.error().reason("unknown section").fatal()),
            }
        }
        // Exhaustive per section, so a new required section can't be reported as another one.
        let missing = |reason| Err(args.error_at_close().reason(reason).fatal());
        let (domain, init, goal) = match (domain, init, goal) {
            (Some(domain), Some(init), Some(goal)) => (domain, init, goal),
            (None, _, _) => return missing("missing `:domain`"),
            (_, None, _) => return missing("missing `:init`"),
            (_, _, None) => return missing("missing `:goal`"),
        };
        Ok(Problem::new(
            name,
            domain,
            requirements.unwrap_or_default(),
            objects.unwrap_or_default(),
            init,
            goal,
            constraints.unwrap_or_default(),
            metric,
            length,
        ))
    })
}

crate::parsers::impl_parser!(Problem => parse_problem);

#[cfg(test)]
mod tests {
    use assert2::check;

    use crate::*;

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
        check!(ordered == shuffled);
    }

    #[test]
    fn missing_required_section_is_reported_where_the_sections_end() {
        let input = "(define (problem p) (:domain d) (:init (at a))\n)";
        let error = Problem::from_str(input).unwrap_err();
        check!((error.line(), error.column()) == (2, 1));
        check!(error.found() == Some(")"));
    }

    #[test]
    fn duplicate_section_is_reported_at_the_second_occurrence() {
        let input = "(define (problem p) (:domain d) (:init) (:goal (and)) (:init))";
        let error = Problem::from_str(input).unwrap_err();
        check!(Some(error.offset()) == input.rfind("(:init)"));
    }

    #[test]
    fn test_parse() {
        let input = r"(define (problem get-paid)
                 (:domain briefcase-world)
                 (:init (place home) (place office)
                        (object p) (object d) (object b)
                        (at B home) (at P home) (at D home) (in P))
                 (:goal (and (at B office) (at D office) (at P home)))
             )";

        let problem = Problem::from_str(input).unwrap();
        check!(problem.name() == &Name::new("get-paid"));
        check!(problem.domain() == &Name::new("briefcase-world"));
        check!(problem.requirements().is_empty());
        check!(problem.init().len() == 9);
        check!(problem.goals().len() == 3);
    }

    #[test]
    fn doc_example_0() {
        let input = r"(define (problem get-paid)
                (:domain briefcase-world)
                (:init (place home) (place office)
                       (object p) (object d) (object b)
                       (at B home) (at P home) (at D home) (in P))
                (:goal (and (at B office) (at D office) (at P home)))
            )";

        let problem = Problem::from_str(input).unwrap();
        check!(problem.name() == &Name::new("get-paid"));
        check!(problem.domain() == &Name::new("briefcase-world"));
        check!(problem.requirements().is_empty());
        check!(problem.init().len() == 9);
        check!(problem.goals().len() == 3);
    }

    #[test]
    fn doc_example_1() {
        let input = r"(define (problem get-paid)
                (:domain briefcase-world)
                (:init (place home) (place office)
                       (object p) (object d) (object b)
                       (at B home) (at P home) (at D home) (in P))
                (:goal (and (at B office) (at D office) (at P home)))
            )";

        let problem = Problem::from_str(input).unwrap();

        check!(problem.name() == &Name::new("get-paid"));
        check!(problem.domain() == &Name::new("briefcase-world"));
        check!(problem.requirements().is_empty());
        check!(problem.init().len() == 9);
        check!(problem.goals().len() == 3);
    }
}
