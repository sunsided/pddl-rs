//! Parsers for names, variables, numbers, terms and types.

mod basic_function_term;
mod function_symbol;
mod function_term;
mod function_type;
mod function_typed_list;
mod name;
mod number;
mod predicate;
mod primitive_type;
mod term;
mod r#type;
mod typed_list;
mod variable;

pub(crate) use basic_function_term::parse_basic_function_term;
pub(crate) use function_symbol::parse_function_symbol;
pub(crate) use function_term::parse_function_term;
pub(crate) use function_typed_list::function_typed_list;
pub(crate) use name::{is_name, parse_name};
pub(crate) use number::parse_number;
pub(crate) use predicate::parse_predicate;
pub(crate) use primitive_type::parse_primitive_type;
pub(crate) use r#type::parse_type;
pub(crate) use term::parse_term;
pub(crate) use typed_list::{typed_list, typed_variables};
pub(crate) use variable::parse_variable;
