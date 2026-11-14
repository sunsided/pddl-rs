//! Provides parsers for terms (`<term>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_function_term, parse_name, parse_variable};
use crate::types::Term;

/// Parses a term (`<term>`), i.e. `<name> | <variable> | <function-term>`.
pub(crate) fn parse_term(node: &Node) -> R<Term> {
    if node.atom().is_some() {
        return parse_variable(node)
            .map(Term::Variable)
            .or_else(|_| parse_name(node).map(Term::Name));
    }
    parse_function_term(node).map(Term::Function)
}

crate::parsers::impl_parser!(Term => parse_term);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let value = Term::from_str("some-name").unwrap();
        check!(value == Term::Name("some-name".into()));

        let value = Term::from_str("?some-var").unwrap();
        check!(value == Term::Variable("some-var".into()));
    }

    #[test]
    fn doc_example_0() {
        check!(Term::from_str("abcde").is_value(Term::Name("abcde".into())));
        check!(Term::from_str("?abcde").is_value(Term::Variable("abcde".into())));
    }

    #[test]
    fn doc_example_1() {
        let value = Term::from_str("some-name").unwrap();
        check!(value == Term::Name("some-name".into()));

        let value = Term::from_str("?some-var").unwrap();
        check!(value == Term::Variable("some-var".into()));
    }
}
