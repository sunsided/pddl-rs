//! Utility parsers such as [`parse_name`] and [`typed_list`](crate::parsers::utility::typed_list).

mod empty_or;
mod name;
pub(crate) mod number;
mod r#type;
mod typed_list;
mod types_def;
mod utilities;

// Parsers
pub use name::parse_name;
pub use number::parse_number;
pub use r#type::parse_type;
pub use types_def::parse_types_def;

// Parser combinators.
pub use empty_or::empty_or;
pub use typed_list::typed_list;

// Utility parser combinators.
#[allow(unused_imports)]
pub(crate) use utilities::{parens, prefix_expr, space_separated_list0, space_separated_list1, ws};
