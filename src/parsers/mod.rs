//! Provides parsing functions, as well as the [`Parser`] trait.

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
mod comments;
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
mod empty_or;
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
pub(crate) mod number;
mod objects_def;
mod optimization;
mod p_effect;
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
mod simple_duration_constraint;
mod structure_def;
mod term;
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

#[cfg(test)]
pub(crate) use test_helpers::Match;
pub use test_helpers::UnwrapValue;

/// Provides the [`Parser::parse`] and [`Parser::from_str`] methods.
pub trait Parser {
    type Item;

    /// Parses the `input` into the specified [`Item`](Self::Item) type.
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item>;

    /// Parses the `input` into the specified [`Item`](Self::Item) type.
    fn parse_span(input: Span) -> ParseResult<Self::Item> {
        Self::parse(input)
    }

    /// Uses the [`Parser::parse`] method to parse the input and, if successful,
    /// discards the unparsed remaining input.
    fn from_str(input: &str) -> Result<Self::Item, nom::Err<ParseError>> {
        let (_, value) = Self::parse(input)?;
        Ok(value)
    }
}

/// Input type for parsers.
pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;

/// A parsing error.
pub type ParseError<'a> = nom_greedyerror::GreedyError<Span<'a>, nom::error::ErrorKind>;

/// A result from a parser.
pub type ParseResult<'a, T, E = ParseError<'a>> = nom::IResult<Span<'a>, T, E>;

/// Re-exports commonly used types.
pub mod preamble {
    pub use crate::parsers::test_helpers::UnwrapValue;
    pub use crate::parsers::Parser;
    pub use crate::parsers::{ParseError, ParseResult, Span};
}

// Parsers.
pub use action_def::parse_action_def;
pub use action_symbol::parse_action_symbol;
pub use assign_op::parse_assign_op;
pub use assign_op_t::parse_assign_op_t;
pub use atomic_formula_skeleton::parse_atomic_formula_skeleton;
pub use atomic_function_skeleton::parse_atomic_function_skeleton;
pub use basic_function_term::parse_basic_function_term;
pub use binary_comp::parse_binary_comp;
pub use binary_op::parse_binary_op;
pub use c_effect::{parse_c_effect, parse_forall_c_effect, parse_when_c_effect};
pub use comments::ignore_eol_comment;
pub use con_gd::parse_con_gd;
pub use cond_effect::parse_cond_effect;
pub use constants_def::parse_constants_def;
pub use d_op::parse_d_op;
pub use d_value::parse_d_value;
pub use da_def::parse_da_def;
pub use da_effect::parse_da_effect;
pub use da_gd::parse_da_gd;
pub use da_symbol::parse_da_symbol;
pub use derived_predicate::parse_derived_predicate;
pub use domain::parse_domain;
pub use domain_constraints_def::parse_domain_constraints_def;
pub use duration_constraint::parse_duration_constraint;
pub use effects::parse_effect;
pub use f_assign_da::parse_f_assign_da;
pub use f_comp::parse_f_comp;
pub use f_exp::parse_f_exp;
pub use f_exp_da::parse_f_exp_da;
pub use f_exp_t::parse_f_exp_t;
pub use f_head::parse_f_head;
pub use function_symbol::parse_function_symbol;
pub use function_term::parse_function_term;
pub use function_type::parse_function_type;
pub use functions_def::parse_functions_def;
pub use gd::parse_gd;
pub use goal_def::parse_problem_goal_def;
pub use init_def::parse_problem_init_def;
pub use init_el::parse_init_el;
pub use interval::parse_interval;
pub use length_spec::parse_problem_length_spec;
pub use metric_f_exp::parse_metric_f_exp;
pub use metric_spec::parse_problem_metric_spec;
pub use multi_op::parse_multi_op;
pub use name::parse_name;
pub use number::parse_number;
pub use objects_def::parse_problem_objects_declaration;
pub use optimization::parse_optimization;
pub use p_effect::parse_p_effect;
pub use pre_gd::parse_pre_gd;
pub use predicate::parse_predicate;
pub use predicates_def::parse_predicates_def;
pub use pref_con_gd::parse_pref_con_gd;
pub use pref_gd::parse_pref_gd;
pub use pref_name::parse_pref_name;
pub use pref_timed_gd::parse_pref_timed_gd;
pub use primitive_type::parse_primitive_type;
pub use problem::parse_problem;
pub use problem_constraints_def::parse_problem_constraints_def;
pub use r#type::parse_type;
pub use requirements::{parse_require_def, parse_require_key};
pub use simple_duration_constraint::parse_simple_duration_constraint;
pub use structure_def::parse_structure_def;
pub use term::parse_term;
pub use time_specifier::parse_time_specifier;
pub use timed_effect::parse_timed_effect;
pub use timed_gd::parse_timed_gd;
pub use timeless_def::parse_timeless_def;
pub use types_def::parse_types_def;
pub use variable::parse_variable;

// Parser combinators.
pub use atomic_formula::atomic_formula;
pub use empty_or::empty_or;
pub use function_typed_list::function_typed_list;
pub use literal::literal;
pub use typed_list::typed_list;

// Utility parser combinators.
#[allow(unused_imports)]
pub(crate) use utilities::{
    parens, prefix_expr, space_separated_list0, space_separated_list1, ws, ws2,
};

#[cfg(test)]
mod tests {
    use crate::{BasicFunctionTerm, Parser};

    #[test]
    fn test_parse() {
        let (_, value) = BasicFunctionTerm::parse("abcde").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));
    }

    #[test]
    fn test_parse_span() {
        let (_, value) = BasicFunctionTerm::parse_span("abcde".into()).unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));
    }

    #[test]
    fn test_from_str() {
        let value = BasicFunctionTerm::from_str("abcde").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));
    }
}
