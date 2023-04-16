use nom::character::complete::{char};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many0};
use nom::sequence::{preceded, tuple};
use crate::parsers::{parse_type, space_separated_list0, space_separated_list1, ws};
use crate::types::Typed;

/// Parser combinator that parses a typed list, i.e. `x* | x⁺ - <type> <typed-list (x)>.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use pddl::parsers::{parse_name, typed_list};
/// # use pddl::types::{Name, PrimitiveType, Type, Typed};
///
/// // Single implicitly typed element.
/// assert_eq!(typed_list(parse_name)("abc"), Ok(("", vec![
///     Typed::new(Name::from_str("abc"), Type::OBJECT)
/// ])));
///
/// // Multiple implicitly typed elements.
/// assert_eq!(typed_list(parse_name)("abc def\nghi"), Ok(("", vec![
///     Typed::new(Name::from_str("abc"), Type::OBJECT),
///     Typed::new(Name::from_str("def"), Type::OBJECT),
///     Typed::new(Name::from_str("ghi"), Type::OBJECT),
/// ])));
///
/// // Multiple explicitly typed elements.
/// assert_eq!(typed_list(parse_name)("abc def - word kitchen - room"), Ok(("", vec![
///     Typed::new(Name::from_str("abc"), Type::from("word")),
///     Typed::new(Name::from_str("def"), Type::from("word")),
///     Typed::new(Name::from_str("kitchen"), Type::from("room")),
/// ])));
///
/// // Mixed
/// assert_eq!(typed_list(parse_name)("abc def - word\ngeorgia - (either state country)\nuvw xyz"), Ok(("", vec![
///     Typed::new(Name::from_str("abc"), Type::from("word")),
///     Typed::new(Name::from_str("def"), Type::from("word")),
///     Typed::new(Name::from_str("georgia"), Type::from_iter(["state", "country"])),
///     Typed::new(Name::from_str("uvw"), Type::OBJECT),
///     Typed::new(Name::from_str("xyz"), Type::OBJECT),
/// ])));
/// ```
pub fn typed_list<'a, F, O>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Typed<O>>>
    where
        F: Clone + FnMut(&'a str) -> IResult<&'a str, O>,
{
    // `x*`
    let implicitly_typed = map(inner.clone(), |o| Typed::new_object(o));
    let implicitly_typed_list = space_separated_list0(implicitly_typed);

    // `x⁺ - <type>`
    let explicitly_typed = map(tuple((
        space_separated_list1(inner.clone()),
        preceded(ws(char('-')), parse_type)
    )), |(os, t)| {
        os.into_iter().map(move |o| Typed::new(o, t.clone())).collect::<Vec<_>>()
    });

    let typed_list_choice = tuple((
        map(many0(explicitly_typed), |vec| {
            vec.into_iter().flatten().collect::<Vec<_>>()
        }),
        implicitly_typed_list));

    let repeated_lists = map(typed_list_choice, |(mut explicit, mut implicit)| {
        explicit.append(&mut implicit);
        explicit
    });

    repeated_lists
}
