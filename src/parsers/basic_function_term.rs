//! Provides parsers for basic function terms.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_function_symbol, parse_name};
use crate::types::BasicFunctionTerm;

/// Parses a basic function term, i.e. `<function-symbol> | (<function-symbol> <name>*)`.
pub(crate) fn parse_basic_function_term(node: &Node) -> R<BasicFunctionTerm> {
    if node.atom().is_some() {
        return parse_function_symbol(node).map(|symbol| BasicFunctionTerm::new(symbol, []));
    }
    parens(node, |args| {
        let symbol = parse_function_symbol(args.next()?)?;
        let names = args.list0(parse_name)?;
        Ok(BasicFunctionTerm::new(symbol, names))
    })
}

crate::parsers::impl_parser!(BasicFunctionTerm => parse_basic_function_term);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let value = BasicFunctionTerm::from_str("abcde").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));

        let value = BasicFunctionTerm::from_str("(abcde)").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));

        let value = BasicFunctionTerm::from_str("(abcde x y z)").unwrap();
        assert_eq!(
            value,
            BasicFunctionTerm::new("abcde".into(), ["x".into(), "y".into(), "z".into()])
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(BasicFunctionTerm::from_str("abcde")
            .is_value(BasicFunctionTerm::new("abcde".into(), [])));

        assert!(BasicFunctionTerm::from_str("(abcde)")
            .is_value(BasicFunctionTerm::new("abcde".into(), [])));

        assert!(
            BasicFunctionTerm::from_str("(abcde x y z)").is_value(BasicFunctionTerm::new(
                "abcde".into(),
                ["x".into(), "y".into(), "z".into()]
            ))
        );
    }

    #[test]
    fn doc_example_1() {
        let value = BasicFunctionTerm::from_str("abcde").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));

        let value = BasicFunctionTerm::from_str("(abcde)").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));

        let value = BasicFunctionTerm::from_str("(abcde x y z)").unwrap();
        assert_eq!(
            value,
            BasicFunctionTerm::new("abcde".into(), ["x".into(), "y".into(), "z".into()])
        );
    }
}
