//! Provides parsers for durative action function assignments (`<f-assign-da>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_assign_op, parse_f_exp_da, parse_f_head};
use crate::types::DurativeActionFunctionAssignment;

/// Parses a durative action function assignment (`<f-assign-da>`), i.e. `(<assign-op> <f-head> <f-exp-da>)`.
pub(crate) fn parse_f_assign_da(node: &Node) -> R<DurativeActionFunctionAssignment> {
    parens(node, |args| {
        Ok((
            parse_assign_op(args.next()?)?,
            parse_f_head(args.next()?)?,
            parse_f_exp_da(args.next()?)?,
        ))
    })
    .map(DurativeActionFunctionAssignment::from)
}

crate::parsers::impl_parser!(DurativeActionFunctionAssignment => parse_f_assign_da);

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn it_works() {
        let input = "(increase (distance-travelled) 5)";
        let _effect = DurativeActionFunctionAssignment::from_str(input).unwrap();
    }

    #[test]
    fn doc_example_0() {
        assert!(DurativeActionFunctionAssignment::from_str("(assign fun-sym ?duration)").is_ok());
    }
}
