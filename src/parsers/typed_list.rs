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
/// let object = Type::from(PrimitiveType::from(Name::from_str("object")));
///
/// // Single implicitly typed element.
/// assert_eq!(typed_list(parse_name)("abc"), Ok(("", vec![Typed::new(Name::from_str("abc"), object.clone())])));
///
/// // Multiple implicitly typed elements.
/// assert_eq!(typed_list(parse_name)("abc def\nghi"), Ok(("", vec![
///     Typed::new(Name::from_str("abc"), object.clone()),
///     Typed::new(Name::from_str("def"), object.clone()),
///     Typed::new(Name::from_str("ghi"), object.clone()),
/// ])));
///
/// // Multiple explicitly typed elements.
/// let word = Type::from(PrimitiveType::from(Name::from_str("word")));
/// let room = Type::from(PrimitiveType::from(Name::from_str("room")));
/// assert_eq!(typed_list(parse_name)("abc def - word kitchen - room"), Ok(("", vec![
///     Typed::new(Name::from_str("abc"), word.clone()),
///     Typed::new(Name::from_str("def"), word.clone()),
///     Typed::new(Name::from_str("kitchen"), room.clone()),
/// ])));
///
/// // Mixed
/// assert_eq!(typed_list(parse_name)("abc def - word\nkitchen - room\nuvw xyz"), Ok(("", vec![
///     Typed::new(Name::from_str("abc"), word.clone()),
///     Typed::new(Name::from_str("def"), word),
///     Typed::new(Name::from_str("kitchen"), room),
///     Typed::new(Name::from_str("uvw"), object.clone()),
///     Typed::new(Name::from_str("xyz"), object),
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
