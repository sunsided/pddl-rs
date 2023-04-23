//! Contains utility types, such as [`Name`] and [`TypedList`].

mod name;
mod r#type;
mod typed;
mod typed_list;
mod types;

pub use name::Name;
pub use r#type::{PrimitiveType, Type};
pub use typed::{ToTyped, Typed};
pub use typed_list::TypedList;
pub use types::Types;

// Internal re-exports.
#[allow(unused_imports)]
pub(crate) use r#type::{TYPE_NUMBER, TYPE_OBJECT};
