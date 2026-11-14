//! Provides parsers for function symbols (`<function-symbol>`).

use crate::parsers::error::R;
use crate::parsers::parse_name;
use crate::parsers::sexpr::Node;
use crate::types::FunctionSymbol;

/// Parses a function symbol (`<function-symbol>`), i.e. `<name>`.
pub(crate) fn parse_function_symbol(node: &Node) -> R<FunctionSymbol> {
    parse_name(node).map(FunctionSymbol::new)
}

crate::parsers::impl_parser!(FunctionSymbol => parse_function_symbol);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{FunctionSymbol, Parser};
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(FunctionSymbol::from_str("abcde").is_value("abcde".into()));
        check!(FunctionSymbol::from_str("a-1_2").is_value("a-1_2".into()));
        check!(FunctionSymbol::from_str("Z01").is_value("Z01".into()));
        check!(FunctionSymbol::from_str("x-_-_").is_value("x-_-_".into()));
        check!(FunctionSymbol::from_str("").is_err());
        check!(FunctionSymbol::from_str(".").is_err());
        check!(FunctionSymbol::from_str("-abc").is_err());
        check!(FunctionSymbol::from_str("0124").is_err());
        check!(FunctionSymbol::from_str("-1").is_err());
    }
}
