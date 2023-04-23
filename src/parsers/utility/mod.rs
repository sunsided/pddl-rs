//! Utility parsers such as [`typed_list`](crate::parsers::utility::typed_list) and [`empty_or`](crate::parsers::utility::empty_or).

mod empty_or;
mod typed_list;
mod utilities;

// Parser combinators.
pub use empty_or::empty_or;
pub use typed_list::typed_list;

// Utility parser combinators.
#[allow(unused_imports)]
pub(crate) use utilities::{parens, prefix_expr, space_separated_list0, space_separated_list1, ws};
