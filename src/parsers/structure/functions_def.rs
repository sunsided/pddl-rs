//! Provides parsers for function definitions (`<functions-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{function_typed_list, parse_atomic_function_skeleton, prefix};
use crate::types::Functions;

/// Parses a function definition (`<functions-def>`), i.e. `(:functions <function typed list (atomic function skeleton)>)`.
pub(crate) fn parse_functions_def(node: &Node) -> R<Functions> {
    prefix(node, ":functions", |args| {
        function_typed_list(args, parse_atomic_function_skeleton).map(Functions::new)
    })
}

crate::parsers::impl_parser!(Functions => parse_functions_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:functions (battery-amount ?r - rover))";
        check!(
            Functions::from_str(input).is_value(Functions::from_iter([FunctionTyped::number(
                AtomicFunctionSkeleton::new(
                    FunctionSymbol::string("battery-amount"),
                    TypedList::from_iter([Typed::new(
                        Variable::from("r"),
                        Type::Exactly("rover".into())
                    )])
                )
            )]))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "(:functions (battery-amount ?r - rover))";
        check!(
            Functions::from_str(input).is_value(Functions::from_iter([FunctionTyped::number(
                AtomicFunctionSkeleton::new(
                    FunctionSymbol::string("battery-amount"),
                    TypedList::from_iter([Typed::new(
                        Variable::from("r"),
                        Type::Exactly("rover".into())
                    )])
                )
            )]))
        );
    }
}
