//! Loads a [`Problem`] for planning code that `pddl-codegen` generated from a
//! domain.
//!
//! Generated code describes its domain as a [`Signature`]: the types in
//! depth-first order, the predicates and functions with their argument types,
//! and the constants. [`load`] checks a problem against it, numbers the objects
//! and grounds the initial state and the goal. The generated task then copies
//! the facts into its typed state.
//!
//! The generated code calls this module; it is public for that, not as an API
//! of its own, and can change with `pddl-codegen`.
//!
//! ## Numbering
//! Objects are numbered type by type in the order of [`Signature::types`],
//! which lists every type before its subtypes and keeps each subtree together.
//! So the objects of a type and all its subtypes are one range,
//! `starts[t]..ends[t]`, and a type check is a range check.

use std::collections::HashMap;

use pddl_runtime::{Comparison, Expr, Formula};

use crate::{
    AtomicFormula, BinaryComparison, BinaryOp, FluentExpression, FunctionHead, GoalDescription,
    InitElement, Literal, MetricFluentExpression, MultiOp, Name, Optimization,
    PreconditionGoalDescription, PreconditionGoalDescriptions, PreferenceGoalDescription, Problem,
    Term, Type, TypedVariables,
};

/// A domain as generated code sees it. All names are lowercase.
#[derive(Debug)]
pub struct Signature {
    /// The domain's name.
    pub domain: &'static str,
    /// The types, each before its subtypes, with each subtree contiguous.
    /// The first is `object`.
    pub types: &'static [TypeSignature],
    /// The predicates, including derived ones.
    pub predicates: &'static [Symbol],
    /// The numeric functions, without `total-cost`.
    pub functions: &'static [Symbol],
    /// The constants, with the index of their type.
    pub constants: &'static [(&'static str, usize)],
}

/// A type and its place in the hierarchy.
#[derive(Debug)]
pub struct TypeSignature {
    /// The name.
    pub name: &'static str,
    /// The index of the supertype; `None` for `object`.
    pub parent: Option<usize>,
}

/// A predicate or function with the type index of each argument.
#[derive(Debug)]
pub struct Symbol {
    /// The name.
    pub name: &'static str,
    /// The type of each argument; `either` types are `object` (index 0).
    pub args: &'static [usize],
}

/// A problem, checked against a [`Signature`] and grounded.
#[derive(Debug, Clone)]
pub struct Loaded {
    /// The name of each object, by its number.
    pub names: Vec<String>,
    /// For each type, the number of its first object.
    pub starts: Vec<u32>,
    /// For each type, one past the number of its last object, subtypes included.
    pub ends: Vec<u32>,
    /// The number of each constant, in the order of [`Signature::constants`].
    pub constants: Vec<u32>,
    /// The facts of the initial state: a predicate index and its arguments.
    pub facts: Vec<(usize, Vec<u32>)>,
    /// The initial values of numeric fluents: a function index, its arguments
    /// and the value.
    pub values: Vec<(usize, Vec<u32>, f64)>,
    /// The goal.
    pub goal: Formula,
    /// Whether the problem minimizes `total-cost`, so that action costs count.
    pub minimize_total_cost: bool,
}

/// Why a problem doesn't fit a generated domain.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum LoadError {
    /// The problem names another domain.
    #[error("the problem is for domain `{found}`, not `{expected}`")]
    WrongDomain {
        /// The generated domain.
        expected: &'static str,
        /// The domain the problem names.
        found: String,
    },
    /// A type that the domain does not declare.
    #[error("unknown type `{0}`")]
    UnknownType(String),
    /// An object declared with more than one type, or with an `either` type.
    #[error("object `{0}` has more than one type")]
    MultipleTypes(String),
    /// A name that is neither an object nor a constant.
    #[error("unknown object `{0}`")]
    UnknownObject(String),
    /// A predicate or function the domain does not declare.
    #[error("unknown predicate or function `{0}`")]
    UnknownSymbol(String),
    /// A predicate or function with the wrong number of arguments.
    #[error("`{0}` has the wrong number of arguments")]
    WrongArity(String),
    /// An object of the wrong type for an argument.
    #[error("`{object}` is not of the type `{expected}` that `{symbol}` takes")]
    WrongType {
        /// The predicate or function.
        symbol: String,
        /// The object.
        object: String,
        /// The type the argument needs.
        expected: &'static str,
    },
    /// A variable used outside the quantifier that declares it.
    #[error("unknown variable `{0}`")]
    UnknownVariable(String),
    /// A construct the generated code does not support.
    #[error("unsupported in a problem: {0}")]
    Unsupported(&'static str),
    /// A function that action costs read has a value that is not a
    /// non-negative integer.
    #[error("`{function}` is {value}, but action costs must be non-negative integers")]
    InvalidCost {
        /// The function.
        function: String,
        /// Its value in the initial state.
        value: f64,
    },
}

/// Lowercases a name.
fn lc(name: &str) -> String {
    name.to_lowercase()
}

/// Checks `problem` against `signature`, numbers its objects and grounds its
/// initial state and goal.
///
/// # Errors
/// A [`LoadError`] if the problem is for another domain, uses names, types or
/// arities the domain does not declare, or uses an unsupported construct:
/// timed initial literals, object fluents, preferences, constraints, or a
/// metric other than minimizing `total-cost`.
pub fn load(signature: &Signature, problem: &Problem) -> Result<Loaded, LoadError> {
    if lc(problem.domain()) != signature.domain {
        return Err(LoadError::WrongDomain {
            expected: signature.domain,
            found: problem.domain().to_string(),
        });
    }
    if !problem.constraints().is_empty() {
        return Err(LoadError::Unsupported("constraints"));
    }
    let minimize_total_cost = match problem.metric_spec() {
        None => false,
        Some(metric) => match (metric.optimization(), metric.expression()) {
            (Optimization::Minimize, MetricFluentExpression::Function(symbol, args))
                if args.is_empty() && lc(symbol) == "total-cost" =>
            {
                true
            }
            _ => {
                return Err(LoadError::Unsupported(
                    "metrics other than minimizing total-cost",
                ))
            }
        },
    };

    let loader = Loader::new(signature, problem)?;
    let mut facts = Vec::new();
    let mut values = Vec::new();
    for element in problem.init().iter() {
        match element {
            InitElement::Literal(Literal::AtomicFormula(AtomicFormula::Predicate(atom))) => {
                let names: Vec<&Name> = atom.values().iter().collect();
                let (pred, args) = loader.symbol(true, atom.predicate(), &names)?;
                facts.push((pred, args));
            }
            // Closed world: negative initial literals hold anyway.
            InitElement::Literal(Literal::NotAtomicFormula(AtomicFormula::Predicate(_))) => {}
            InitElement::Literal(_) => {
                return Err(LoadError::Unsupported("equality in the initial state"))
            }
            InitElement::At(..) => return Err(LoadError::Unsupported("timed initial literals")),
            InitElement::IsValue(head, value) => {
                if lc(head.symbol()) == "total-cost" && head.names().is_empty() {
                    continue; // costs are not part of the state
                }
                let names: Vec<&Name> = head.names().iter().collect();
                let (func, args) = loader.symbol(false, head.symbol(), &names)?;
                values.push((func, args, **value));
            }
            InitElement::IsObject(..) => return Err(LoadError::Unsupported("object fluents")),
        }
    }
    let goal = loader.goals(problem.goals(), &HashMap::new())?;

    Ok(Loaded {
        names: loader.names,
        starts: loader.starts,
        ends: loader.ends,
        constants: loader.constants,
        facts,
        values,
        goal,
        minimize_total_cost,
    })
}

/// The variables bound by enclosing quantifiers, to object numbers.
type Scope = HashMap<String, u32>;

/// A callback for each binding of quantified variables.
type OnBinding<'f> = dyn FnMut(&Scope) -> Result<(), LoadError> + 'f;

/// The numbered objects, while the problem loads.
struct Loader<'a> {
    /// The domain.
    signature: &'a Signature,
    /// The name of each object, by number.
    names: Vec<String>,
    /// The number of each object, by name.
    numbers: HashMap<String, u32>,
    /// The first object of each type.
    starts: Vec<u32>,
    /// One past the last object of each type, subtypes included.
    ends: Vec<u32>,
    /// The number of each constant.
    constants: Vec<u32>,
}

impl<'a> Loader<'a> {
    /// Numbers the constants and objects type by type.
    fn new(signature: &'a Signature, problem: &Problem) -> Result<Self, LoadError> {
        let type_index = |ty: &Type, object: &str| -> Result<usize, LoadError> {
            let name = match ty {
                Type::Exactly(name) => lc(name),
                Type::EitherOf(names) if names.len() == 1 => lc(&names[0]),
                Type::EitherOf(_) => return Err(LoadError::MultipleTypes(object.to_string())),
            };
            signature
                .types
                .iter()
                .position(|t| t.name == name)
                .ok_or(LoadError::UnknownType(name))
        };

        // Each object's type, in declaration order: constants first.
        let mut declared: Vec<(String, usize)> = signature
            .constants
            .iter()
            .map(|&(name, ty)| (name.to_string(), ty))
            .collect();
        for typed in problem.objects().iter() {
            let name = lc(typed.value());
            let ty = type_index(typed.type_(), &name)?;
            match declared.iter().find(|(n, _)| *n == name) {
                Some(&(_, other)) if other == ty => {} // redeclared constant
                Some(_) => return Err(LoadError::MultipleTypes(name)),
                None => declared.push((name, ty)),
            }
        }

        let count = signature.types.len();
        let mut starts = vec![0; count];
        let mut ends = vec![0; count];
        let mut names = Vec::new();
        for (t, start) in starts.iter_mut().enumerate() {
            *start = u32::try_from(names.len()).expect("fewer than 2^32 objects");
            names.extend(
                declared
                    .iter()
                    .filter(|(_, ty)| *ty == t)
                    .map(|(n, _)| n.clone()),
            );
            ends[t] = u32::try_from(names.len()).expect("fewer than 2^32 objects");
        }
        // A subtree is contiguous, so its end is the largest end in it.
        for t in (0..count).rev() {
            if let Some(parent) = signature.types[t].parent {
                ends[parent] = ends[parent].max(ends[t]);
            }
        }
        let numbers: HashMap<String, u32> = names
            .iter()
            .enumerate()
            .map(|(i, n)| {
                (
                    n.clone(),
                    u32::try_from(i).expect("fewer than 2^32 objects"),
                )
            })
            .collect();
        let constants = signature
            .constants
            .iter()
            .map(|(name, _)| numbers[*name])
            .collect();

        Ok(Self {
            signature,
            names,
            numbers,
            starts,
            ends,
            constants,
        })
    }

    /// The number of an object.
    fn object(&self, name: &str) -> Result<u32, LoadError> {
        let name = lc(name);
        self.numbers
            .get(&name)
            .copied()
            .ok_or(LoadError::UnknownObject(name))
    }

    /// Resolves a predicate (`predicate`) or function and checks its arguments.
    fn symbol(
        &self,
        predicate: bool,
        name: &str,
        args: &[&Name],
    ) -> Result<(usize, Vec<u32>), LoadError> {
        let args = args
            .iter()
            .map(|a| self.object(a))
            .collect::<Result<Vec<_>, _>>()?;
        self.check(predicate, name, args)
    }

    /// Resolves a predicate or function and checks the types of its
    /// numbered arguments.
    fn check(
        &self,
        predicate: bool,
        name: &str,
        args: Vec<u32>,
    ) -> Result<(usize, Vec<u32>), LoadError> {
        let name = lc(name);
        let symbols = if predicate {
            self.signature.predicates
        } else {
            self.signature.functions
        };
        let index = symbols
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| LoadError::UnknownSymbol(name.clone()))?;
        let symbol = &symbols[index];
        if symbol.args.len() != args.len() {
            return Err(LoadError::WrongArity(name));
        }
        for (&arg, &ty) in args.iter().zip(symbol.args) {
            if !(self.starts[ty]..self.ends[ty]).contains(&arg) {
                return Err(LoadError::WrongType {
                    symbol: name,
                    object: self.names[arg as usize].clone(),
                    expected: self.signature.types[ty].name,
                });
            }
        }
        Ok((index, args))
    }

    /// Resolves a term under the quantified variables `scope`.
    fn term(&self, term: &Term, scope: &HashMap<String, u32>) -> Result<u32, LoadError> {
        match term {
            Term::Name(name) => self.object(name),
            Term::Variable(var) => {
                let name = lc(var);
                scope
                    .get(&name)
                    .copied()
                    .ok_or(LoadError::UnknownVariable(name))
            }
            Term::Function(_) => Err(LoadError::Unsupported("object fluents")),
        }
    }

    /// Calls `f` with every binding of `vars` that extends `scope`.
    fn each_binding(
        &self,
        vars: &TypedVariables,
        scope: &HashMap<String, u32>,
        f: &mut OnBinding<'_>,
    ) -> Result<(), LoadError> {
        let mut domains = Vec::new();
        for var in vars.iter() {
            let mut objects = Vec::new();
            for ty in var.type_().clone() {
                let name = lc(&ty);
                let t = self
                    .signature
                    .types
                    .iter()
                    .position(|t| t.name == name)
                    .ok_or(LoadError::UnknownType(name))?;
                objects.extend(self.starts[t]..self.ends[t]);
            }
            objects.sort_unstable();
            objects.dedup();
            domains.push((lc(var.value()), objects));
        }
        bind(&domains, &mut scope.clone(), f)
    }

    /// Grounds a list of goals into a conjunction.
    fn goals(
        &self,
        goals: &PreconditionGoalDescriptions,
        scope: &HashMap<String, u32>,
    ) -> Result<Formula, LoadError> {
        let mut parts = Vec::new();
        for goal in goals {
            match goal {
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(gd)) => {
                    parts.push(self.goal(gd, scope)?);
                }
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Preference(
                    _,
                )) => return Err(LoadError::Unsupported("preferences")),
                PreconditionGoalDescription::Forall(vars, inner) => {
                    self.each_binding(vars, scope, &mut |s| {
                        parts.push(self.goals(inner, s)?);
                        Ok(())
                    })?;
                }
            }
        }
        Ok(flatten_and(parts))
    }

    /// Grounds a goal description.
    fn goal(
        &self,
        gd: &GoalDescription,
        scope: &HashMap<String, u32>,
    ) -> Result<Formula, LoadError> {
        Ok(match gd {
            GoalDescription::AtomicFormula(formula)
            | GoalDescription::Literal(Literal::AtomicFormula(formula)) => {
                self.atom(formula, scope)?
            }
            GoalDescription::Literal(Literal::NotAtomicFormula(formula)) => {
                Formula::Not(Box::new(self.atom(formula, scope)?))
            }
            GoalDescription::Not(inner) => Formula::Not(Box::new(self.goal(inner, scope)?)),
            GoalDescription::And(parts) => flatten_and(
                parts
                    .iter()
                    .map(|p| self.goal(p, scope))
                    .collect::<Result<_, _>>()?,
            ),
            GoalDescription::Or(parts) => Formula::Or(
                parts
                    .iter()
                    .map(|p| self.goal(p, scope))
                    .collect::<Result<_, _>>()?,
            ),
            GoalDescription::Imply(a, b) => Formula::Or(vec![
                Formula::Not(Box::new(self.goal(a, scope)?)),
                self.goal(b, scope)?,
            ]),
            GoalDescription::Exists(vars, inner) => {
                let mut parts = Vec::new();
                self.each_binding(vars, scope, &mut |s| {
                    parts.push(self.goal(inner, s)?);
                    Ok(())
                })?;
                Formula::Or(parts)
            }
            GoalDescription::ForAll(vars, inner) => {
                let mut parts = Vec::new();
                self.each_binding(vars, scope, &mut |s| {
                    parts.push(self.goal(inner, s)?);
                    Ok(())
                })?;
                flatten_and(parts)
            }
            GoalDescription::FluentComparison(comparison) => Formula::Compare(
                match comparison.comparison() {
                    BinaryComparison::LessThan => Comparison::Less,
                    BinaryComparison::LessThanOrEqual => Comparison::LessOrEqual,
                    BinaryComparison::Equal => Comparison::Equal,
                    BinaryComparison::GreaterOrEqual => Comparison::GreaterOrEqual,
                    BinaryComparison::GreaterThan => Comparison::Greater,
                },
                self.expr(comparison.first(), scope)?,
                self.expr(comparison.second(), scope)?,
            ),
        })
    }

    /// Grounds an atom or an equality.
    fn atom(
        &self,
        formula: &AtomicFormula<Term>,
        scope: &HashMap<String, u32>,
    ) -> Result<Formula, LoadError> {
        Ok(match formula {
            AtomicFormula::Equality(eq) => {
                if self.term(eq.first(), scope)? == self.term(eq.second(), scope)? {
                    Formula::True
                } else {
                    Formula::False
                }
            }
            AtomicFormula::Predicate(atom) => {
                let args = atom
                    .values()
                    .iter()
                    .map(|t| self.term(t, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                match self.check(true, atom.predicate(), args) {
                    Ok((pred, args)) => Formula::Atom {
                        pred,
                        args: args.into(),
                    },
                    // A fact on an object of the wrong type never holds.
                    Err(LoadError::WrongType { .. }) => Formula::False,
                    Err(e) => return Err(e),
                }
            }
        })
    }

    /// Grounds a numeric expression.
    fn expr(
        &self,
        expr: &FluentExpression,
        scope: &HashMap<String, u32>,
    ) -> Result<Expr, LoadError> {
        let boxed = |e: &FluentExpression| self.expr(e, scope).map(Box::new);
        Ok(match expr {
            FluentExpression::Number(n) => Expr::Number(**n),
            FluentExpression::Function(head) => {
                let (symbol, terms): (&Name, &[Term]) = match head {
                    FunctionHead::Simple(symbol) => (symbol, &[]),
                    FunctionHead::WithTerms(symbol, terms) => (symbol, terms),
                };
                let args = terms
                    .iter()
                    .map(|t| self.term(t, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                let (func, args) = self.check(false, symbol, args)?;
                Expr::Fluent {
                    func,
                    args: args.into(),
                }
            }
            FluentExpression::Negative(inner) => Expr::Neg(boxed(inner)?),
            FluentExpression::BinaryOp(op, a, b) => {
                let (a, b) = (boxed(a)?, boxed(b)?);
                match op {
                    BinaryOp::Addition => Expr::Add(a, b),
                    BinaryOp::Subtraction => Expr::Sub(a, b),
                    BinaryOp::Multiplication => Expr::Mul(a, b),
                    BinaryOp::Division => Expr::Div(a, b),
                }
            }
            FluentExpression::MultiOp(op, first, rest) => {
                let mut acc = self.expr(first, scope)?;
                for e in rest {
                    let e = boxed(e)?;
                    acc = match op {
                        MultiOp::Addition => Expr::Add(Box::new(acc), e),
                        MultiOp::Multiplication => Expr::Mul(Box::new(acc), e),
                    };
                }
                acc
            }
        })
    }
}

/// Binds the first of the quantified variables to each of its objects, then the rest.
fn bind(
    domains: &[(String, Vec<u32>)],
    scope: &mut HashMap<String, u32>,
    f: &mut OnBinding<'_>,
) -> Result<(), LoadError> {
    let Some(((name, objects), rest)) = domains.split_first() else {
        return f(scope);
    };
    let shadowed = scope.get(name).copied();
    for &object in objects {
        scope.insert(name.clone(), object);
        bind(rest, scope, f)?;
    }
    match shadowed {
        Some(object) => scope.insert(name.clone(), object),
        None => scope.remove(name),
    };
    Ok(())
}

/// A conjunction, with nested conjunctions merged into it.
fn flatten_and(parts: Vec<Formula>) -> Formula {
    let mut flat = Vec::new();
    for part in parts {
        match part {
            Formula::And(inner) => flat.extend(inner),
            other => flat.push(other),
        }
    }
    Formula::And(flat)
}

#[cfg(all(test, feature = "parser"))]
mod tests {
    use super::*;
    use crate::Parser;

    /// `object` > `vehicle` > `truck`, and `place`; a constant `depot`.
    static SIGNATURE: Signature = Signature {
        domain: "d",
        types: &[
            TypeSignature {
                name: "object",
                parent: None,
            },
            TypeSignature {
                name: "vehicle",
                parent: Some(0),
            },
            TypeSignature {
                name: "truck",
                parent: Some(1),
            },
            TypeSignature {
                name: "place",
                parent: Some(0),
            },
        ],
        predicates: &[Symbol {
            name: "at",
            args: &[1, 3],
        }],
        functions: &[Symbol {
            name: "fuel",
            args: &[1],
        }],
        constants: &[("depot", 3)],
    };

    /// Loads a problem given as text.
    fn load_str(text: &str) -> Result<Loaded, LoadError> {
        load(&SIGNATURE, &Problem::from_str(text).unwrap())
    }

    #[test]
    fn numbers_subtypes_inside_their_supertypes() {
        let loaded = load_str(
            "(define (problem p) (:domain d)
               (:objects t1 - truck v1 - vehicle shop - place t2 - truck)
               (:init (at t1 depot) (= (fuel t2) 3) (= (total-cost) 0))
               (:goal (and (at t2 shop) (forall (?v - vehicle) (>= (fuel ?v) 1))))
               (:metric minimize (total-cost)))",
        )
        .unwrap();
        assert_eq!(loaded.names, ["v1", "t1", "t2", "depot", "shop"]);
        assert_eq!(loaded.starts, [0, 0, 1, 3]);
        assert_eq!(loaded.ends, [5, 3, 3, 5]);
        assert_eq!(loaded.constants, [3]);
        assert_eq!(loaded.facts, [(0, vec![1, 3])]);
        assert_eq!(loaded.values, [(0, vec![2], 3.0)]);
        assert!(loaded.minimize_total_cost);
        let Formula::And(goal) = &loaded.goal else {
            panic!("{:?}", loaded.goal)
        };
        assert_eq!(goal.len(), 4, "{goal:?}");
    }

    #[test]
    fn rejects_problems_that_do_not_fit() {
        let problem = |objects: &str, init: &str, goal: &str| {
            load_str(&format!(
                "(define (problem p) (:domain d) (:objects {objects}) (:init {init}) (:goal {goal}))"
            ))
        };
        assert_eq!(
            problem("x - robot", "", "(and)").unwrap_err(),
            LoadError::UnknownType("robot".into())
        );
        assert_eq!(
            problem("x - (either truck place)", "", "(and)").unwrap_err(),
            LoadError::MultipleTypes("x".into())
        );
        assert_eq!(
            problem("depot - truck", "", "(and)").unwrap_err(),
            LoadError::MultipleTypes("depot".into())
        );
        assert_eq!(
            problem("t - truck", "(at t nowhere)", "(and)").unwrap_err(),
            LoadError::UnknownObject("nowhere".into())
        );
        assert_eq!(
            problem("t - truck", "(at depot t)", "(and)").unwrap_err(),
            LoadError::WrongType {
                symbol: "at".into(),
                object: "depot".into(),
                expected: "vehicle"
            }
        );
        assert_eq!(
            problem("t - truck", "(on t)", "(and)").unwrap_err(),
            LoadError::UnknownSymbol("on".into())
        );
        assert_eq!(
            problem("t - truck", "(at t)", "(and)").unwrap_err(),
            LoadError::WrongArity("at".into())
        );
        assert_eq!(
            problem("t - truck", "(at 5 (at t depot))", "(and)").unwrap_err(),
            LoadError::Unsupported("timed initial literals")
        );
        assert_eq!(
            problem("t - truck", "", "(at ?x depot)").unwrap_err(),
            LoadError::UnknownVariable("x".into())
        );
        assert!(matches!(
            load_str("(define (problem p) (:domain other) (:init) (:goal (and)))"),
            Err(LoadError::WrongDomain { .. })
        ));
        assert_eq!(
            load_str("(define (problem p) (:domain d) (:init) (:goal (and)) (:metric maximize (total-cost)))")
                .unwrap_err(),
            LoadError::Unsupported("metrics other than minimizing total-cost")
        );
    }

    #[test]
    fn grounds_goal_connectives() {
        let loaded = load_str(
            "(define (problem p) (:domain d) (:objects t - truck)
               (:init (at t depot))
               (:goal (and (or (at t depot) (not (= t depot)))
                           (imply (at t depot) (exists (?p - place) (at t ?p)))
                           (at depot depot)
                           (< (- (* 2 (fuel t) 1) (/ 4 (fuel t))) (+ 1 (fuel t))))))",
        )
        .unwrap();
        let fact = |pred: usize, args: &[u32]| {
            loaded
                .facts
                .iter()
                .any(|(p, a)| *p == pred && a.as_slice() == args)
        };
        let Formula::And(parts) = &loaded.goal else {
            panic!()
        };
        // The wrongly typed `(at depot depot)` never holds.
        assert_eq!(parts[2], Formula::False);
        assert!(parts[0].holds(&fact, &|_, _| None));
        assert!(parts[1].holds(&fact, &|_, _| None));
        assert!(parts[3].holds(&fact, &|_, _| Some(2.0)));
        assert!(!parts[3].holds(&fact, &|_, _| None));
    }
}
