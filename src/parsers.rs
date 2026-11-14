//! Provides the [`Parser`] trait and the [`ParseError`] it reports.
//!
//! Parsing runs in two steps: `sexpr` reads the text into a tree of atoms and
//! parenthesized lists, each with its position, and the grammar rules in the
//! other modules turn that tree into the typed AST.

mod action_def;
mod action_symbol;
mod assign_op;
mod assign_op_t;
mod atomic_formula;
mod atomic_formula_skeleton;
mod atomic_function_skeleton;
mod basic_function_term;
mod binary_comp;
mod binary_op;
mod c_effect;
mod con_gd;
mod cond_effect;
mod constants_def;
mod d_op;
mod d_value;
mod da_def;
mod da_effect;
mod da_gd;
mod da_symbol;
mod derived_predicate;
mod domain;
mod domain_constraints_def;
mod duration_constraint;
mod effects;
mod error;
mod f_assign_da;
mod f_comp;
mod f_exp;
mod f_exp_da;
mod f_exp_t;
mod f_head;
mod function_symbol;
mod function_term;
mod function_type;
mod function_typed_list;
mod functions_def;
mod gd;
mod goal_def;
mod init_def;
mod init_el;
mod interval;
mod length_spec;
mod literal;
mod metric_f_exp;
mod metric_spec;
mod multi_op;
mod name;
mod number;
mod objects_def;
mod optimization;
mod p_effect;
mod pddl_file;
mod pre_gd;
mod predicate;
mod predicates_def;
mod pref_con_gd;
mod pref_gd;
mod pref_name;
mod pref_timed_gd;
mod primitive_type;
mod problem;
mod problem_constraints_def;
mod requirements;
mod sexpr;
mod simple_duration_constraint;
mod structure_def;
mod term;
#[cfg(test)]
mod test_helpers;
mod time_specifier;
mod timed_effect;
mod timed_gd;
mod timeless_def;
mod r#type;
mod typed_list;
mod types_def;
mod utilities;
mod variable;

pub use error::ParseError;

use error::{Error, R};
use sexpr::Node;

#[cfg(test)]
pub(crate) use test_helpers::UnwrapValue;

// Grammar rules, shared between the rule modules.
pub(crate) use action_def::parse_action_def;
pub(crate) use action_symbol::parse_action_symbol;
pub(crate) use assign_op::parse_assign_op;
pub(crate) use assign_op_t::parse_assign_op_t;
pub(crate) use atomic_formula::atomic_formula;
pub(crate) use atomic_formula_skeleton::parse_atomic_formula_skeleton;
pub(crate) use atomic_function_skeleton::parse_atomic_function_skeleton;
pub(crate) use basic_function_term::parse_basic_function_term;
pub(crate) use binary_comp::parse_binary_comp;
pub(crate) use binary_op::parse_binary_op;
pub(crate) use c_effect::parse_c_effect;
pub(crate) use con_gd::parse_con_gd;
pub(crate) use cond_effect::parse_effect_condition;
pub(crate) use constants_def::parse_constants_def;
pub(crate) use d_op::parse_d_op;
pub(crate) use d_value::parse_d_value;
pub(crate) use da_def::parse_da_def;
pub(crate) use da_effect::parse_da_effect;
pub(crate) use da_gd::parse_da_gd;
pub(crate) use da_symbol::parse_da_symbol;
pub(crate) use derived_predicate::parse_derived_predicate;
pub(crate) use domain::parse_domain;
pub(crate) use domain_constraints_def::parse_domain_constraints_def;
pub(crate) use duration_constraint::parse_duration_constraint;
pub(crate) use effects::parse_effect;
pub(crate) use f_assign_da::parse_f_assign_da;
pub(crate) use f_comp::parse_f_comp;
pub(crate) use f_exp::{op_expr, parse_f_exp};
pub(crate) use f_exp_da::parse_f_exp_da;
pub(crate) use f_exp_t::parse_f_exp_t;
pub(crate) use f_head::parse_f_head;
pub(crate) use function_symbol::parse_function_symbol;
pub(crate) use function_term::parse_function_term;
pub(crate) use function_typed_list::function_typed_list;
pub(crate) use functions_def::parse_functions_def;
pub(crate) use gd::parse_gd;
pub(crate) use goal_def::parse_problem_goal_def;
pub(crate) use init_def::parse_problem_init_def;
pub(crate) use init_el::parse_init_el;
pub(crate) use interval::parse_interval;
pub(crate) use length_spec::parse_problem_length_spec;
pub(crate) use literal::literal;
pub(crate) use metric_f_exp::parse_metric_f_exp;
pub(crate) use metric_spec::parse_problem_metric_spec;
pub(crate) use name::{is_name, parse_name};
pub(crate) use number::parse_number;
pub(crate) use objects_def::parse_problem_objects_declaration;
pub(crate) use optimization::parse_optimization;
pub(crate) use p_effect::parse_p_effect;
pub(crate) use pre_gd::parse_pre_gd;
pub(crate) use predicate::parse_predicate;
pub(crate) use predicates_def::parse_predicates_def;
pub(crate) use pref_con_gd::parse_pref_con_gd;
pub(crate) use pref_gd::parse_pref_gd;
pub(crate) use pref_name::parse_pref_name;
pub(crate) use pref_timed_gd::parse_pref_timed_gd;
pub(crate) use primitive_type::parse_primitive_type;
pub(crate) use problem::parse_problem;
pub(crate) use problem_constraints_def::parse_problem_constraints_def;
pub(crate) use r#type::parse_type;
pub(crate) use requirements::parse_require_def;
pub(crate) use simple_duration_constraint::parse_simple_duration_constraint;
pub(crate) use structure_def::parse_structure_def;
pub(crate) use term::parse_term;
pub(crate) use time_specifier::parse_time_specifier;
pub(crate) use timed_effect::parse_timed_effect;
pub(crate) use timed_gd::parse_timed_gd;
pub(crate) use timeless_def::parse_timeless_def;
pub(crate) use typed_list::{typed_list, typed_variables};
pub(crate) use types_def::parse_types_def;
pub(crate) use utilities::{
    alt, empty_or, keyword_arg, opt_keyword_arg, parens, prefix, set_once, word, Args,
};
pub(crate) use variable::parse_variable;

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
        check!(AssignOp, crate::types::assign_op::names:
            CHANGE, ASSIGN, SCALE_UP, SCALE_DOWN, INCREASE, DECREASE);
        check!(TimedAssignOperator, crate::types::assign_op_t::names:
            INCREASE, DECREASE);
        check!(BinaryOp, crate::types::binary_op::names:
            MULTIPLICATION, ADDITION, SUBTRACTION, DIVISION);
        check!(BinaryComparison, crate::types::binary_comp::names:
            GREATER_THAN, LESS_THAN, EQUAL, GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL);
        check!(DurationOperator, crate::types::d_op::names:
            EQUAL, GREATER_OR_EQUAL, LESS_THAN_OR_EQUAL);
        check!(Interval, crate::types::interval::names: ALL);
        check!(MultiOp, crate::types::multi_op::names:
            MULTIPLICATION, ADDITION);
        check!(Optimization, crate::types::optimization::names:
            MINIMIZE, MAXIMIZE);
        check!(TimeSpecifier, crate::types::time_specifier::names:
            START, END);
        check!(Requirement, crate::types::requirement::names:
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
