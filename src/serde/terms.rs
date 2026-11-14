//! Names, variables, terms, numbers and types.

use crate::serde::{
    deserialize_from_pddl, formula, serialize_deref, serialize_display, type_, Head,
};
use crate::{
    BasicFunctionTerm, FunctionSymbol, FunctionTerm, FunctionType, Name, Number, Predicate,
    PrimitiveType, Term, Type, Typed, TypedList, Variable,
};
use ::serde::ser::{SerializeMap, SerializeSeq};
use ::serde::{Serialize, Serializer};

/// The largest magnitude up to which every integer is exactly representable as `f64`.
const MAX_EXACT_INTEGER: f64 = 9_007_199_254_740_992.0;

impl Serialize for Name {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self)
    }
}

impl Serialize for Number {
    /// Whole numbers serialize as integers, so `5` doesn't come out as `5.0`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = **self;
        if value.fract() == 0.0 && value.abs() <= MAX_EXACT_INTEGER {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "checked to be a whole number in range"
            )]
            serializer.serialize_i64(value as i64)
        } else {
            serializer.serialize_f64(value)
        }
    }
}

serialize_deref!(Predicate, FunctionSymbol, PrimitiveType);
serialize_display!(Variable);

impl Serialize for Term {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Term::Name(name) => name.serialize(serializer),
            Term::Variable(variable) => variable.serialize(serializer),
            Term::Function(function) => function.serialize(serializer),
        }
    }
}

impl Serialize for FunctionTerm {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Head(self.symbol(), self.terms()).serialize(serializer)
    }
}

impl Serialize for BasicFunctionTerm {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Head(self.symbol(), self.names()).serialize(serializer)
    }
}

impl Serialize for Type {
    /// A single type is its name; `either` types are an array of names.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Type::Exactly(r#type) => r#type.serialize(serializer),
            Type::EitherOf(types) => {
                let mut seq = serializer.serialize_seq(Some(types.len()))?;
                for r#type in types {
                    seq.serialize_element(r#type)?;
                }
                seq.end()
            }
        }
    }
}

impl Serialize for FunctionType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl<T: Serialize> Serialize for Typed<T> {
    /// A single entry `{value: type}`, as in a [`TypedList`].
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(self.value(), self.type_())?;
        map.end()
    }
}

impl<T: Serialize> Serialize for TypedList<T> {
    /// An object from each value to its type, in order.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for typed in self.iter() {
            map.serialize_entry(typed.value(), typed.type_())?;
        }
        map.end()
    }
}

deserialize_from_pddl!(
    Name => formula,
    Number => formula,
    Variable => formula,
    Term => formula,
    FunctionTerm => formula,
    BasicFunctionTerm => formula,
    Predicate => formula,
    FunctionSymbol => formula,
    PrimitiveType => formula,
    Type => type_,
    FunctionType => type_,
);
