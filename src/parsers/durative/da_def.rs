//! Provides parsers for durative action definitions (`<durative-action-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    empty_or, keyword_arg, parse_da_effect, parse_da_gd, parse_da_symbol,
    parse_duration_constraint, prefix, typed_variables,
};
use crate::types::DurativeActionDefinition;

/// Parses a durative action definition (`<durative-action-def>`), i.e. `(:durative-action <da-symbol>
/// :parameters (<typed list (variable)>) :duration <duration-constraint>
/// :condition <da-GD> :effect <da-effect>)`. `()` stands for no condition or effect.
pub(crate) fn parse_da_def(node: &Node) -> R<DurativeActionDefinition> {
    prefix(node, ":durative-action", |args| {
        let symbol = parse_da_symbol(args.next()?)?;
        let parameters = keyword_arg(args, ":parameters", typed_variables)?;
        let duration = keyword_arg(args, ":duration", parse_duration_constraint)?;
        let condition = keyword_arg(args, ":condition", |n| empty_or(n, parse_da_gd))?;
        let effect = keyword_arg(args, ":effect", |n| empty_or(n, parse_da_effect))?;
        Ok(DurativeActionDefinition::new(
            symbol, parameters, duration, condition, effect,
        ))
    })
}

crate::parsers::impl_parser!(DurativeActionDefinition => parse_da_def);

#[cfg(test)]
mod tests {
    use assert2::check;

    use crate::*;

    #[test]
    fn it_works() {
        let input = r#"(:durative-action move
                :parameters
                    (?r - rover
                     ?fromwp - waypoint
                     ?towp - waypoint)

                :duration
                    (= ?duration 5)

                :condition
                    (and
                        (at start (rover ?rover))
                        (at start (waypoint ?from-waypoint))
                        (at start (waypoint ?to-waypoint))
                        (over all (can-move ?from-waypoint ?to-waypoint))
                        (at start (at ?rover ?from-waypoint))
                        (at start (> (battery-amount ?rover) 8)))

                :effect
                    (and
                        (decrease (fuel-level ?t) (* 2 #t))
                        (at end (at ?rover ?to-waypoint))
                        (at end (been-at ?rover ?to-waypoint))
                        (at start (not (at ?rover ?from-waypoint)))
                        (at start (decrease (battery-amount ?rover) 8))
                        (at end (increase (distance-travelled) 5))
                        )
            )"#;
        let _gd = DurativeActionDefinition::from_str(input).unwrap();
    }

    #[test]
    fn doc_example_0() {
        let input = r#"(:durative-action move
                :parameters
                    (?r - rover
                     ?fromwp - waypoint
                     ?towp - waypoint)
        
                :duration
                    (= ?duration 5)
        
                :condition
                    (and
                        (at start (rover ?rover))
                        (at start (waypoint ?from-waypoint))
                        (at start (waypoint ?to-waypoint))
                        (over all (can-move ?from-waypoint ?to-waypoint))
                        (at start (at ?rover ?from-waypoint))
                        (at start (> (battery-amount ?rover) 8)))
        
                :effect
                    (and
                        (decrease (fuel-level ?t) (* 2 #t))
                        (at end (at ?rover ?to-waypoint))
                        (at end (been-at ?rover ?to-waypoint))
                        (at start (not (at ?rover ?from-waypoint)))
                        (at start (decrease (battery-amount ?rover) 8))
                        (at end (increase (distance-travelled) 5))
                        )
            )"#;

        let da_def = DurativeActionDefinition::from_str(input).unwrap();
        check!(da_def.symbol() == &"move".into());
        check!(da_def.parameters().len() == 3);
        check!(da_def.duration().is_some());
        check!(da_def.condition().is_some());
        check!(da_def.effect().is_some());
    }
}
