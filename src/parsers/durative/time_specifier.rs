//! Provides parsers for time specifiers (`<time-specifier>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::durative::time_specifier::names;
use crate::types::TimeSpecifier;

/// Parses a time specifier (`<time-specifier>`), ignoring case.
pub(crate) fn parse_time_specifier(node: &Node) -> R<TimeSpecifier> {
    word(
        node,
        &[
            (names::START, TimeSpecifier::Start),
            (names::END, TimeSpecifier::End),
        ],
    )
}

crate::parsers::impl_parser!(TimeSpecifier => parse_time_specifier);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, TimeSpecifier};
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(TimeSpecifier::from_str("END").is_value(TimeSpecifier::End));
        check!(TimeSpecifier::from_str("ending").is_err());
        check!(TimeSpecifier::from_str("(END)").is_err());
    }
}
