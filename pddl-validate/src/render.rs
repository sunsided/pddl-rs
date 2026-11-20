//! Renders formulas as PDDL text, with bound variables replaced by their objects.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use pddl::{
    AtomicFormula, BinaryComparison, BinaryOp, FluentComparison, FluentExpression, FunctionHead,
    GoalDescription, Literal, MultiOp, Term, TypedVariables,
};

use crate::atom::lower;
use crate::eval::Scope;
use crate::model::universe::type_text;

/// Renders a goal description, ground as far as `scope` binds its variables.
pub fn goal(gd: &GoalDescription, scope: &Scope) -> String {
    match gd {
        GoalDescription::AtomicFormula(formula)
        | GoalDescription::Literal(Literal::AtomicFormula(formula)) => atomic(formula, scope),
        GoalDescription::Literal(Literal::NotAtomicFormula(formula)) => {
            format!("(not {})", atomic(formula, scope))
        }
        GoalDescription::And(items) => list("and", items, scope),
        GoalDescription::Or(items) => list("or", items, scope),
        GoalDescription::Not(inner) => format!("(not {})", goal(inner, scope)),
        GoalDescription::Imply(premise, conclusion) => format!(
            "(imply {} {})",
            goal(premise, scope),
            goal(conclusion, scope)
        ),
        GoalDescription::Exists(variables, inner) => quantified("exists", variables, inner, scope),
        GoalDescription::ForAll(variables, inner) => quantified("forall", variables, inner, scope),
        GoalDescription::FluentComparison(comparison) => self::comparison(comparison, scope),
    }
}

/// Renders a numeric comparison.
pub fn comparison(comparison: &FluentComparison, scope: &Scope) -> String {
    format!(
        "({} {} {})",
        comparison_text(*comparison.comparison()),
        expression(comparison.first(), scope),
        expression(comparison.second(), scope)
    )
}

/// Renders an atomic formula.
pub fn atomic(formula: &AtomicFormula<Term>, scope: &Scope) -> String {
    match formula {
        AtomicFormula::Equality(equality) => format!(
            "(= {} {})",
            term(equality.first(), scope),
            term(equality.second(), scope)
        ),
        AtomicFormula::Predicate(atom) => application(atom.predicate(), atom.values(), scope),
    }
}

/// Renders a numeric expression.
pub fn expression(exp: &FluentExpression, scope: &Scope) -> String {
    match exp {
        FluentExpression::Number(number) => format!("{}", **number),
        FluentExpression::Function(head) => function(head, scope),
        FluentExpression::Negative(inner) => format!("(- {})", expression(inner, scope)),
        FluentExpression::BinaryOp(op, left, right) => format!(
            "({} {} {})",
            binary_text(*op),
            expression(left, scope),
            expression(right, scope)
        ),
        FluentExpression::MultiOp(op, first, rest) => {
            let mut parts = alloc::vec![expression(first, scope)];
            parts.extend(rest.iter().map(|e| expression(e, scope)));
            let op = match op {
                MultiOp::Addition => "+",
                MultiOp::Multiplication => "*",
            };
            format!("({op} {})", parts.join(" "))
        }
    }
}

/// Renders a function application such as `(fuel t1)`.
pub fn function(head: &FunctionHead, scope: &Scope) -> String {
    match head {
        FunctionHead::Simple(symbol) => format!("({})", lower(symbol)),
        FunctionHead::WithTerms(symbol, terms) => application(symbol, terms, scope),
    }
}

/// Renders a term: an object, a bound variable's object, or an unbound `?variable`.
pub fn term(term: &Term, scope: &Scope) -> String {
    match term {
        Term::Name(name) => lower(name),
        Term::Variable(variable) => {
            let name = lower(variable);
            crate::eval::lookup(scope, &name).map_or_else(|| format!("?{name}"), String::from)
        }
        Term::Function(function) => application(function.symbol(), function.terms(), scope),
    }
}

/// Renders `(symbol term...)`.
fn application(symbol: &str, terms: &[Term], scope: &Scope) -> String {
    let mut text = format!("({}", lower(symbol));
    for argument in terms {
        text.push(' ');
        text.push_str(&self::term(argument, scope));
    }

    text.push(')');
    text
}

/// Renders `(keyword item...)`.
fn list(keyword: &str, items: &[GoalDescription], scope: &Scope) -> String {
    let parts: Vec<String> = items.iter().map(|item| goal(item, scope)).collect();
    if parts.is_empty() {
        format!("({keyword})")
    } else {
        format!("({keyword} {})", parts.join(" "))
    }
}

/// Renders a quantified formula. Its own variables stay unbound in the body.
fn quantified(
    keyword: &str,
    variables: &TypedVariables,
    inner: &GoalDescription,
    scope: &Scope,
) -> String {
    let own: Vec<String> = variables
        .iter()
        .map(|v| format!("?{} - {}", lower(v.value()), type_text(v.type_())))
        .collect();
    let shadowed: Scope = scope
        .iter()
        .filter(|(name, _)| !variables.iter().any(|v| lower(v.value()) == *name))
        .cloned()
        .collect();

    format!("({keyword} ({}) {})", own.join(" "), goal(inner, &shadowed))
}

/// Returns the PDDL keyword of a comparison.
pub const fn comparison_text(comparison: BinaryComparison) -> &'static str {
    match comparison {
        BinaryComparison::GreaterThan => ">",
        BinaryComparison::LessThan => "<",
        BinaryComparison::Equal => "=",
        BinaryComparison::GreaterOrEqual => ">=",
        BinaryComparison::LessThanOrEqual => "<=",
    }
}

/// Returns the PDDL keyword of a binary operator.
pub const fn binary_text(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Multiplication => "*",
        BinaryOp::Addition => "+",
        BinaryOp::Subtraction => "-",
        BinaryOp::Division => "/",
    }
}

#[cfg(test)]
mod tests {
    //! Pins the PDDL text of every goal and expression form.

    use alloc::string::ToString;
    use alloc::vec;

    use pddl::Parser;

    use super::*;

    /// Parses `text` as a goal and renders it with `?t` bound to `t1`.
    #[expect(
        clippy::expect_used,
        reason = "a test fails loudly on a broken fixture"
    )]
    fn render(text: &str) -> String {
        let gd = GoalDescription::from_str(text).expect("the goal parses");
        goal(&gd, &vec![("t".to_string(), "t1".to_string())])
    }

    #[test]
    fn goals_render_with_bound_variables_replaced() {
        let cases = [
            ("(at ?t ?place)", "(at t1 ?place)"),
            ("(not (at ?t home))", "(not (at t1 home))"),
            ("(= ?t t2)", "(= t1 t2)"),
            ("(and)", "(and)"),
            ("(or (p) (not (and)))", "(or (p) (not (and)))"),
            ("(imply (p) (q ?t))", "(imply (p) (q t1))"),
            (
                "(forall (?t - truck) (at ?t home))",
                "(forall (?t - truck) (at ?t home))",
            ),
            (
                "(exists (?x - place) (at ?t ?x))",
                "(exists (?x - place) (at t1 ?x))",
            ),
            ("(p (f ?t))", "(p (f t1))"),
        ];

        for (text, rendered) in cases {
            assert_eq!(render(text), rendered, "{text}");
        }
    }

    #[test]
    #[expect(
        clippy::expect_used,
        clippy::panic,
        reason = "a test fails loudly on a broken fixture"
    )]
    fn negated_literals_render_as_not() {
        let GoalDescription::AtomicFormula(atom) =
            GoalDescription::from_str("(at ?t home)").expect("the goal parses")
        else {
            panic!("an atomic formula");
        };
        let gd = GoalDescription::Literal(Literal::new_not(atom));

        assert_eq!(goal(&gd, &vec![]), "(not (at ?t home))");
    }

    #[test]
    fn expressions_render_every_operator() {
        let cases = [
            ("(> (fuel ?t) 0.5)", "(> (fuel t1) 0.5)"),
            ("(< (- (fuel)) 1)", "(< (- (fuel)) 1)"),
            ("(= (* (fuel) 2) 1)", "(= (* (fuel) 2) 1)"),
            ("(>= (+ (fuel) 2) 1)", "(>= (+ (fuel) 2) 1)"),
            ("(<= (- (fuel) 2) 1)", "(<= (- (fuel) 2) 1)"),
            ("(= (/ (fuel) 2) 1)", "(= (/ (fuel) 2) 1)"),
            ("(= (+ (fuel) 2 3) 1)", "(= (+ (fuel) 2 3) 1)"),
            ("(= (* (fuel) 2 3) 1)", "(= (* (fuel) 2 3) 1)"),
        ];

        for (text, rendered) in cases {
            assert_eq!(render(text), rendered, "{text}");
        }
    }
}
