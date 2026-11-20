//! The static check of every formula of the domain and problem, before any plan runs.
//!
//! It finds undeclared names, wrong arities and unbound variables, so evaluation never
//! meets them, and it finds every construct the validator does not interpret, so no plan
//! that uses one is ever accepted. Formulas inside an unsupported construct are not
//! checked further. [`durative`] checks durative actions.

mod durative;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use pddl::{
    AtomicFormula, ConditionalEffect, Domain, EffectCondition, Effects, FluentExpression,
    FunctionHead, GoalDescription, InitElement, Literal, MetricFluentExpression,
    PreconditionGoalDescription, PreconditionGoalDescriptions, PreferenceGoalDescription,
    PrimitiveEffect, Problem, Term, TypedVariables,
};

use super::Model;
use crate::atom::lower;
use crate::error::{ModelError, ModelErrorKind};
use crate::eval::numeric::{numeric_assignment, numeric_equality};
use crate::report::{Construct, Unsupported};

/// What the static check found.
#[derive(Debug)]
pub enum Issue {
    /// The domain or problem is malformed.
    Model(ModelError),
    /// The domain or problem uses a construct the validator does not interpret.
    Unsupported(Unsupported),
}

/// Checks every action, derived-predicate rule, initial fact and goal.
///
/// # Errors
///
/// The first [`Issue`] found, in the order actions, rules, initial state, goal, metric.
pub fn check(model: &Model<'_>, problem: &Problem) -> Result<(), Issue> {
    for (name, action) in &model.actions {
        let mut checker = Checker::new(model, format!("action `{name}`"));
        checker.bind(action.parameters())?;
        checker.preconditions(action.precondition())?;
        if let Some(effects) = action.effect() {
            checker.effects(effects)?;
        }
    }

    for (name, action) in &model.durative {
        let mut checker = Checker::new(model, format!("durative action `{name}`"));
        checker.durative(action)?;
    }

    for rule in &model.rules {
        let mut checker = Checker::new(model, format!("derived predicate `{}`", rule.predicate));
        checker.bind(rule.parameters)?;
        checker.goal(rule.body)?;
    }

    let checker = Checker::new(model, String::from("init"));
    for element in problem.init().iter() {
        checker.init(element)?;
    }

    let mut checker = Checker::new(model, String::from("goal"));
    checker.preconditions(problem.goals())?;
    if let Some(metric) = problem.metric_spec() {
        Checker::new(model, String::from("metric")).metric(metric.expression())?;
    }

    Ok(())
}

/// Checks that `domain` declares only number-valued functions.
///
/// # Errors
///
/// [`Construct::ObjectFluents`] for a function of an object type.
pub fn check_function_types(domain: &Domain) -> Result<(), Issue> {
    for function in domain.functions().iter() {
        let names = super::universe::primitive_names(function.type_ref());
        if names.iter().any(|name| name != "number") {
            let location = format!("function `{}`", lower(function.symbol()));
            let unsupported = Unsupported::new(Construct::ObjectFluents, location);
            return Err(Issue::Unsupported(unsupported));
        }
    }

    Ok(())
}

/// Walks the formulas of one action, rule or problem section.
struct Checker<'m, 'a> {
    /// The indexed domain and problem.
    model: &'m Model<'a>,
    /// Where the walked formulas are, for messages.
    location: String,
    /// The variables in scope, innermost last, lowercased.
    variables: Vec<String>,
}

impl<'m, 'a> Checker<'m, 'a> {
    /// Starts a walk at `location` with no variables in scope.
    const fn new(model: &'m Model<'a>, location: String) -> Self {
        Self {
            model,
            location,
            variables: Vec::new(),
        }
    }

    /// Builds a model error at the current location.
    fn error(&self, kind: ModelErrorKind) -> Issue {
        Issue::Model(ModelError::new(self.location.clone(), kind))
    }

    /// Builds an unsupported finding at the current location.
    fn unsupported(&self, construct: Construct) -> Issue {
        Issue::Unsupported(Unsupported::new(construct, self.location.clone()))
    }

    /// Brings `variables` into scope and checks their types. Returns how many were added,
    /// for [`Self::unbind`].
    fn bind(&mut self, variables: &TypedVariables) -> Result<usize, Issue> {
        for variable in variables.iter() {
            if let Some(name) = self.model.universe.unknown_type(variable.type_()) {
                return Err(self.error(ModelErrorKind::UnknownType(name)));
            }

            self.variables.push(lower(variable.value()));
        }

        Ok(variables.len())
    }

    /// Takes the last `count` variables out of scope.
    fn unbind(&mut self, count: usize) {
        let keep = self.variables.len().saturating_sub(count);
        self.variables.truncate(keep);
    }

    /// Checks a precondition or goal list.
    fn preconditions(&mut self, list: &PreconditionGoalDescriptions) -> Result<(), Issue> {
        for item in list {
            match item {
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(gd)) => {
                    self.goal(gd)?;
                }
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Preference(
                    _,
                )) => return Err(self.unsupported(Construct::Preferences)),
                PreconditionGoalDescription::Forall(variables, inner) => {
                    let count = self.bind(variables)?;
                    self.preconditions(inner)?;
                    self.unbind(count);
                }
            }
        }

        Ok(())
    }

    /// Checks a goal description.
    fn goal(&mut self, gd: &GoalDescription) -> Result<(), Issue> {
        match gd {
            GoalDescription::AtomicFormula(formula)
            | GoalDescription::Literal(
                Literal::AtomicFormula(formula) | Literal::NotAtomicFormula(formula),
            ) => self.atomic(formula),
            GoalDescription::And(items) | GoalDescription::Or(items) => {
                items.iter().try_for_each(|item| self.goal(item))
            }
            GoalDescription::Not(inner) => self.goal(inner),
            GoalDescription::Imply(premise, conclusion) => {
                self.goal(premise)?;
                self.goal(conclusion)
            }
            GoalDescription::Exists(variables, inner)
            | GoalDescription::ForAll(variables, inner) => {
                let count = self.bind(variables)?;
                self.goal(inner)?;
                self.unbind(count);
                Ok(())
            }
            GoalDescription::FluentComparison(comparison) => {
                self.expression(comparison.first())?;
                self.expression(comparison.second())
            }
        }
    }

    /// Checks an atomic formula: a declared predicate with the right arity, or an equality.
    fn atomic(&self, formula: &AtomicFormula<Term>) -> Result<(), Issue> {
        match formula {
            AtomicFormula::Equality(equality) => {
                if let Some(comparison) = numeric_equality(equality) {
                    self.expression(comparison.first())?;
                    return self.expression(comparison.second());
                }

                self.term(equality.first())?;
                self.term(equality.second())
            }
            AtomicFormula::Predicate(atom) => {
                let found = atom.values().len();
                let unknown = ModelErrorKind::UnknownPredicate;
                self.symbol(&self.model.predicates, atom.predicate(), found, unknown)?;
                atom.values().iter().try_for_each(|term| self.term(term))
            }
        }
    }

    /// Checks that `symbol` is declared in `table` with `found` parameters; `unknown`
    /// builds the error for a symbol that is not.
    fn symbol(
        &self,
        table: &alloc::collections::BTreeMap<String, usize>,
        symbol: &str,
        found: usize,
        unknown: fn(String) -> ModelErrorKind,
    ) -> Result<(), Issue> {
        let symbol = lower(symbol);
        let Some(&expected) = table.get(&symbol) else {
            return Err(self.error(unknown(symbol)));
        };

        if expected == found {
            Ok(())
        } else {
            Err(self.error(ModelErrorKind::Arity {
                symbol,
                expected,
                found,
            }))
        }
    }

    /// Checks that a term is a declared object or a variable in scope.
    fn term(&self, term: &Term) -> Result<(), Issue> {
        match term {
            Term::Name(name) => self.object(name),
            Term::Variable(variable) => {
                let name = lower(variable);
                if self.variables.contains(&name) {
                    Ok(())
                } else {
                    Err(self.error(ModelErrorKind::UnboundVariable(name)))
                }
            }
            Term::Function(_) => Err(self.unsupported(Construct::ObjectFluents)),
        }
    }

    /// Checks that `name` is a declared object or constant.
    fn object(&self, name: &str) -> Result<(), Issue> {
        let name = lower(name);
        if self.model.universe.contains(&name) {
            Ok(())
        } else {
            Err(self.error(ModelErrorKind::UnknownObject(name)))
        }
    }

    /// Checks an effect list.
    fn effects(&mut self, effects: &Effects) -> Result<(), Issue> {
        for effect in effects {
            match effect {
                ConditionalEffect::Effect(primitive) => self.primitive(primitive)?,
                ConditionalEffect::Forall(forall) => {
                    let count = self.bind(&forall.variables)?;
                    self.effects(&forall.effects)?;
                    self.unbind(count);
                }
                ConditionalEffect::When(when) => {
                    self.goal(&when.condition)?;
                    match &when.effect {
                        EffectCondition::Single(primitive) => self.primitive(primitive)?,
                        EffectCondition::All(primitives) => {
                            primitives.iter().try_for_each(|p| self.primitive(p))?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Checks a primitive effect.
    fn primitive(&self, effect: &PrimitiveEffect) -> Result<(), Issue> {
        match effect {
            PrimitiveEffect::AtomicFormula(formula)
            | PrimitiveEffect::NotAtomicFormula(formula) => self.settable(formula),
            PrimitiveEffect::AssignNumericFluent(_, head, value) => {
                self.function(head)?;
                self.expression(value)
            }
            PrimitiveEffect::AssignObjectFluent(..) => match numeric_assignment(effect) {
                Some((head, value)) => {
                    self.function(&head)?;
                    self.expression(&value)
                }
                None => Err(self.unsupported(Construct::ObjectFluents)),
            },
        }
    }

    /// Checks an atom that an effect or the initial state sets: a declared, non-derived
    /// predicate, never an equality.
    fn settable(&self, formula: &AtomicFormula<Term>) -> Result<(), Issue> {
        match formula {
            AtomicFormula::Equality(_) => Err(self.error(ModelErrorKind::EqualitySet)),
            AtomicFormula::Predicate(atom)
                if self.model.derived.contains(&lower(atom.predicate())) =>
            {
                Err(self.error(ModelErrorKind::DerivedPredicateSet(lower(atom.predicate()))))
            }
            AtomicFormula::Predicate(_) => self.atomic(formula),
        }
    }

    /// Checks a numeric expression.
    fn expression(&self, exp: &FluentExpression) -> Result<(), Issue> {
        match exp {
            FluentExpression::Number(_) => Ok(()),
            FluentExpression::Function(head) => self.function(head),
            FluentExpression::Negative(inner) => self.expression(inner),
            FluentExpression::BinaryOp(_, left, right) => {
                self.expression(left)?;
                self.expression(right)
            }
            FluentExpression::MultiOp(_, first, rest) => {
                self.expression(first)?;
                rest.iter().try_for_each(|item| self.expression(item))
            }
        }
    }

    /// Checks a function application: a declared function with the right arity.
    fn function(&self, head: &FunctionHead) -> Result<(), Issue> {
        let unknown = ModelErrorKind::UnknownFunction;
        match head {
            FunctionHead::Simple(symbol) => self.symbol(&self.model.functions, symbol, 0, unknown),
            FunctionHead::WithTerms(symbol, terms) => {
                self.symbol(&self.model.functions, symbol, terms.len(), unknown)?;
                terms.iter().try_for_each(|term| self.term(term))
            }
        }
    }

    /// Checks a metric expression.
    fn metric(&self, exp: &MetricFluentExpression) -> Result<(), Issue> {
        match exp {
            MetricFluentExpression::Number(_) | MetricFluentExpression::TotalTime => Ok(()),
            MetricFluentExpression::Function(symbol, names)
                if self.model.is_total_time(symbol, names.len()) =>
            {
                Ok(())
            }
            MetricFluentExpression::Function(symbol, names) => {
                let unknown = ModelErrorKind::UnknownFunction;
                self.symbol(&self.model.functions, symbol, names.len(), unknown)?;
                names.iter().try_for_each(|name| self.object(name))
            }
            MetricFluentExpression::Negative(inner) => self.metric(inner),
            MetricFluentExpression::BinaryOp(_, left, right) => {
                self.metric(left)?;
                self.metric(right)
            }
            MetricFluentExpression::MultiOp(_, first, rest) => {
                self.metric(first)?;
                rest.iter().try_for_each(|item| self.metric(item))
            }
            MetricFluentExpression::IsViolated(_) => Err(self.unsupported(Construct::Preferences)),
        }
    }

    /// Checks an element of the initial state.
    fn init(&self, element: &InitElement) -> Result<(), Issue> {
        match element {
            InitElement::Literal(
                Literal::AtomicFormula(formula) | Literal::NotAtomicFormula(formula),
            )
            | InitElement::At(
                _,
                Literal::AtomicFormula(formula) | Literal::NotAtomicFormula(formula),
            ) => self.settable(&name_formula(formula)),
            InitElement::IsValue(term, _) => {
                let found = term.names().len();
                let unknown = ModelErrorKind::UnknownFunction;
                self.symbol(&self.model.functions, term.symbol(), found, unknown)?;
                term.names().iter().try_for_each(|name| self.object(name))
            }
            InitElement::IsObject(..) => Err(self.unsupported(Construct::ObjectFluents)),
        }
    }
}

/// Converts an atomic formula over names, as the initial state has, to one over terms.
fn name_formula(formula: &AtomicFormula<pddl::Name>) -> AtomicFormula<Term> {
    match formula {
        AtomicFormula::Equality(equality) => AtomicFormula::new_equality(
            Term::Name(equality.first().clone()),
            Term::Name(equality.second().clone()),
        ),
        AtomicFormula::Predicate(atom) => AtomicFormula::predicate(
            atom.predicate().clone(),
            atom.values().iter().cloned().map(Term::Name),
        ),
    }
}
