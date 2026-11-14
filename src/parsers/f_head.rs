//! Provides parsers for function heads (`<f-head>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_function_symbol, parse_term};
use crate::types::FunctionHead;

/// Parses a function head (`<f-head>`), i.e. `<function-symbol> | (<function-symbol> <term>*)`.
pub(crate) fn parse_f_head(node: &Node) -> R<FunctionHead> {
    if node.atom().is_some() {
        return parse_function_symbol(node).map(FunctionHead::new);
    }
    parens(node, |args| {
        let symbol = parse_function_symbol(args.next()?)?;
        if args.peek().is_none() {
            return Ok(FunctionHead::new(symbol));
        }
        let terms = args.list0(parse_term)?;
        Ok(FunctionHead::with_terms(symbol, terms))
    })
}

crate::parsers::impl_parser!(FunctionHead => parse_f_head);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(FunctionHead::from_str("fun-sym")
            .is_value(FunctionHead::new(FunctionSymbol::string("fun-sym"))));

        assert!(FunctionHead::from_str("(fun-sym)")
            .is_value(FunctionHead::new(FunctionSymbol::string("fun-sym"))));

        // Whitespace before `)` gives the same head as `(fun-sym)` (#119).
        assert!(FunctionHead::from_str("(fun-sym )")
            .is_value(FunctionHead::new(FunctionSymbol::string("fun-sym"))));

        assert!(
            FunctionHead::from_str("(fun-sym term)").is_value(FunctionHead::with_terms(
                FunctionSymbol::string("fun-sym"),
                [Term::Name("term".into())]
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(FunctionHead::from_str("fun-sym")
            .is_value(FunctionHead::new(FunctionSymbol::string("fun-sym"))));

        assert!(FunctionHead::from_str("(fun-sym)")
            .is_value(FunctionHead::new(FunctionSymbol::string("fun-sym"))));

        assert!(
            FunctionHead::from_str("(fun-sym term)").is_value(FunctionHead::with_terms(
                FunctionSymbol::string("fun-sym"),
                [Term::Name("term".into())]
            ))
        );
    }
}
