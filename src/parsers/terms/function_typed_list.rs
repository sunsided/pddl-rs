//! Provides the [`function_typed_list`] rule.

use super::typed_list::typed_groups;
use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::Args;
use crate::types::{FunctionType, FunctionTyped, FunctionTypedList};

/// Parses the remaining `args` as a function typed list, i.e.
/// `x* | x⁺ - <function-type> <function-typed-list (x)>`; untyped elements are `number`.
pub(crate) fn function_typed_list<'n, 'a, O>(
    args: &mut Args<'n, 'a>,
    inner: impl Fn(&'n Node<'a>) -> R<O>,
) -> R<FunctionTypedList<O>> {
    let typed = |o, t| FunctionTyped::new(o, FunctionType::new(t));
    typed_groups(args, inner, typed, FunctionTyped::number).map(FunctionTypedList::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{parens, parse_atomic_function_skeleton, parse_one};
    use crate::{AtomicFunctionSkeleton, FunctionSymbol, Type, Typed, TypedList, Variable};
    use assert2::check;

    fn parse(input: &str) -> FunctionTypedList<AtomicFunctionSkeleton> {
        parse_one(input, |node| {
            parens(node, |args| {
                function_typed_list(args, parse_atomic_function_skeleton)
            })
        })
        .unwrap()
    }

    #[test]
    fn test_parse() {
        check!(
            parse("((battery-amount ?r - rover))")
                == FunctionTypedList::from_iter([FunctionTyped::number(
                    AtomicFunctionSkeleton::new(
                        FunctionSymbol::string("battery-amount"),
                        TypedList::from_iter([Typed::new(
                            Variable::from("r"),
                            Type::Exactly("rover".into())
                        )])
                    )
                )])
        );

        check!(
            parse("((move ?from ?to - location) - number (f))")
                == FunctionTypedList::from_iter([
                    FunctionTyped::new(
                        AtomicFunctionSkeleton::new(
                            FunctionSymbol::string("move"),
                            TypedList::from_iter([
                                Typed::new(
                                    Variable::from("from"),
                                    Type::Exactly("location".into())
                                ),
                                Typed::new(Variable::from("to"), Type::Exactly("location".into()))
                            ])
                        ),
                        FunctionType::new(Type::Exactly("number".into()))
                    ),
                    FunctionTyped::number(AtomicFunctionSkeleton::new(
                        FunctionSymbol::string("f"),
                        TypedList::default()
                    )),
                ])
        );
    }
}
