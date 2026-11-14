//! Provides parsers for preference names (`<pref-name>`).

use crate::parsers::error::R;
use crate::parsers::parse_name;
use crate::parsers::sexpr::Node;
use crate::types::PreferenceName;

/// Parses a preference name (`<pref-name>`), i.e. `<name>`.
pub(crate) fn parse_pref_name(node: &Node) -> R<PreferenceName> {
    parse_name(node).map(PreferenceName::new)
}

crate::parsers::impl_parser!(PreferenceName => parse_pref_name);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, PreferenceName};

    #[test]
    fn test_parse() {
        assert!(PreferenceName::from_str("abcde").is_value("abcde".into()));
        assert!(PreferenceName::from_str("a-1_2").is_value("a-1_2".into()));
        assert!(PreferenceName::from_str("Z01").is_value("Z01".into()));
        assert!(PreferenceName::from_str("x-_-_").is_value("x-_-_".into()));
        assert!(PreferenceName::from_str("").is_err());
        assert!(PreferenceName::from_str(".").is_err());
        assert!(PreferenceName::from_str("-abc").is_err());
        assert!(PreferenceName::from_str("0124").is_err());
        assert!(PreferenceName::from_str("-1").is_err());
    }
}
