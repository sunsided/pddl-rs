//! Provides parsers for numbers (`<number>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::types::Number;

/// Parses a number (`<number>`), i.e. `<digit>⁺[.<digit>⁺]`.
pub(crate) fn parse_number(node: &Node) -> R<Number> {
    let text = node
        .atom()
        .filter(|t| is_number(t))
        .ok_or_else(|| node.error())?;
    let value: f64 = text.parse().map_err(|_| node.error())?;
    // Too many digits overflow to infinity, which `Number` rejects. Fatal: no
    // other token starts with a digit, and trying alternatives would lose the reason.
    Number::try_new(value).map_err(|_| node.error().reason("number out of range").fatal())
}

/// Whether `text` is `<digit>⁺[.<digit>⁺]`.
fn is_number(text: &str) -> bool {
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    match text.split_once('.') {
        Some((whole, fraction)) => digits(whole) && digits(fraction),
        None => digits(text),
    }
}

crate::parsers::impl_parser!(Number => parse_number);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Number, Parser};

    #[test]
    fn test_parse() {
        assert!(Number::from_str("0").is_value(0.0.into()));
        assert!(Number::from_str("1000").is_value(1000.0.into()));
        assert!(Number::from_str("012").is_value(12.0.into()));
        assert!(Number::from_str("1.234").is_value(1.234.into()));
        assert!(Number::from_str("1.20").is_value(1.2.into()));

        assert!(Number::from_str("1000a").is_err());
        assert!(Number::from_str(".0").is_err());
        assert!(Number::from_str(".").is_err());
        assert!(Number::from_str("1.").is_err());
        assert!(Number::from_str("1.2.3").is_err());
        assert!(Number::from_str("-1").is_err());
        assert!(Number::from_str("1e5").is_err());
    }

    #[test]
    fn too_many_digits_is_an_error_not_a_panic() {
        // About 1e308 still fits an f64; one more digit overflows.
        assert!(Number::from_str(&"9".repeat(308)).is_ok());
        let error = Number::from_str(&"9".repeat(309)).unwrap_err();
        assert_eq!(error.reason(), Some("number out of range"));
        assert_eq!(
            (error.offset(), error.found().map(|f| &f[..3])),
            (0, Some("999"))
        );
    }
}
