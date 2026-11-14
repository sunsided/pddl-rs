//! Provides parsers for predicates (`<predicate>`).

use crate::parsers::error::R;
use crate::parsers::parse_name;
use crate::parsers::sexpr::Node;
use crate::types::Predicate;

/// Parses a predicate (`<predicate>`), i.e. `<name>`.
pub(crate) fn parse_predicate(node: &Node) -> R<Predicate> {
    parse_name(node).map(Predicate::from)
}

crate::parsers::impl_parser!(Predicate => parse_predicate);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, Predicate};

    #[test]
    fn test_parse() {
        assert!(Predicate::from_str("abcde").is_value("abcde".into()));
        assert!(Predicate::from_str("a-1_2").is_value("a-1_2".into()));
        assert!(Predicate::from_str("Z01").is_value("Z01".into()));
        assert!(Predicate::from_str("x-_-_").is_value("x-_-_".into()));
        assert!(Predicate::from_str("").is_err());
        assert!(Predicate::from_str(".").is_err());
        assert!(Predicate::from_str("-abc").is_err());
        assert!(Predicate::from_str("0124").is_err());
        assert!(Predicate::from_str("-1").is_err());
    }
}
