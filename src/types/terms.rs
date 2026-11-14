//! Types for names, variables, numbers, terms and types.

mod basic_function_term;
mod function_symbols;
mod function_term;
mod function_type;
mod function_typed;
mod function_typed_list;
mod name;
mod number;
mod predicate;
/// Contains terms via the [`Term`] type.
mod term;
mod r#type;
mod typed;
/// Contains lists of typed elements via the [`TypedList`] type.
mod typed_list;
mod variable;

pub use basic_function_term::BasicFunctionTerm;
pub use function_symbols::FunctionSymbol;
pub use function_term::FunctionTerm;
pub use function_type::FunctionType;
pub use function_typed::FunctionTyped;
pub use function_typed_list::FunctionTypedList;
pub use name::Name;
pub use number::Number;
pub use predicate::Predicate;
pub use r#type::{PrimitiveType, Type};
#[allow(unused_imports)]
pub(crate) use r#type::{TYPE_NUMBER, TYPE_OBJECT};
pub use term::Term;
pub use typed::{ToTyped, Typed};
pub use typed_list::TypedList;
pub use variable::Variable;
