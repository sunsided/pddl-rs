//! Provides parsers for length specifications (`<length-spec>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{prefix, Args};
use crate::types::LengthSpec;

/// Parses a length specification (`<length-spec>`), i.e. `(:length [(:serial <integer>)] [(:parallel <integer>)])`.
///
/// Deprecated since PDDL 2.1.
pub(crate) fn parse_problem_length_spec(node: &Node) -> R<LengthSpec> {
    prefix(node, ":length", |args| {
        let serial = opt_limit(args, ":serial")?;
        let parallel = opt_limit(args, ":parallel")?;
        Ok(LengthSpec::new(serial, parallel))
    })
}

/// Parses `(<keyword> <integer>)` if the next child starts with `keyword`.
fn opt_limit(args: &mut Args, keyword: &'static str) -> R<Option<u64>> {
    match args.peek() {
        Some(node) if node.head().is_some_and(|h| h.eq_ignore_ascii_case(keyword)) => {
            let node = args.next()?;
            prefix(node, keyword, |args| {
                let value = args.next()?;
                value
                    .atom()
                    .filter(|a| a.bytes().all(|b| b.is_ascii_digit()))
                    .and_then(|a| a.parse().ok())
                    .ok_or_else(|| value.error())
            })
            .map(Some)
        }
        _ => Ok(None),
    }
}

crate::parsers::impl_parser!(LengthSpec => parse_problem_length_spec);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        check!(LengthSpec::from_str("(:length)").is_value(LengthSpec::default()));
        check!(
            LengthSpec::from_str("(:length (:serial 123))").is_value(LengthSpec::new_serial(123))
        );
        check!(
            LengthSpec::from_str("(:length (:parallel 42))").is_value(LengthSpec::new_parallel(42))
        );
        check!(
            LengthSpec::from_str("(:length (:serial 123) (:parallel 42))")
                .is_value(LengthSpec::new(Some(123), Some(42)))
        );
    }

    #[test]
    fn doc_example_0() {
        check!(LengthSpec::from_str("(:length)").is_value(LengthSpec::default()));
        check!(
            LengthSpec::from_str("(:length (:serial 123))").is_value(LengthSpec::new_serial(123))
        );
        check!(
            LengthSpec::from_str("(:length (:parallel 42))").is_value(LengthSpec::new_parallel(42))
        );
        check!(
            LengthSpec::from_str("(:length (:serial 123) (:parallel 42))")
                .is_value(LengthSpec::new(Some(123), Some(42)))
        );
    }
}
