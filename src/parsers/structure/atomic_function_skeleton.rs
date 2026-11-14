//! Provides parsers for atomic function skeletons (`<atomic function skeleton>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_function_symbol, parse_variable, typed_list};
use crate::types::AtomicFunctionSkeleton;

/// Parses an atomic function skeleton (`<atomic function skeleton>`), i.e. `(<function-symbol> <typed list (variable)>)`.
pub(crate) fn parse_atomic_function_skeleton(node: &Node) -> R<AtomicFunctionSkeleton> {
    parens(node, |args| {
        let symbol = parse_function_symbol(args.next()?)?;
        let variables = typed_list(args, parse_variable)?;
        Ok(AtomicFunctionSkeleton::from((symbol, variables)))
    })
}

crate::parsers::impl_parser!(AtomicFunctionSkeleton => parse_atomic_function_skeleton);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let value = AtomicFunctionSkeleton::from_str("(battery-amount ?r - rover)").unwrap();
        check!(
            value
                == AtomicFunctionSkeleton::new(
                    FunctionSymbol::from("battery-amount"),
                    TypedList::from_iter([Variable::from("r").to_typed("rover")])
                )
        );
    }

    #[test]
    fn doc_example_0() {
        check!(
            AtomicFunctionSkeleton::from_str("(battery-amount ?r - rover)").is_value(
                AtomicFunctionSkeleton::new(
                    FunctionSymbol::from("battery-amount"),
                    TypedList::from_iter([Variable::from("r").to_typed("rover")])
                )
            )
        );
    }

    #[test]
    fn doc_example_1() {
        let value = AtomicFunctionSkeleton::from_str("(battery-amount ?r - rover)").unwrap();

        check!(
            value
                == AtomicFunctionSkeleton::new(
                    FunctionSymbol::from("battery-amount"),
                    TypedList::from_iter([Variable::from("r").to_typed("rover")])
                )
        );
    }
}
