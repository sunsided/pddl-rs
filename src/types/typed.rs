//! Contains typed elements.

use crate::types::{Type, TYPE_OBJECT};

/// A typed element.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Typed<'a, O>(O, Type<'a>);

impl<'a, O> Typed<'a, O> {
    pub const fn new(value: O, r#type: Type<'a>) -> Self {
        Self(value, r#type)
    }

    pub const fn new_object(value: O) -> Self {
        Self::new(value, Type::Exactly(TYPE_OBJECT))
    }

    pub const fn as_value(&self) -> &O {
        &self.0
    }

    pub const fn as_type(&self) -> &Type<'a> {
        &self.1
    }
}
