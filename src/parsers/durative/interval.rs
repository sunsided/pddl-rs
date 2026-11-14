//! Provides parsers for intervals (`<interval>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::durative::interval::names;
use crate::types::Interval;

/// Parses an interval (`<interval>`), ignoring case.
pub(crate) fn parse_interval(node: &Node) -> R<Interval> {
    word(node, &[(names::ALL, Interval::All)])
}

crate::parsers::impl_parser!(Interval => parse_interval);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Interval, Parser};
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(Interval::from_str("all").is_value(Interval::All));
        check!(Interval::from_str("any").is_err());
        check!(Interval::from_str("(all)").is_err());
    }
}
