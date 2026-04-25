//! Contains function types via the [`FunctionType`] ... type.

use crate::types::{PrimitiveType, Type};
use std::ops::Deref;

/// A function type.
///
/// ## Requirements
/// Requires [Fluents](crate::Requirement::Fluents), as well as either
/// [Numeric Fluents](crate::Requirement::NumericFluents) for the default `number` type, or
/// [Object Fluents](crate::Requirement::ObjectFluents) and [Typing](crate::Requirement::Typing)
/// for an arbitrary type.
///
/// ## Usage
/// Used by [`FunctionTyped`](crate::FunctionTyped) in [`FunctionTypedList`](crate::FunctionTypedList).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType(Type);

impl FunctionType {
    pub const NUMBER: FunctionType = FunctionType::from(Type::NUMBER);

    pub fn new<T: Into<Type>>(r#type: T) -> Self {
        Self(r#type.into())
    }

    pub const fn from(r#type: Type) -> Self {
        Self(r#type)
    }
}

impl Default for FunctionType {
    fn default() -> Self {
        Self(Type::NUMBER)
    }
}

impl From<&str> for FunctionType {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<Vec<&str>> for FunctionType {
    fn from(value: Vec<&str>) -> Self {
        Self(Type::from_iter(
            value.iter().map(|&x| PrimitiveType::from(x)),
        ))
    }
}

impl From<Type> for FunctionType {
    fn from(value: Type) -> Self {
        Self(value)
    }
}

impl Deref for FunctionType {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Name, PrimitiveType};

    #[test]
    fn default_is_number() {
        let ft = FunctionType::default();
        assert_eq!(ft, FunctionType::NUMBER);
    }

    #[test]
    fn number_constant() {
        assert_eq!(FunctionType::NUMBER.deref(), &Type::NUMBER);
    }

    #[test]
    fn new_from_type() {
        let ft = FunctionType::new(Type::OBJECT);
        assert_eq!(*ft, Type::OBJECT);
    }

    #[test]
    fn new_from_str() {
        let ft = FunctionType::new("location");
        assert_eq!(
            *ft,
            Type::exactly(PrimitiveType::new(Name::new("location")))
        );
    }

    #[test]
    fn from_str() {
        let ft: FunctionType = "number".into();
        assert_eq!(*ft, Type::NUMBER);
    }

    #[test]
    fn from_vec_str() {
        let ft: FunctionType = vec!["location", "room"].into();
        assert!(matches!(*ft, Type::EitherOf(_)));
        if let Type::EitherOf(types) = &*ft {
            assert_eq!(types.len(), 2);
        }
    }

    #[test]
    fn from_type() {
        let t = Type::OBJECT;
        let ft = FunctionType::from(t);
        assert_eq!(*ft, Type::OBJECT);
    }

    #[test]
    fn from_const() {
        let ft = FunctionType::from(Type::NUMBER);
        assert_eq!(ft, FunctionType::NUMBER);
    }

    #[test]
    fn deref_works() {
        let ft = FunctionType::new(Type::OBJECT);
        let t: &Type = &ft;
        assert_eq!(t, &Type::OBJECT);
    }

    #[test]
    fn clone_works() {
        let ft = FunctionType::new(Type::OBJECT);
        let clone = ft.clone();
        assert_eq!(ft, clone);
    }

    #[test]
    fn eq_works() {
        let a = FunctionType::new(Type::OBJECT);
        let b = FunctionType::new(Type::OBJECT);
        let c = FunctionType::NUMBER;
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn debug_impl() {
        let ft = FunctionType::NUMBER;
        let dbg = format!("{ft:?}");
        assert!(dbg.contains("FunctionType"));
    }
}
