//! Contains helper traits for dealing with [`ParseResult`] values.

use crate::parsers::{ParseResult, Span};
use crate::Type;

/// A helper trait for dealing with [`ParseResult`] values.
pub trait UnwrapValue<V> {
    /// Unwraps the value or panicks if there was none.
    fn unwrap_value(self) -> V;

    /// Unwraps the value and compares it with the specified value.
    fn is_value(&self, value: V) -> bool;
}

impl<'a, V> UnwrapValue<V> for ParseResult<'a, V>
where
    V: PartialEq,
{
    fn unwrap_value(self) -> V {
        self.expect("expected a value").1
    }

    fn is_value(&self, value: V) -> bool {
        match self {
            Ok((_, lhs)) => lhs.eq(&value),
            Err(_) => false,
        }
    }
}

/// A helper trait for dealing with [`ParseResult`] values.
pub trait Match<V> {
    /// Ensures the parser produced the specified value
    /// and leaves no remaining value to be parsed.
    #[allow(unused)]
    fn is_exactly(&self, value: V) -> bool {
        self.is_result("", value)
    }

    /// Ensures the remainder value are as specified.
    #[allow(unused)]
    fn is_result(&self, remainder: &str, value: V) -> bool;
}

impl<'a, E> Match<Option<&str>> for ParseResult<'a, Option<Span<'a>>, E> {
    fn is_result(&self, remainder: &str, value: Option<&str>) -> bool {
        if let Ok((lhs, rhs)) = self {
            if !remainder.eq(*lhs.fragment()) {
                false
            } else if value.is_none() {
                return rhs.is_none();
            } else if let Some(rhs) = rhs {
                value.eq(&Some(*rhs.fragment()))
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'a, T, E> Match<&'a str> for ParseResult<'a, T, E>
where
    T: From<&'a str> + PartialEq,
{
    fn is_result(&self, remainder: &str, value: &'a str) -> bool {
        if let Ok((lhs, rhs)) = self {
            if !remainder.eq(*lhs.fragment()) {
                false
            } else {
                T::from(value).eq(rhs)
            }
        } else {
            false
        }
    }
}

impl<'a, E> Match<Type> for ParseResult<'a, Type, E> {
    fn is_result(&self, remainder: &str, value: Type) -> bool {
        if let Ok((lhs, rhs)) = self {
            if !remainder.eq(*lhs.fragment()) {
                false
            } else {
                value.eq(rhs)
            }
        } else {
            false
        }
    }
}

impl<'a, T, E> Match<Vec<T>> for ParseResult<'a, Vec<T>, E>
where
    T: PartialEq,
{
    fn is_result(&self, remainder: &str, value: Vec<T>) -> bool {
        if let Ok((lhs, rhs)) = self {
            if !remainder.eq(*lhs.fragment()) {
                false
            } else {
                value.eq(rhs)
            }
        } else {
            false
        }
    }
}
