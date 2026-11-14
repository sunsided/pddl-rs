//! Provides parsers for action symbols.

use crate::parsers::error::R;
use crate::parsers::parse_name;
use crate::parsers::sexpr::Node;
use crate::types::ActionSymbol;

/// Parses a action symbol, i.e. `<name>`.
pub(crate) fn parse_action_symbol(node: &Node) -> R<ActionSymbol> {
    parse_name(node).map(ActionSymbol::new)
}

crate::parsers::impl_parser!(ActionSymbol => parse_action_symbol);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{ActionSymbol, Parser};

    #[test]
    fn test_parse() {
        assert!(ActionSymbol::from_str("abcde").is_value("abcde".into()));
        assert!(ActionSymbol::from_str("a-1_2").is_value("a-1_2".into()));
        assert!(ActionSymbol::from_str("Z01").is_value("Z01".into()));
        assert!(ActionSymbol::from_str("x-_-_").is_value("x-_-_".into()));
        assert!(ActionSymbol::from_str("").is_err());
        assert!(ActionSymbol::from_str(".").is_err());
        assert!(ActionSymbol::from_str("-abc").is_err());
        assert!(ActionSymbol::from_str("0124").is_err());
        assert!(ActionSymbol::from_str("-1").is_err());
    }
}
