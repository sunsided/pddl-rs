//! Contains the constraint goal description types [`ConstraintGoalDescription`] and [`ConstraintGoalDescriptionInner`].

use crate::types::{GoalDescription, Number, TypedVariables};

/// A constraint goal description.
///
/// Represents temporal constraint goals for problem-level constraints in PDDL 3.1.
/// Supports `always`, `sometime`, `within`, `at-most-once`, `sometime-after`,
/// `sometime-before`, `always-within`, `hold-during`, and `hold-after`.
///
/// # BNF
/// Corresponds to `<con-GD>` in the PDDL 3.1 specification.
///
/// ## Usage
/// Used by [`PreferenceConstraintGoalDescription`](crate::types::PreferenceConstraintGoalDescription) and [`ConstraintGoalDescriptionInner`].
#[doc(alias = "con-GD")]
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintGoalDescription {
    /// A conjunction, `(and <con-GD>*)`.
    And(Vec<ConstraintGoalDescription>),
    /// A universal quantification, `(forall (<typed list(variable)>) <con-GD>)`.
    Forall(TypedVariables, Box<ConstraintGoalDescription>),
    /// A goal that must hold in the final state, `(at end <GD>)`.
    AtEnd(GoalDescription),
    /// A goal that must hold in every state, `(always <con2-GD>)`.
    Always(ConstraintGoalDescriptionInner),
    /// A goal that must hold in at least one state, `(sometime <con2-GD>)`.
    Sometime(ConstraintGoalDescriptionInner),
    /// A goal that must become true within the given time, `(within <number> <con2-GD>)`.
    Within(Number, ConstraintGoalDescriptionInner),
    /// A goal that may become true at most once, `(at-most-once <con2-GD>)`.
    AtMostOnce(ConstraintGoalDescriptionInner),
    /// Whenever the first goal holds, the second must hold then or later,
    /// `(sometime-after <con2-GD> <con2-GD>)`.
    SometimeAfter(
        ConstraintGoalDescriptionInner,
        ConstraintGoalDescriptionInner,
    ),
    /// Whenever the first goal holds, the second must have held earlier,
    /// `(sometime-before <con2-GD> <con2-GD>)`.
    SometimeBefore(
        ConstraintGoalDescriptionInner,
        ConstraintGoalDescriptionInner,
    ),
    /// Whenever the first goal holds, the second must hold within the given time,
    /// `(always-within <number> <con2-GD> <con2-GD>)`.
    AlwaysWithin(
        Number,
        ConstraintGoalDescriptionInner,
        ConstraintGoalDescriptionInner,
    ),
    /// A goal that must hold between the two given times,
    /// `(hold-during <number> <number> <con2-GD>)`.
    HoldDuring(Number, Number, ConstraintGoalDescriptionInner),
    /// A goal that must hold in at least one state after the given time,
    /// `(hold-after <number> <con2-GD>)`.
    HoldAfter(Number, ConstraintGoalDescriptionInner),
}

/// Alias for [`ConstraintGoalDescription`]; matches BNF `<con-GD>`.
#[deprecated(since = "0.2.0", note = "Use `ConstraintGoalDescription` instead")]
pub type ConGD = ConstraintGoalDescription;

/// A type that represents either a [`GoalDescription`] or an embedded [`ConstraintGoalDescription`].
///
/// # BNF
/// Corresponds to `<con2-GD>` in the PDDL 3.1 specification.
///
/// ## Usage
/// Used by [`ConstraintGoalDescription`].
#[doc(alias = "con2-GD")]
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintGoalDescriptionInner {
    /// A plain goal description, `<GD>`.
    Goal(GoalDescription),
    /// A nested constraint goal description, `<con-GD>`.
    Nested(Box<ConstraintGoalDescription>),
}

/// Alias for [`ConstraintGoalDescriptionInner`]; matches BNF `<con2-GD>`.
#[deprecated(since = "0.2.0", note = "Use `ConstraintGoalDescriptionInner` instead")]
pub type Con2GD = ConstraintGoalDescriptionInner;

impl Default for ConstraintGoalDescription {
    fn default() -> Self {
        Self::And(Vec::default())
    }
}

impl ConstraintGoalDescription {
    /// Creates a conjunction `(and <con-GD>*)`.
    #[doc(alias = "new_and")]
    pub fn and<G: IntoIterator<Item = ConstraintGoalDescription>>(goals: G) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(goals.into_iter().collect())
    }

    /// Alias for [`Self::and`].
    pub fn new_and<G: IntoIterator<Item = ConstraintGoalDescription>>(goals: G) -> Self {
        Self::and(goals)
    }

    /// Creates a universal quantification `(forall (<variables>) <con-GD>)`.
    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: ConstraintGoalDescription) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    /// Alias for [`Self::forall`].
    pub fn new_forall(variables: TypedVariables, gd: ConstraintGoalDescription) -> Self {
        Self::r#forall(variables, gd)
    }

    /// Creates an `(at end <GD>)` constraint.
    #[doc(alias = "new_at_end")]
    pub const fn at_end(gd: GoalDescription) -> Self {
        Self::AtEnd(gd)
    }

    /// Alias for [`Self::at_end`].
    pub fn new_at_end(gd: GoalDescription) -> Self {
        Self::at_end(gd)
    }

    /// Creates an `(always <con2-GD>)` constraint.
    #[doc(alias = "new_always")]
    pub const fn always(gd: ConstraintGoalDescriptionInner) -> Self {
        Self::Always(gd)
    }

    /// Alias for [`Self::always`].
    pub fn new_always(gd: ConstraintGoalDescriptionInner) -> Self {
        Self::always(gd)
    }

    /// Creates a `(sometime <con2-GD>)` constraint.
    #[doc(alias = "new_sometime")]
    pub const fn sometime(gd: ConstraintGoalDescriptionInner) -> Self {
        Self::Sometime(gd)
    }

    /// Alias for [`Self::sometime`].
    pub fn new_sometime(gd: ConstraintGoalDescriptionInner) -> Self {
        Self::sometime(gd)
    }

    /// Creates a `(within <number> <con2-GD>)` constraint.
    #[doc(alias = "new_within")]
    pub const fn within(number: Number, gd: ConstraintGoalDescriptionInner) -> Self {
        Self::Within(number, gd)
    }

    /// Alias for [`Self::within`].
    pub fn new_within(number: Number, gd: ConstraintGoalDescriptionInner) -> Self {
        Self::within(number, gd)
    }

    /// Creates an `(at-most-once <con2-GD>)` constraint.
    #[doc(alias = "new_at_most_once")]
    pub const fn at_most_once(gd: ConstraintGoalDescriptionInner) -> Self {
        Self::AtMostOnce(gd)
    }

    /// Alias for [`Self::at_most_once`].
    pub fn new_at_most_once(gd: ConstraintGoalDescriptionInner) -> Self {
        Self::at_most_once(gd)
    }

    /// Creates a `(sometime-after <first> <then>)` constraint: whenever `first`
    /// holds, `then` must hold at that point or later.
    #[doc(alias = "new_sometime_after")]
    pub const fn sometime_after(
        first: ConstraintGoalDescriptionInner,
        then: ConstraintGoalDescriptionInner,
    ) -> Self {
        Self::SometimeAfter(first, then)
    }

    /// Alias for [`Self::sometime_after`].
    pub fn new_sometime_after(
        first: ConstraintGoalDescriptionInner,
        then: ConstraintGoalDescriptionInner,
    ) -> Self {
        Self::sometime_after(first, then)
    }

    /// Creates a `(sometime-before <later> <earlier>)` constraint: whenever `later`
    /// holds, `earlier` must have held before.
    #[doc(alias = "new_sometime_before")]
    pub const fn sometime_before(
        later: ConstraintGoalDescriptionInner,
        earlier: ConstraintGoalDescriptionInner,
    ) -> Self {
        Self::SometimeBefore(later, earlier)
    }

    /// Alias for [`Self::sometime_before`].
    pub fn new_sometime_before(
        later: ConstraintGoalDescriptionInner,
        earlier: ConstraintGoalDescriptionInner,
    ) -> Self {
        Self::sometime_before(later, earlier)
    }

    /// Creates an `(always-within <number> <first> <second>)` constraint: whenever
    /// `first` holds, `second` must hold within `number` time units.
    #[doc(alias = "new_always_within")]
    pub const fn always_within(
        number: Number,
        first: ConstraintGoalDescriptionInner,
        second: ConstraintGoalDescriptionInner,
    ) -> Self {
        Self::AlwaysWithin(number, first, second)
    }

    /// Alias for [`Self::always_within`].
    pub fn new_always_within(
        number: Number,
        first: ConstraintGoalDescriptionInner,
        second: ConstraintGoalDescriptionInner,
    ) -> Self {
        Self::always_within(number, first, second)
    }

    /// Creates a `(hold-during <begin> <end> <con2-GD>)` constraint.
    #[doc(alias = "new_hold_during")]
    pub const fn hold_during(
        begin: Number,
        end: Number,
        gd: ConstraintGoalDescriptionInner,
    ) -> Self {
        Self::HoldDuring(begin, end, gd)
    }

    /// Alias for [`Self::hold_during`].
    pub fn new_hold_during(begin: Number, end: Number, gd: ConstraintGoalDescriptionInner) -> Self {
        Self::hold_during(begin, end, gd)
    }

    /// Creates a `(hold-after <number> <con2-GD>)` constraint: the goal must hold in at
    /// least one state after `number`.
    #[doc(alias = "new_hold_after")]
    pub const fn hold_after(number: Number, gd: ConstraintGoalDescriptionInner) -> Self {
        Self::HoldAfter(number, gd)
    }

    /// Alias for [`Self::hold_after`].
    pub fn new_hold_after(number: Number, gd: ConstraintGoalDescriptionInner) -> Self {
        Self::hold_after(number, gd)
    }

    /// Returns `true` if this constraint contains no goals, e.g. `(and)`.
    pub fn is_empty(&self) -> bool {
        match self {
            ConstraintGoalDescription::And(x) => x.iter().all(|y| y.is_empty()),
            ConstraintGoalDescription::Forall(_, x) => x.is_empty(),
            ConstraintGoalDescription::AtEnd(x) => x.is_empty(),
            ConstraintGoalDescription::Always(x) => x.is_empty(),
            ConstraintGoalDescription::Sometime(x) => x.is_empty(),
            ConstraintGoalDescription::Within(_, x) => x.is_empty(),
            ConstraintGoalDescription::AtMostOnce(x) => x.is_empty(),
            ConstraintGoalDescription::SometimeAfter(x, y) => x.is_empty() && y.is_empty(),
            ConstraintGoalDescription::SometimeBefore(x, y) => x.is_empty() && y.is_empty(),
            ConstraintGoalDescription::AlwaysWithin(_, x, y) => x.is_empty() && y.is_empty(),
            ConstraintGoalDescription::HoldDuring(_, _, x) => x.is_empty(),
            ConstraintGoalDescription::HoldAfter(_, x) => x.is_empty(),
        }
    }
}

impl ConstraintGoalDescriptionInner {
    /// Creates a nested constraint goal description.
    #[doc(alias = "new_nested")]
    pub fn nested(gd: ConstraintGoalDescription) -> Self {
        Self::Nested(Box::new(gd))
    }

    /// Alias for [`Self::nested`].
    pub fn new_nested(gd: ConstraintGoalDescription) -> Self {
        Self::nested(gd)
    }

    /// Creates a plain goal description.
    #[doc(alias = "new_goal")]
    pub const fn goal(gd: GoalDescription) -> Self {
        Self::Goal(gd)
    }

    /// Alias for [`Self::goal`].
    pub fn new_goal(gd: GoalDescription) -> Self {
        Self::goal(gd)
    }

    /// Returns `true` if the contained goal description is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Goal(gd) => gd.is_empty(),
            Self::Nested(gd) => gd.is_empty(),
        }
    }
}

impl From<ConstraintGoalDescription> for ConstraintGoalDescriptionInner {
    fn from(value: ConstraintGoalDescription) -> Self {
        ConstraintGoalDescriptionInner::nested(value)
    }
}

impl From<GoalDescription> for ConstraintGoalDescriptionInner {
    fn from(value: GoalDescription) -> Self {
        ConstraintGoalDescriptionInner::goal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_goal_description_is_empty() {
        let empty = ConstraintGoalDescription::default();
        assert!(empty.is_empty());

        let gd = GoalDescription::and(Vec::new());
        let non_empty = ConstraintGoalDescription::at_end(gd);
        assert!(non_empty.is_empty());
    }

    #[test]
    fn constraint_goal_description_is_empty_nested() {
        let inner = ConstraintGoalDescriptionInner::goal(GoalDescription::and(Vec::new()));
        let always = ConstraintGoalDescription::always(inner);
        assert!(always.is_empty());
    }

    #[test]
    fn constraint_goal_description_inner_is_empty() {
        let goal_inner = ConstraintGoalDescriptionInner::goal(GoalDescription::and(Vec::new()));
        assert!(goal_inner.is_empty());

        let nested = ConstraintGoalDescriptionInner::nested(ConstraintGoalDescription::default());
        assert!(nested.is_empty());
    }

    #[test]
    fn constraint_goal_description_constructors() {
        let inner = ConstraintGoalDescriptionInner::goal(GoalDescription::and(Vec::new()));

        let _ = ConstraintGoalDescription::sometime(inner.clone());
        let _ = ConstraintGoalDescription::at_most_once(inner.clone());
        let _ = ConstraintGoalDescription::sometime_after(inner.clone(), inner.clone());
        let _ = ConstraintGoalDescription::sometime_before(inner.clone(), inner.clone());
        let _ =
            ConstraintGoalDescription::always_within(Number::from(5), inner.clone(), inner.clone());
        let _ = ConstraintGoalDescription::hold_during(
            Number::from(0),
            Number::from(10),
            inner.clone(),
        );
        let _ = ConstraintGoalDescription::hold_after(Number::from(5), inner);
    }

    #[test]
    fn constraint_goal_description_inner_from() {
        let gd = GoalDescription::and(Vec::new());
        let from_gd: ConstraintGoalDescriptionInner = gd.clone().into();
        assert!(matches!(from_gd, ConstraintGoalDescriptionInner::Goal(_)));

        let con_gd = ConstraintGoalDescription::default();
        let from_con_gd: ConstraintGoalDescriptionInner = con_gd.into();
        assert!(matches!(
            from_con_gd,
            ConstraintGoalDescriptionInner::Nested(_)
        ));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let inner = ConstraintGoalDescriptionInner::goal(GoalDescription::and(Vec::new()));
        let num = Number::from(5);

        let _ = ConstraintGoalDescription::new_and(vec![ConstraintGoalDescription::default()]);
        let _ = ConstraintGoalDescription::new_forall(
            TypedVariables::default(),
            ConstraintGoalDescription::default(),
        );
        let _ = ConstraintGoalDescription::new_at_end(GoalDescription::and(Vec::new()));
        let _ = ConstraintGoalDescription::new_always(inner.clone());
        let _ = ConstraintGoalDescription::new_sometime(inner.clone());
        let _ = ConstraintGoalDescription::new_within(num, inner.clone());
        let _ = ConstraintGoalDescription::new_at_most_once(inner.clone());
        let _ = ConstraintGoalDescription::new_sometime_after(inner.clone(), inner.clone());
        let _ = ConstraintGoalDescription::new_sometime_before(inner.clone(), inner.clone());
        let _ = ConstraintGoalDescription::new_always_within(num, inner.clone(), inner.clone());
        let _ = ConstraintGoalDescription::new_hold_during(
            Number::from(0),
            Number::from(10),
            inner.clone(),
        );
        let _ = ConstraintGoalDescription::new_hold_after(num, inner.clone());

        // ConstraintGoalDescriptionInner
        let _ = ConstraintGoalDescriptionInner::new_nested(ConstraintGoalDescription::default());
        let _ = ConstraintGoalDescriptionInner::new_goal(GoalDescription::and(Vec::new()));
    }

    #[test]
    fn is_empty_non_empty_variants() {
        use crate::types::{AtomicFormula, Predicate, Term};

        let term = Term::new_name(crate::types::Name::new("x"));
        let atomic = AtomicFormula::predicate(Predicate::string("at"), vec![term]);
        let gd = GoalDescription::AtomicFormula(atomic);
        let inner = ConstraintGoalDescriptionInner::goal(gd.clone());

        let always = ConstraintGoalDescription::always(inner.clone());
        assert!(!always.is_empty());

        let sometime = ConstraintGoalDescription::sometime(inner.clone());
        assert!(!sometime.is_empty());

        let within = ConstraintGoalDescription::within(Number::from(5), inner.clone());
        assert!(!within.is_empty());

        let at_most_once = ConstraintGoalDescription::at_most_once(inner.clone());
        assert!(!at_most_once.is_empty());

        let sometime_after =
            ConstraintGoalDescription::sometime_after(inner.clone(), inner.clone());
        assert!(!sometime_after.is_empty());

        let sometime_before =
            ConstraintGoalDescription::sometime_before(inner.clone(), inner.clone());
        assert!(!sometime_before.is_empty());

        let always_within =
            ConstraintGoalDescription::always_within(Number::from(5), inner.clone(), inner.clone());
        assert!(!always_within.is_empty());

        let hold_during = ConstraintGoalDescription::hold_during(
            Number::from(0),
            Number::from(10),
            inner.clone(),
        );
        assert!(!hold_during.is_empty());

        let hold_after = ConstraintGoalDescription::hold_after(Number::from(5), inner);
        assert!(!hold_after.is_empty());

        let at_end = ConstraintGoalDescription::at_end(gd);
        assert!(!at_end.is_empty());

        let nested_inner = ConstraintGoalDescriptionInner::nested(
            ConstraintGoalDescription::at_end(GoalDescription::and(vec![
                GoalDescription::AtomicFormula(AtomicFormula::predicate(
                    Predicate::string("on"),
                    vec![
                        Term::new_name(crate::types::Name::new("a")),
                        Term::new_name(crate::types::Name::new("b")),
                    ],
                )),
            ])),
        );
        assert!(!nested_inner.is_empty());
    }
}
