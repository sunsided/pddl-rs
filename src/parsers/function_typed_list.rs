use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::{
    parse_type, space_separated_list0, space_separated_list1, ws, ParseError, ParseResult, Span,
};
use crate::types::{FunctionType, FunctionTyped, FunctionTypedList};

pub fn function_typed_list<'a, F, O>(
    inner: F,
) -> impl Parser<Span<'a>, Output = FunctionTypedList<O>, Error = ParseError<'a>>
where
    F: Clone + Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    // `x*`
    let implicitly_typed = map(inner.clone(), |o| FunctionTyped::new_number(o));
    let implicitly_typed_list = space_separated_list0(implicitly_typed);

    // `x⁺ - <type>`
    let explicitly_typed = map(
        (
            space_separated_list1(inner.clone()),
            preceded(ws(char('-')), parse_type),
        ),
        |(os, t)| {
            os.into_iter()
                .map(move |o| FunctionTyped::new(o, FunctionType::new(t.clone())))
                .collect::<Vec<_>>()
        },
    );

    let typed_list_choice = (
        map(many0(explicitly_typed), |vec| {
            vec.into_iter().flatten().collect::<Vec<_>>()
        }),
        implicitly_typed_list,
    );

    map(typed_list_choice, |(mut explicit, mut implicit)| {
        explicit.append(&mut implicit);
        FunctionTypedList::new(explicit)
    })
}

#[cfg(test)]
mod tests {
    use crate::parsers::{function_typed_list, parse_atomic_function_skeleton, UnwrapValue};
    use crate::{
        AtomicFunctionSkeleton, FunctionSymbol, FunctionTyped, FunctionTypedList, Type, Typed,
        TypedList, Variable,
    };
    use nom::Parser;

    #[test]
    fn test_parse() {
        assert!(function_typed_list(parse_atomic_function_skeleton)
            .parse("(battery-amount ?r - rover)".into())
            .is_value(FunctionTypedList::from_iter([FunctionTyped::new_number(
                AtomicFunctionSkeleton::new(
                    FunctionSymbol::from_str("battery-amount"),
                    TypedList::from_iter([Typed::new(
                        Variable::from("r"),
                        Type::Exactly("rover".into())
                    )])
                )
            )])));

        assert!(function_typed_list(parse_atomic_function_skeleton)
            .parse("(move ?from ?to - location)".into())
            .is_value(FunctionTypedList::from_iter([FunctionTyped::new_number(
                AtomicFunctionSkeleton::new(
                    FunctionSymbol::from_str("move"),
                    TypedList::from_iter([
                        Typed::new(Variable::from("from"), Type::Exactly("location".into())),
                        Typed::new(Variable::from("to"), Type::Exactly("location".into()))
                    ])
                )
            )])));
    }
}
