//! Types for names, variables, numbers, terms and types.

pub(super) mod basic_function_term;
pub(super) mod function_symbols;
pub(super) mod function_term;
pub(super) mod function_type;
pub(super) mod function_typed;
pub(super) mod function_typed_list;
pub(super) mod name;
pub(super) mod number;
pub(super) mod predicate;
/// Contains terms via the [`Term`](crate::Term) type.
pub(super) mod term;
pub(super) mod r#type;
pub(super) mod typed;
/// Contains lists of typed elements via the [`TypedList`](crate::TypedList) type.
pub(super) mod typed_list;
pub(super) mod variable;
