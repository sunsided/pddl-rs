use crate::parsers::{parse_type, space_separated_list0, space_separated_list1, ws};
use crate::types::{Typed, TypedList};
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser combinator that parses a typed list, i.e. `x* | x⁺ - <type> <typed-list (x)>.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use pddl::parsers::{parse_name, typed_list};
/// # use pddl::types::{Name, PrimitiveType, Type, Typed, TypedList};
///
/// // Single implicitly typed element.
/// assert_eq!(typed_list(parse_name)("abc"), Ok(("", TypedList::from_iter([
///     Typed::new(Name::new("abc"), Type::OBJECT)
/// ]))));
///
/// // Multiple implicitly typed elements.
/// assert_eq!(typed_list(parse_name)("abc def\nghi"), Ok(("", TypedList::from_iter([
///     Typed::new(Name::new("abc"), Type::OBJECT),
///     Typed::new(Name::new("def"), Type::OBJECT),
///     Typed::new(Name::new("ghi"), Type::OBJECT),
/// ]))));
///
/// // Multiple explicitly typed elements.
/// assert_eq!(typed_list(parse_name)("abc def - word kitchen - room"), Ok(("", TypedList::from_iter([
///     Typed::new(Name::new("abc"), Type::from("word")),
///     Typed::new(Name::new("def"), Type::from("word")),
///     Typed::new(Name::new("kitchen"), Type::from("room")),
/// ]))));
///
/// // Mixed
/// assert_eq!(typed_list(parse_name)("abc def - word\ngeorgia - (either state country)\nuvw xyz"), Ok(("", TypedList::from_iter([
///     Typed::new(Name::new("abc"), Type::from("word")),
///     Typed::new(Name::new("def"), Type::from("word")),
///     Typed::new(Name::new("georgia"), Type::from_iter(["state", "country"])),
///     Typed::new(Name::new("uvw"), Type::OBJECT),
///     Typed::new(Name::new("xyz"), Type::OBJECT),
/// ]))));
/// ```
pub fn typed_list<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, TypedList<O>>
where
    F: Clone + FnMut(&'a str) -> IResult<&'a str, O>,
{
    // `x*`
    let implicitly_typed = map(inner.clone(), |o| Typed::new_object(o));
    let implicitly_typed_list = space_separated_list0(implicitly_typed);

    // `x⁺ - <type>`
    let explicitly_typed = map(
        tuple((
            space_separated_list1(inner.clone()),
            preceded(ws(char('-')), parse_type),
        )),
        |(os, t)| {
            os.into_iter()
                .map(move |o| Typed::new(o, t.clone()))
                .collect::<Vec<_>>()
        },
    );

    let typed_list_choice = tuple((
        map(many0(explicitly_typed), |vec| {
            vec.into_iter().flatten().collect::<Vec<_>>()
        }),
        implicitly_typed_list,
    ));

    let repeated_lists = map(typed_list_choice, |(mut explicit, mut implicit)| {
        explicit.append(&mut implicit);
        TypedList::new(explicit)
    });

    repeated_lists
}
