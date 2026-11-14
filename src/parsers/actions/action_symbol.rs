//! Provides parsers for action symbols (`<action-symbol>`).

use crate::parsers::error::R;
use crate::parsers::parse_name;
use crate::parsers::sexpr::Node;
use crate::types::ActionSymbol;

/// Parses an action symbol (`<action-symbol>`), i.e. `<name>`.
pub(crate) fn parse_action_symbol(node: &Node) -> R<ActionSymbol> {
    parse_name(node).map(ActionSymbol::new)
}

crate::parsers::impl_parser!(ActionSymbol => parse_action_symbol);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{ActionSymbol, Parser};
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(ActionSymbol::from_str("abcde").is_value("abcde".into()));
        check!(ActionSymbol::from_str("a-1_2").is_value("a-1_2".into()));
        check!(ActionSymbol::from_str("Z01").is_value("Z01".into()));
        check!(ActionSymbol::from_str("x-_-_").is_value("x-_-_".into()));
        check!(ActionSymbol::from_str("").is_err());
        check!(ActionSymbol::from_str(".").is_err());
        check!(ActionSymbol::from_str("-abc").is_err());
        check!(ActionSymbol::from_str("0124").is_err());
        check!(ActionSymbol::from_str("-1").is_err());
    }
}
