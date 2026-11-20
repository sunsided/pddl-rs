//! Ground formulas that a task evaluates at run time: [`Formula`] and [`Expr`].
//!
//! Generated code compiles a domain's preconditions and effects, but a
//! problem's goal is only known when the problem loads. The loader grounds its
//! quantifiers over the problem's objects and gives a [`Formula`] over
//! predicate and function indices, which the task evaluates against a state.

/// A ground goal formula over predicate indices and object indices.
#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    /// Always holds.
    True,
    /// Never holds.
    False,
    /// The fact `pred(args)`, with object indices as the loader numbers them.
    Atom {
        /// The predicate's index in the domain's predicate list.
        pred: usize,
        /// The arguments.
        args: Box<[u32]>,
    },
    /// The negation.
    Not(Box<Formula>),
    /// The conjunction; `True` if empty.
    And(Vec<Formula>),
    /// The disjunction; `False` if empty.
    Or(Vec<Formula>),
    /// A numeric comparison.
    Compare(Comparison, Expr, Expr),
}

/// A numeric comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `=`
    Equal,
    /// `>=`
    GreaterOrEqual,
    /// `>`
    Greater,
}

impl Comparison {
    /// Compares two values.
    #[must_use]
    pub fn holds(self, a: f64, b: f64) -> bool {
        match self {
            Self::Less => a < b,
            Self::LessOrEqual => a <= b,
            #[allow(clippy::float_cmp)] // PDDL's `=` is exact
            Self::Equal => a == b,
            Self::GreaterOrEqual => a >= b,
            Self::Greater => a > b,
        }
    }
}

/// A ground numeric expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A constant.
    Number(f64),
    /// The value of the fluent `func(args)`.
    Fluent {
        /// The function's index in the domain's function list.
        func: usize,
        /// The arguments.
        args: Box<[u32]>,
    },
    /// `a + b`
    Add(Box<Expr>, Box<Expr>),
    /// `a - b`
    Sub(Box<Expr>, Box<Expr>),
    /// `a * b`
    Mul(Box<Expr>, Box<Expr>),
    /// `a / b`
    Div(Box<Expr>, Box<Expr>),
    /// `-a`
    Neg(Box<Expr>),
}

impl Expr {
    /// The value, or `None` if a fluent it reads is undefined or it divides
    /// by zero.
    pub fn eval(&self, value: &impl Fn(usize, &[u32]) -> Option<f64>) -> Option<f64> {
        let result = match self {
            Self::Number(n) => *n,
            Self::Fluent { func, args } => value(*func, args)?,
            Self::Add(a, b) => a.eval(value)? + b.eval(value)?,
            Self::Sub(a, b) => a.eval(value)? - b.eval(value)?,
            Self::Mul(a, b) => a.eval(value)? * b.eval(value)?,
            Self::Div(a, b) => {
                let divisor = b.eval(value)?;
                if divisor == 0.0 {
                    return None;
                }
                a.eval(value)? / divisor
            }
            Self::Neg(a) => -a.eval(value)?,
        };
        result.is_finite().then_some(result)
    }
}

impl Formula {
    /// Whether the formula holds, given how to look up facts and fluents.
    ///
    /// A comparison that reads an undefined fluent makes the whole formula
    /// fail, also under a negation: PDDL leaves such a formula undefined, and
    /// an undefined goal is not reached.
    pub fn holds(
        &self,
        fact: &impl Fn(usize, &[u32]) -> bool,
        value: &impl Fn(usize, &[u32]) -> Option<f64>,
    ) -> bool {
        self.eval(fact, value) == Some(true)
    }

    /// The number of conjuncts at the top level that don't hold: a goal-count
    /// heuristic. A formula that is not a conjunction counts as one.
    pub fn unsatisfied(
        &self,
        fact: &impl Fn(usize, &[u32]) -> bool,
        value: &impl Fn(usize, &[u32]) -> Option<f64>,
    ) -> u32 {
        match self {
            Self::And(parts) => parts.iter().map(|part| part.unsatisfied(fact, value)).sum(),
            _ => u32::from(!self.holds(fact, value)),
        }
    }

    /// The truth value, or `None` if a comparison reads an undefined value.
    fn eval(
        &self,
        fact: &impl Fn(usize, &[u32]) -> bool,
        value: &impl Fn(usize, &[u32]) -> Option<f64>,
    ) -> Option<bool> {
        Some(match self {
            Self::True => true,
            Self::False => false,
            Self::Atom { pred, args } => fact(*pred, args),
            Self::Not(inner) => !inner.eval(fact, value)?,
            // No early exit on a decided part: an undefined part anywhere
            // makes the formula undefined, whatever the order of the parts.
            Self::And(parts) => parts
                .iter()
                .try_fold(true, |all, part| Some(part.eval(fact, value)? & all))?,
            Self::Or(parts) => parts
                .iter()
                .try_fold(false, |any, part| Some(part.eval(fact, value)? | any))?,
            Self::Compare(op, a, b) => op.holds(a.eval(value)?, b.eval(value)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `p(0)` holds, `p(1)` doesn't; fluent 0 is 2.0 at `[0]` and undefined elsewhere.
    fn eval(formula: &Formula) -> bool {
        formula.holds(&|_, args| args == [0], &|_, args| {
            (args == [0]).then_some(2.0)
        })
    }

    /// The atom `p(i)`.
    fn p(i: u32) -> Formula {
        Formula::Atom {
            pred: 0,
            args: Box::new([i]),
        }
    }

    /// The fluent `f(i)`.
    fn f(i: u32) -> Expr {
        Expr::Fluent {
            func: 0,
            args: Box::new([i]),
        }
    }

    #[test]
    fn connectives_follow_logic() {
        assert!(eval(&p(0)) && !eval(&p(1)));
        assert!(eval(&Formula::Not(Box::new(p(1)))));
        assert!(eval(&Formula::And(vec![])) && !eval(&Formula::Or(vec![])));
        assert!(!eval(&Formula::And(vec![p(0), p(1)])));
        assert!(eval(&Formula::Or(vec![p(1), p(0)])));
        assert!(eval(&Formula::True) && !eval(&Formula::False));
    }

    #[test]
    fn comparisons_on_undefined_values_fail_even_negated() {
        let ge = |a, b| Formula::Compare(Comparison::GreaterOrEqual, a, b);
        assert!(eval(&ge(f(0), Expr::Number(2.0))));
        assert!(!eval(&ge(f(1), Expr::Number(2.0))));
        assert!(!eval(&Formula::Not(Box::new(ge(f(1), Expr::Number(2.0))))));
        let sum = Expr::Add(
            Box::new(f(0)),
            Box::new(Expr::Neg(Box::new(Expr::Number(1.0)))),
        );
        assert!(eval(&Formula::Compare(
            Comparison::Equal,
            sum,
            Expr::Number(1.0)
        )));
        let quotient = Expr::Div(Box::new(f(0)), Box::new(Expr::Number(0.0)));
        assert!(!eval(&Formula::Compare(
            Comparison::Less,
            quotient,
            Expr::Number(1.0)
        )));
        let product = Expr::Mul(
            Box::new(f(0)),
            Box::new(Expr::Sub(Box::new(f(0)), Box::new(Expr::Number(1.0)))),
        );
        assert!(eval(&Formula::Compare(
            Comparison::Greater,
            product,
            Expr::Number(1.0)
        )));
        assert!(Comparison::Less.holds(1.0, 2.0) && Comparison::LessOrEqual.holds(2.0, 2.0));
    }

    #[test]
    fn undefined_values_fail_a_formula_in_any_order() {
        let undefined = || Formula::Compare(Comparison::Less, f(1), Expr::Number(0.0));
        let not = |inner| Formula::Not(Box::new(inner));
        for parts in [vec![p(1), undefined()], vec![undefined(), p(1)]] {
            assert!(!eval(&not(Formula::And(parts.clone()))), "{parts:?}");
        }
        for parts in [vec![p(0), undefined()], vec![undefined(), p(0)]] {
            assert!(!eval(&Formula::Or(parts.clone())), "{parts:?}");
        }
    }

    #[test]
    fn unsatisfied_counts_top_level_conjuncts() {
        let goal = Formula::And(vec![p(0), p(1), p(2)]);
        let fact = |_: usize, args: &[u32]| args == [0];
        assert_eq!(goal.unsatisfied(&fact, &|_, _| None), 2);
        assert_eq!(p(1).unsatisfied(&fact, &|_, _| None), 1);
    }
}
