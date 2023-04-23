use crate::parsers::utility::{parse_type, space_separated_list0, space_separated_list1, ws};
use crate::types::domain::{FunctionType, FunctionTyped, FunctionTypedList};
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
/// # use pddl::parsers::domain::{function_typed_list, parse_atomic_function_skeleton};
/// # use pddl::types::domain::{AtomicFunctionSkeleton, FunctionSymbol, FunctionTyped, FunctionTypedList, Variable};
/// # use pddl::types::utility::{Type, Typed, TypedList};
///
/// // Single implicitly typed element.
/// assert_eq!(function_typed_list(parse_atomic_function_skeleton)("(battery-amount ?r - rover)"), Ok(("",
///     FunctionTypedList::from_iter([
///         FunctionTyped::new_number(
///             AtomicFunctionSkeleton::new(
///                 FunctionSymbol::from_str("battery-amount"),
///                 TypedList::from_iter([
///                     Typed::new(Variable::from("r"), Type::Exactly("rover".into()))
///                 ])
///             )
///         )
///     ])
/// )));
/// ```
pub fn function_typed_list<'a, F, O>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, FunctionTypedList<O>>
where
    F: Clone + FnMut(&'a str) -> IResult<&'a str, O>,
{
    // TODO: With :numeric-fluents, this list can be x⁺ (i.e., implicitly typed number).
    // TODO: Without :numeric-fluents, this list is allowed to be empty or an explicitly typed list.

    // `x*`
    let implicitly_typed = map(inner.clone(), |o| FunctionTyped::new_number(o));
    let implicitly_typed_list = space_separated_list0(implicitly_typed);

    // `x⁺ - <type>`
    let explicitly_typed = map(
        tuple((
            space_separated_list1(inner.clone()),
            preceded(ws(char('-')), parse_type),
        )),
        |(os, t)| {
            os.into_iter()
                .map(move |o| FunctionTyped::new(o, FunctionType::new(t.clone())))
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
        FunctionTypedList::new(explicit)
    });

    repeated_lists
}
