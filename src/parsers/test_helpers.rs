//! Helpers for asserting on [`Parser::from_str`](crate::parsers::Parser::from_str) results.

use crate::parsers::ParseError;

/// Shorthands for checking a parse result in tests.
pub(crate) trait UnwrapValue<V> {
    /// Unwraps the value or panics with the error.
    fn unwrap_value(self) -> V;

    /// Whether parsing succeeded with exactly `value`.
    fn is_value(&self, value: V) -> bool;
}

impl<V: PartialEq> UnwrapValue<V> for Result<V, ParseError> {
    fn unwrap_value(self) -> V {
        self.unwrap_or_else(|e| panic!("expected a value: {e}"))
    }

    fn is_value(&self, value: V) -> bool {
        self.as_ref().is_ok_and(|v| *v == value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Name, Parser};

    #[test]
    fn test_is_value() {
        assert!(Name::from_str("abcde").is_value("abcde".into()));
    }

    #[test]
    fn test_is_value_fails() {
        assert!(!Name::from_str("abcde").is_value("xyz".into()));
        assert!(!Name::from_str("-").is_value("abcde".into()));
    }

    #[test]
    fn test_unwrap_value() {
        assert_eq!(Name::from_str("abcde").unwrap_value(), "abcde");
    }
}
