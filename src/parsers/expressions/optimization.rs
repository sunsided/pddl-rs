//! Provides parsers for optimization goals (`<optimization>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::expressions::optimization::names;
use crate::types::Optimization;

/// Parses an optimization goal (`<optimization>`), ignoring case.
pub(crate) fn parse_optimization(node: &Node) -> R<Optimization> {
    word(
        node,
        &[
            (names::MINIMIZE, Optimization::Minimize),
            (names::MAXIMIZE, Optimization::Maximize),
        ],
    )
}

crate::parsers::impl_parser!(Optimization => parse_optimization);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Optimization, Parser};

    #[test]
    fn test_parse() {
        assert!(Optimization::from_str("MAXIMIZE").is_value(Optimization::Maximize));
        assert!(Optimization::from_str("minimise").is_err());
        assert!(Optimization::from_str("(MAXIMIZE)").is_err());
    }
}
