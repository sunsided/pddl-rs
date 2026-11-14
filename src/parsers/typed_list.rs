//! Provides the [`typed_list`] rule.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_type, parse_variable, Args};
use crate::types::{Type, Typed, TypedList, Variable};

/// Parses `(<typed list (variable)>)`, e.g. the parameters of an action.
pub(crate) fn typed_variables(node: &Node) -> R<TypedList<Variable>> {
    parens(node, |args| typed_list(args, parse_variable))
}

/// Parses the remaining `args` as a typed list, i.e. `x* | x⁺ - <type> <typed-list (x)>`.
pub(crate) fn typed_list<'n, 'a, O>(
    args: &mut Args<'n, 'a>,
    inner: impl Fn(&'n Node<'a>) -> R<O>,
) -> R<TypedList<O>> {
    typed_groups(args, inner, Typed::new, Typed::object).map(TypedList::new)
}

/// Reads groups of `x⁺ - <type>` followed by untyped `x*`, building each typed
/// element with `typed` and each untyped one with `untyped`.
pub(crate) fn typed_groups<'n, 'a, O, E>(
    args: &mut Args<'n, 'a>,
    inner: impl Fn(&'n Node<'a>) -> R<O>,
    typed: impl Fn(O, Type) -> E,
    untyped: impl Fn(O) -> E,
) -> R<Vec<E>> {
    let mut elements = Vec::new();
    let mut group = Vec::new();
    while let Some(node) = args.peek() {
        if node.is("-") {
            if group.is_empty() {
                return Err(node.error());
            }
            args.next()?;
            let r#type = parse_type(args.next()?)?;
            elements.extend(group.drain(..).map(|o| typed(o, r#type.clone())));
        } else {
            group.push(inner(args.next()?)?);
        }
    }
    elements.extend(group.into_iter().map(untyped));
    Ok(elements)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{parse_name, parse_one};
    use crate::{Name, ToTyped};

    fn parse(input: &str) -> Result<TypedList<Name>, crate::ParseError> {
        parse_one(input, |node| {
            parens(node, |args| typed_list(args, parse_name))
        })
    }

    #[test]
    fn test_parse() {
        // Single implicitly typed element.
        assert_eq!(
            parse("(abc)").unwrap(),
            TypedList::from_iter([Name::new("abc").to_typed(Type::OBJECT)])
        );

        // Multiple implicitly typed elements.
        assert_eq!(
            parse("(abc def\nghi)").unwrap(),
            TypedList::from_iter([
                Name::new("abc").to_typed(Type::OBJECT),
                Name::new("def").to_typed(Type::OBJECT),
                Name::new("ghi").to_typed(Type::OBJECT)
            ])
        );

        // Multiple explicitly typed elements.
        assert_eq!(
            parse("(abc def - word kitchen - room)").unwrap(),
            TypedList::from_iter([
                Name::new("abc").to_typed("word"),
                Name::new("def").to_typed("word"),
                Name::new("kitchen").to_typed("room"),
            ])
        );

        // Mixed
        assert_eq!(
            parse("(abc def - word\ngeorgia - (either state country)\nuvw xyz)").unwrap(),
            TypedList::from_iter([
                Name::new("abc").to_typed("word"),
                Name::new("def").to_typed("word"),
                Name::new("georgia").to_typed_either(["state", "country"]),
                Name::new("uvw").to_typed(Type::OBJECT),
                Name::new("xyz").to_typed(Type::OBJECT)
            ])
        );

        assert_eq!(parse("()").unwrap(), TypedList::default());
    }

    #[test]
    fn errors_point_at_the_mistake() {
        assert_eq!(parse("(- t)").unwrap_err().offset(), 1);
        assert_eq!(parse("(a -)").unwrap_err().offset(), 4);
        assert_eq!(parse("(a - b 1)").unwrap_err().offset(), 7);
    }
}
