//! Provides parsers for names (`<name>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::types::Name;

/// Parses a name (`<name>`), i.e. `<letter><any char>*`, where `<any char>` is a letter, digit, `-` or `_`.
pub(crate) fn parse_name(node: &Node) -> R<Name> {
    match node.atom() {
        Some(text) if is_name(text) => Ok(Name::from(text)),
        _ => Err(node.error()),
    }
}

/// Whether `text` is a valid name: an ASCII letter, then ASCII letters, digits, `-` or `_`.
pub(crate) fn is_name(text: &str) -> bool {
    let mut chars = text.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

crate::parsers::impl_parser!(Name => parse_name);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Name, Parser};
    use assert2::check;

    #[test]
    fn parse_name_works() {
        check!(Name::from_str("abcde").is_value("abcde".into()));
        check!(Name::from_str("a-1_2").is_value("a-1_2".into()));
        check!(Name::from_str("Z01").is_value("Z01".into()));
        check!(Name::from_str("x-_-_").is_value("x-_-_".into()));
        check!(Name::from_str("").is_err());
        check!(Name::from_str(".").is_err());
        check!(Name::from_str("-abc").is_err());
        check!(Name::from_str("0124").is_err());
        check!(Name::from_str("-1").is_err());
        check!(Name::from_str("a.b").is_err());
        check!(Name::from_str("ä").is_err());
    }
}
