//! Provides parsers for timed fluent expressions (`<f-exp-t>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_f_exp, prefix};
use crate::types::TimedFluentExpression;

/// Parses a timed fluent expression (`<f-exp-t>`), i.e. `(* <f-exp> #t) | (* #t <f-exp>) | #t`.
pub(crate) fn parse_f_exp_t(node: &Node) -> R<TimedFluentExpression> {
    if node.atom().is_some() {
        return if node.is("#t") {
            Ok(TimedFluentExpression::new())
        } else {
            Err(node.error())
        };
    }
    prefix(node, "*", |args| {
        let first = args.next()?;
        if first.is("#t") {
            return parse_f_exp(args.next()?);
        }
        let exp = parse_f_exp(first)?;
        args.expect("#t")?;
        Ok(exp)
    })
    .map(TimedFluentExpression::new_scaled)
}

crate::parsers::impl_parser!(TimedFluentExpression => parse_f_exp_t);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(TimedFluentExpression::from_str("#t").is_value(TimedFluentExpression::Now));

        assert!(
            TimedFluentExpression::from_str("(* (fuel ?tank) #t)").is_value(
                TimedFluentExpression::scaled(FluentExpression::function(
                    FunctionHead::with_terms(
                        FunctionSymbol::string("fuel"),
                        [Term::Variable(Variable::string("tank"))]
                    )
                ))
            )
        );

        assert!(
            TimedFluentExpression::from_str("(* #t (fuel ?tank))").is_value(
                TimedFluentExpression::scaled(FluentExpression::function(
                    FunctionHead::with_terms(
                        FunctionSymbol::string("fuel"),
                        [Term::Variable(Variable::string("tank"))]
                    )
                ))
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(TimedFluentExpression::from_str("#t").is_value(TimedFluentExpression::Now));

        assert!(
            TimedFluentExpression::from_str("(* (fuel ?tank) #t)").is_value(
                TimedFluentExpression::scaled(FluentExpression::function(
                    FunctionHead::with_terms(
                        FunctionSymbol::string("fuel"),
                        [Term::Variable(Variable::string("tank"))]
                    )
                ))
            )
        );

        assert!(
            TimedFluentExpression::from_str("(* #t (fuel ?tank))").is_value(
                TimedFluentExpression::scaled(FluentExpression::function(
                    FunctionHead::with_terms(
                        FunctionSymbol::string("fuel"),
                        [Term::Variable(Variable::string("tank"))]
                    )
                ))
            )
        );
    }
}
