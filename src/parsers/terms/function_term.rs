//! Provides parsers for function terms (`<function-term>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_function_symbol, parse_term};
use crate::types::FunctionTerm;

/// Parses a function term (`<function-term>`), i.e. `(<function-symbol> <term>*)`.
pub(crate) fn parse_function_term(node: &Node) -> R<FunctionTerm> {
    parens(node, |args| {
        let symbol = parse_function_symbol(args.next()?)?;
        let terms = args.list0(parse_term)?;
        Ok(FunctionTerm::new(symbol, terms))
    })
}

crate::parsers::impl_parser!(FunctionTerm => parse_function_term);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        check!(FunctionTerm::from_str("(fun-sym)")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![])));

        let x = Term::Name("x".into());
        check!(FunctionTerm::from_str("(fun-sym x)")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![x])));

        let x = Term::Name("x".into());
        let y = Term::Variable("y".into());
        check!(FunctionTerm::from_str("(fun-sym ?y x)")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![y, x])));

        let x = Term::Name("x".into());
        let y = Term::Variable("y".into());
        let a = Term::Name("a".into());
        let ft = Term::Function(FunctionTerm::new(FunctionSymbol::from("fn"), vec![a]));
        check!(FunctionTerm::from_str("(fun-sym ?y x (fn a))")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![y, x, ft])));
    }

    #[test]
    fn whitespace_before_closing_paren() {
        for input in ["(fun-sym x )", "(fun-sym x\n  )", "(fun-sym x ; c\n)"] {
            let x = Term::Name("x".into());
            check!(FunctionTerm::from_str(input)
                .is_value(FunctionTerm::new("fun-sym".into(), vec![x])));
        }
    }

    #[test]
    fn doc_example_0() {
        check!(FunctionTerm::from_str("(fun-sym)")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![])));

        let x = Term::Name("x".into());
        check!(FunctionTerm::from_str("(fun-sym x)")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![x])));

        let x = Term::Name("x".into());
        let y = Term::Variable("y".into());
        check!(FunctionTerm::from_str("(fun-sym ?y x)")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![y, x])));

        let x = Term::Name("x".into());
        let y = Term::Variable("y".into());
        let a = Term::Name("a".into());
        let ft = Term::Function(FunctionTerm::new(FunctionSymbol::from("fn"), vec![a]));
        check!(FunctionTerm::from_str("(fun-sym ?y x (fn a))")
            .is_value(FunctionTerm::new("fun-sym".into(), vec![y, x, ft])));
    }
}
