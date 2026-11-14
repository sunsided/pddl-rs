//! Provides parsers for durative action symbols.

use crate::parsers::error::R;
use crate::parsers::parse_name;
use crate::parsers::sexpr::Node;
use crate::types::DurativeActionSymbol;

/// Parses a durative action symbol, i.e. `<name>`.
pub(crate) fn parse_da_symbol(node: &Node) -> R<DurativeActionSymbol> {
    parse_name(node).map(DurativeActionSymbol::from)
}

crate::parsers::impl_parser!(DurativeActionSymbol => parse_da_symbol);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{DurativeActionSymbol, Parser};

    #[test]
    fn test_parse() {
        assert!(DurativeActionSymbol::from_str("abcde").is_value("abcde".into()));
        assert!(DurativeActionSymbol::from_str("a-1_2").is_value("a-1_2".into()));
        assert!(DurativeActionSymbol::from_str("Z01").is_value("Z01".into()));
        assert!(DurativeActionSymbol::from_str("x-_-_").is_value("x-_-_".into()));
        assert!(DurativeActionSymbol::from_str("").is_err());
        assert!(DurativeActionSymbol::from_str(".").is_err());
        assert!(DurativeActionSymbol::from_str("-abc").is_err());
        assert!(DurativeActionSymbol::from_str("0124").is_err());
        assert!(DurativeActionSymbol::from_str("-1").is_err());
    }
}
