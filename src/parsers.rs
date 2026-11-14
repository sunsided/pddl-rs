//! Provides the [`Parser`] trait and the [`ParseError`] it reports.
//!
//! Parsing runs in two steps: `sexpr` reads the text into a tree of atoms and
//! parenthesized lists, each with its position, and the grammar rules turn that
//! tree into the typed AST. The rules are grouped by grammar area, mirroring
//! `crate::types`: `structure` (domains, problems and their sections), `actions`,
//! `goals`, `durative`, `expressions` and `terms`. `utilities` holds the helpers
//! they share.

mod actions;
mod durative;
mod expressions;
mod goals;
mod structure;
mod terms;

// structure: domain and problem definitions and their sections
pub(crate) use structure::atomic_formula_skeleton::parse_atomic_formula_skeleton;
pub(crate) use structure::atomic_function_skeleton::parse_atomic_function_skeleton;
pub(crate) use structure::constants_def::parse_constants_def;
pub(crate) use structure::domain::parse_domain;
pub(crate) use structure::domain_constraints_def::parse_domain_constraints_def;
pub(crate) use structure::functions_def::parse_functions_def;
pub(crate) use structure::goal_def::parse_problem_goal_def;
pub(crate) use structure::init_def::parse_problem_init_def;
pub(crate) use structure::init_el::parse_init_el;
pub(crate) use structure::length_spec::parse_problem_length_spec;
pub(crate) use structure::metric_spec::parse_problem_metric_spec;
pub(crate) use structure::objects_def::parse_problem_objects_declaration;
pub(crate) use structure::predicates_def::parse_predicates_def;
pub(crate) use structure::problem::parse_problem;
pub(crate) use structure::problem_constraints_def::parse_problem_constraints_def;
pub(crate) use structure::requirements::parse_require_def;
pub(crate) use structure::structure_def::parse_structure_def;
pub(crate) use structure::timeless_def::parse_timeless_def;
pub(crate) use structure::types_def::parse_types_def;

// actions: actions, derived predicates and their effects
pub(crate) use actions::action_def::parse_action_def;
pub(crate) use actions::action_symbol::parse_action_symbol;
pub(crate) use actions::c_effect::parse_c_effect;
pub(crate) use actions::cond_effect::parse_effect_condition;
pub(crate) use actions::derived_predicate::parse_derived_predicate;
pub(crate) use actions::effects::parse_effect;
pub(crate) use actions::p_effect::parse_p_effect;

// goals: goal descriptions, literals, atomic formulas and preferences
pub(crate) use goals::atomic_formula::atomic_formula;
pub(crate) use goals::con_gd::parse_con_gd;
pub(crate) use goals::gd::parse_gd;
pub(crate) use goals::literal::literal;
pub(crate) use goals::pre_gd::parse_pre_gd;
pub(crate) use goals::pref_con_gd::parse_pref_con_gd;
pub(crate) use goals::pref_gd::parse_pref_gd;
pub(crate) use goals::pref_name::parse_pref_name;

// durative: durative actions: their conditions, effects, durations and timing
pub(crate) use durative::assign_op_t::parse_assign_op_t;
pub(crate) use durative::d_op::parse_d_op;
pub(crate) use durative::d_value::parse_d_value;
pub(crate) use durative::da_def::parse_da_def;
pub(crate) use durative::da_effect::parse_da_effect;
pub(crate) use durative::da_gd::parse_da_gd;
pub(crate) use durative::da_symbol::parse_da_symbol;
pub(crate) use durative::duration_constraint::parse_duration_constraint;
pub(crate) use durative::f_assign_da::parse_f_assign_da;
pub(crate) use durative::f_exp_da::parse_f_exp_da;
pub(crate) use durative::f_exp_t::parse_f_exp_t;
pub(crate) use durative::interval::parse_interval;
pub(crate) use durative::pref_timed_gd::parse_pref_timed_gd;
pub(crate) use durative::simple_duration_constraint::parse_simple_duration_constraint;
pub(crate) use durative::time_specifier::parse_time_specifier;
pub(crate) use durative::timed_effect::parse_timed_effect;
pub(crate) use durative::timed_gd::parse_timed_gd;

// expressions: numeric fluent expressions, comparisons and operators
pub(crate) use expressions::assign_op::parse_assign_op;
pub(crate) use expressions::binary_comp::parse_binary_comp;
pub(crate) use expressions::binary_op::parse_binary_op;
pub(crate) use expressions::f_comp::parse_f_comp;
pub(crate) use expressions::f_exp::{op_expr, parse_f_exp};
pub(crate) use expressions::f_head::parse_f_head;
pub(crate) use expressions::metric_f_exp::parse_metric_f_exp;
pub(crate) use expressions::optimization::parse_optimization;

// terms: names, variables, numbers, terms and types
pub(crate) use terms::basic_function_term::parse_basic_function_term;
pub(crate) use terms::function_symbol::parse_function_symbol;
pub(crate) use terms::function_term::parse_function_term;
pub(crate) use terms::function_typed_list::function_typed_list;
pub(crate) use terms::name::{is_name, parse_name};
pub(crate) use terms::number::parse_number;
pub(crate) use terms::predicate::parse_predicate;
pub(crate) use terms::primitive_type::parse_primitive_type;
pub(crate) use terms::r#type::parse_type;
pub(crate) use terms::term::parse_term;
pub(crate) use terms::typed_list::{typed_list, typed_variables};
pub(crate) use terms::variable::parse_variable;

mod error;
mod sexpr;
#[cfg(test)]
mod test_helpers;
mod utilities;

pub use error::ParseError;

use error::{Error, R};
use sexpr::Node;

#[cfg(test)]
pub(crate) use test_helpers::UnwrapValue;

// Grammar rules, shared between the rule modules.
pub(crate) use utilities::{
    alt, empty_or, keyword_arg, opt_keyword_arg, parens, prefix, set_once, word, Args,
};

/// Provides the [`Parser::from_str`] method.
pub trait Parser {
    /// The parsed value; `Self` except for
    /// [`DurationConstraint`](crate::DurationConstraint), which may be empty.
    type Item;

    /// Parses the entire `input` into the specified [`Item`](Self::Item) type.
    ///
    /// Only whitespace and `;` comments may follow the parsed item; any other
    /// trailing input is an error. To read several `(define ...)` blocks from
    /// one input, parse it as a [`PddlFile`](crate::PddlFile) instead.
    ///
    /// On failure, returns a [`ParseError`] with the line, column and an
    /// excerpt of the input at which parsing stopped.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Name, Parser};
    /// assert!(Name::from_str("abc ; comment\n").is_ok());
    ///
    /// let error = Name::from_str("abc def").unwrap_err();
    /// assert_eq!(error.found(), Some("def"));
    /// ```
    fn from_str(input: &str) -> Result<Self::Item, ParseError>;
}

/// Parses `input` with `rule`, which reads a prefix of the top-level nodes and
/// returns how many it read. Any node or unbalanced parenthesis after them is an error.
pub(crate) fn parse_nodes<T>(
    input: &str,
    rule: impl for<'n, 'a> FnOnce(&'n [Node<'a>], usize) -> R<(T, usize)>,
) -> Result<T, ParseError> {
    let run = || {
        let forest = sexpr::read(input)?;
        let (value, read) = rule(&forest.nodes, input.len())?;
        if let Some(extra) = forest.nodes.get(read) {
            return Err(extra.error());
        }
        match forest.unbalanced {
            Some(error) => Err(error),
            None => Ok(value),
        }
    };
    run().map_err(|e| ParseError::new(input, e))
}

/// Parses `input` as exactly one atom or list with `rule`.
pub(crate) fn parse_one<T>(
    input: &str,
    rule: impl for<'n, 'a> FnOnce(&'n Node<'a>) -> R<T>,
) -> Result<T, ParseError> {
    parse_nodes(input, |nodes, len| match nodes.first() {
        Some(node) => rule(node).map(|value| (value, 1)),
        None => Err(Error::at(len, 0)),
    })
}

/// Implements [`Parser`] for each `type => rule` pair through [`parse_one`].
macro_rules! impl_parser {
    ($($ty:ty => $rule:path),+ $(,)?) => {$(
        impl $crate::parsers::Parser for $ty {
            type Item = $ty;

            fn from_str(input: &str) -> Result<Self::Item, $crate::parsers::ParseError> {
                $crate::parsers::parse_one(input, $rule)
            }
        }
    )+};
}
pub(crate) use impl_parser;

#[cfg(test)]
mod tests {
    use crate::{BasicFunctionTerm, Domain, Parser};

    const DOMAIN: &str = "(define (domain d) (:requirements :strips) (:predicates (p)))";

    /// Every keyword parser must map each keyword, in any case, to the same
    /// variant as the type's `TryFrom<&str>`, and consume it completely.
    #[test]
    fn keyword_parsers_agree_with_try_from() {
        macro_rules! check {
            ($ty:ty, $names:path: $($name:ident),+ $(,)?) => {{
                use $names as names;
                $(
                    let expected = <$ty>::try_from(names::$name).unwrap();
                    for input in [names::$name.to_string(), names::$name.to_ascii_uppercase()] {
                        let parsed = <$ty as Parser>::from_str(&input);
                        assert_eq!(parsed, Ok(expected.clone()), "{input}");
                    }
                )+
            }};
        }
        use crate::types::*;
        check!(AssignOp, crate::types::expressions::assign_op::names:
            CHANGE, ASSIGN, SCALE_UP, SCALE_DOWN, INCREASE, DECREASE);
        check!(TimedAssignOperator, crate::types::durative::assign_op_t::names:
            INCREASE, DECREASE);
        check!(BinaryOp, crate::types::expressions::binary_op::names:
            MULTIPLICATION, ADDITION, SUBTRACTION, DIVISION);
        check!(BinaryComparison, crate::types::expressions::binary_comp::names:
            GREATER_THAN, LESS_THAN, EQUAL, GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL);
        check!(DurationOperator, crate::types::durative::d_op::names:
            EQUAL, GREATER_OR_EQUAL, LESS_THAN_OR_EQUAL);
        check!(Interval, crate::types::durative::interval::names: ALL);
        check!(MultiOp, crate::types::expressions::multi_op::names:
            MULTIPLICATION, ADDITION);
        check!(Optimization, crate::types::expressions::optimization::names:
            MINIMIZE, MAXIMIZE);
        check!(TimeSpecifier, crate::types::durative::time_specifier::names:
            START, END);
        check!(Requirement, crate::types::structure::requirement::names:
            STRIPS, TYPING, NEGATIVE_PRECONDITIONS, DISJUNCTIVE_PRECONDITIONS, EQUALITY,
            EXISTENTIAL_PRECONDITIONS, UNIVERSAL_PRECONDITIONS, QUANTIFIED_PRECONDITIONS,
            CONDITIONAL_EFFECTS, FLUENTS, NUMERIC_FLUENTS, OBJECT_FLUENTS, ADL,
            DURATIVE_ACTIONS, DURATION_INEQUALITIES, CONTINUOUS_EFFECTS, DERIVED_PREDICATES,
            TIMED_INITIAL_LITERALS, PREFERENCES, CONSTRAINTS, ACTION_COSTS);
    }

    #[test]
    fn test_from_str() {
        let value = BasicFunctionTerm::from_str("abcde").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));
    }

    #[test]
    fn from_str_allows_trailing_whitespace_and_comments() {
        let input = format!("{DOMAIN}\n  ; trailing comment\n; another\n\n");
        assert!(Domain::from_str(&input).is_ok());
    }

    #[test]
    fn from_str_rejects_trailing_input() {
        let input = format!("{DOMAIN}\n)))garbage");
        let error = Domain::from_str(&input).unwrap_err();
        assert_eq!((error.line(), error.column()), (2, 1));
        assert_eq!(error.found(), Some(")))garbage"));
    }

    #[test]
    fn from_str_rejects_empty_input() {
        let error = Domain::from_str("  ; nothing\n").unwrap_err();
        assert_eq!(error.offset(), 12);
        assert_eq!(error.found(), None);
    }

    #[test]
    fn from_str_rejects_a_stray_closing_parenthesis() {
        let error = BasicFunctionTerm::from_str("abc)").unwrap_err();
        assert_eq!(error.span(), 3..4);
    }

    #[test]
    fn from_str_rejects_a_second_define_block() {
        let input = format!("{DOMAIN}\n(define (problem q) (:domain d))");
        let error = Domain::from_str(&input).unwrap_err();
        assert_eq!(Some(error.offset()), input.find("(define (problem"));
        assert!(error.found().unwrap().starts_with("(define (problem q)"));
    }
}
