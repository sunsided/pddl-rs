//! Provides parsers for intervals.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::interval::names;
use crate::types::Interval;

/// Parses an interval, ignoring case.
pub(crate) fn parse_interval(node: &Node) -> R<Interval> {
    word(node, &[(names::ALL, Interval::All)])
}

crate::parsers::impl_parser!(Interval => parse_interval);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Interval, Parser};

    #[test]
    fn test_parse() {
        assert!(Interval::from_str("all").is_value(Interval::All));
        assert!(Interval::from_str("any").is_err());
        assert!(Interval::from_str("(all)").is_err());
    }
}
