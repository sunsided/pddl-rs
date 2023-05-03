//! Contains domain requirements such as [Strips](Requirement::Strips).

use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

/// Domain requirements.
///
/// ## Usage
/// Used by [`Requirements`](crate::Requirements).
///
/// ## Notes
/// Some requirements imply others; some are abbreviations for common sets
/// of requirements. If a domain stipulates no requirements, it is assumed
/// to declare a requirement for [`Strips`](Self::Strips).
///
/// ## References
/// - "Complete BNF description of PDDL 3.1 (completely corrected)", Daniel L. Kovacs (`dkovacs@mit.bme.hu`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Requirement {
    /// Basic STRIPS-style adds and deletes.
    Strips,
    /// Allow type names in declarations of variables.
    Typing,
    /// Allow `not` in goal descriptions.
    NegativePreconditions,
    /// Allow `or` in goal descriptions
    DisjunctivePreconditions,
    /// Support `=` as built-in predicate.
    Equality,
    /// Allow `exists` in goal descriptions.
    ExistentialPreconditions,
    /// Allow `forall` in goal descriptions
    UniversalPreconditions,
    /// Same as
    /// * [`ExistentialPreconditions`](Self::ExistentialPreconditions) and
    /// * [`UniversalPreconditions`](Self::UniversalPreconditions).
    QuantifiedPreconditions,
    /// Allow `when` in action effects.
    ConditionalEffects,
    /// Same as
    /// * [`NumericFluents`](Self::NumericFluents) and
    /// * [`ObjectFluents`](Self::ObjectFluents).
    Fluents,
    /// Allow numeric function definitions and use of effects using assignment operators and arithmetic
    /// preconditions.
    NumericFluents,
    // TODO: document ObjectFluents
    ObjectFluents,
    /// Same as
    /// * [`Strips`](Self::Strips),
    /// * [`Typing`](Self::Typing),
    /// * [`NegativePreconditions`](Self::NegativePreconditions),
    /// * [`DisjunctivePreconditions`](Self::DisjunctivePreconditions),
    /// * [`Equality`](Self::Equality),
    /// * [`QuantifiedPreconditions`](Self::QuantifiedPreconditions) and
    /// * [`ConditionalEffects`](Self::ConditionalEffects).
    Adl,
    /// Allows durative actions. Note that this does not imply [`NumericFluents`](Self::NumericFluents).
    DurativeActions,
    /// Allows duration constraints in durative actions using inequalities.
    DurationInequalities,
    /// Allows durative actions to affect fluents continuously over the duration of the actions.
    ContinuousEffects,
    /// Allows predicates whose truth value is defined by a formula.
    DerivedPredicates,
    /// Allows the initial state to specify literals that will become true at a specified time point. Implies
    /// [`DurativeActions`](Self::DurativeActions).
    TimedInitialLiterals,
    /// Allows use of preferences in action preconditions and goals.
    Preferences,
    /// Allows use of constraints fields in domain and problem files.
    /// These may contain modal operators supporting trajectory constraints.
    Constraints,
    /// If this requirement is included in a PDDL specification, the use of numeric fluents is enabled
    /// (similar to the [`NumericFluents`](Self::NumericFluents) requirement).
    ///
    /// However, numeric fluents may only be used in certain very
    /// limited ways:
    ///
    /// 1. Numeric fluents may not be used in any conditions (preconditions, goal conditions,
    ///    conditions of conditional effects, etc.).
    /// 2. A numeric fluent may only be used as the target of an effect if it is 0-ary and called `total-cost`.
    ///    If such an effect is used, then the `total-cost` fluent must be explicitly initialized
    ///    to `0` in the initial state.
    /// 3. The only allowable use of numeric fluents in effects is in effects of the form
    ///    `(increase (total-cost) <numeric-term>)`, where the `<numeric-term>` is either a
    ///    nonnegative numeric constant or of the form `(<function-symbol> <term>*)`.
    ///    (The `<term>` here is interpreted as shown in the PDDL grammar, i.e. it is a
    ///    variable symbol or an object constant.
    ///    Note that this `<term>` cannot be a `<function-term>`, even if the object
    ///    fluents requirement is used.)
    /// 4. No numeric fluent may be initialized to a negative value.
    /// 5. If the problem contains a :metric specification, the objective must be
    ///    `(minimize (total-cost))`, or - only if the [`DurativeActions`](Self::DurativeActions)
    ///    requirement is also set - to  minimize a linear combination of `total-cost` and `total-time`
    ///    with non-negative coefficients.
    ///
    /// Note that an action can have multiple effects that increase `(total-cost)`, which is particularly useful
    /// in the context of conditional effects.
    ///
    /// Also note that these restrictions imply that `(total-cost)` never decreases throughout plan execution,
    /// i.e., action costs are never negative.
    ActionCosts,
}

pub mod names {
    pub const STRIPS: &'static str = ":strips";
    pub const TYPING: &'static str = ":typing";
    pub const NEGATIVE_PRECONDITIONS: &'static str = ":negative-preconditions";
    pub const DISJUNCTIVE_PRECONDITIONS: &'static str = ":disjunctive-preconditions";
    pub const EQUALITY: &'static str = ":equality";
    pub const EXISTENTIAL_PRECONDITIONS: &'static str = ":existential-preconditions";
    pub const UNIVERSAL_PRECONDITIONS: &'static str = ":universal-preconditions";
    pub const QUANTIFIED_PRECONDITIONS: &'static str = ":quantified-preconditions";
    pub const CONDITIONAL_EFFECTS: &'static str = ":conditional-effects";
    pub const FLUENTS: &'static str = ":fluents";
    pub const NUMERIC_FLUENTS: &'static str = ":numeric-fluents";
    pub const OBJECT_FLUENTS: &'static str = ":object-fluents";
    pub const ADL: &'static str = ":adl";
    pub const DURATIVE_ACTIONS: &'static str = ":durative-actions";
    pub const DURATION_INEQUALITIES: &'static str = ":duration-inequalities";
    pub const CONTINUOUS_EFFECTS: &'static str = ":continuous-effects";
    pub const DERIVED_PREDICATES: &'static str = ":derived-predicates";
    pub const TIMED_INITIAL_LITERALS: &'static str = ":timed-initial-literals";
    pub const PREFERENCES: &'static str = ":preferences";
    pub const CONSTRAINTS: &'static str = ":constraints";
    pub const ACTION_COSTS: &'static str = ":action-costs";
}

impl Requirement {
    pub fn contains<T: Borrow<Requirement>>(&self, other: T) -> bool {
        let other = other.borrow();
        match self {
            Self::QuantifiedPreconditions => {
                // Check for self to ensure every options is captured.
                other == Self::QuantifiedPreconditions
                    || other == Self::ExistentialPreconditions
                    || other == Self::UniversalPreconditions
            }
            Self::Adl => {
                // Check for self to ensure every options is captured.
                other == Self::Adl
                    || other == Self::Strips
                    || other == Self::Typing
                    || other == Self::NegativePreconditions
                    || other == Self::DisjunctivePreconditions
                    || other == Self::Equality
                    || other == Self::QuantifiedPreconditions
                    || other == Self::ContinuousEffects
            }

            Self::Fluents => {
                // Check for self to ensure every options is captured.
                other == Self::Fluents
                    || other == Self::NumericFluents
                    || other == Self::ObjectFluents
            }
            x => other.eq(x),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Requirement::Strips => names::STRIPS,
            Requirement::Typing => names::TYPING,
            Requirement::NegativePreconditions => names::NEGATIVE_PRECONDITIONS,
            Requirement::DisjunctivePreconditions => names::DISJUNCTIVE_PRECONDITIONS,
            Requirement::Equality => names::EQUALITY,
            Requirement::ExistentialPreconditions => names::EXISTENTIAL_PRECONDITIONS,
            Requirement::UniversalPreconditions => names::UNIVERSAL_PRECONDITIONS,
            Requirement::QuantifiedPreconditions => names::QUANTIFIED_PRECONDITIONS,
            Requirement::ConditionalEffects => names::CONDITIONAL_EFFECTS,
            Requirement::Fluents => names::FLUENTS,
            Requirement::NumericFluents => names::NUMERIC_FLUENTS,
            Requirement::ObjectFluents => names::OBJECT_FLUENTS,
            Requirement::Adl => names::ADL,
            Requirement::DurativeActions => names::DURATIVE_ACTIONS,
            Requirement::DurationInequalities => names::DURATION_INEQUALITIES,
            Requirement::ContinuousEffects => names::CONTINUOUS_EFFECTS,
            Requirement::DerivedPredicates => names::DERIVED_PREDICATES,
            Requirement::TimedInitialLiterals => names::TIMED_INITIAL_LITERALS,
            Requirement::Preferences => names::PREFERENCES,
            Requirement::Constraints => names::CONSTRAINTS,
            Requirement::ActionCosts => names::ACTION_COSTS,
        }
    }

    /// Expands a requirement into a list of requirements.
    /// This can be helpful when dealing with requirements such as `:quantified-preconditions`,
    /// `:fluents` and `:adl` that are shorthands for a set of other requirements.
    pub fn expand(&self) -> Vec<Requirement> {
        match self {
            Requirement::QuantifiedPreconditions => vec![
                Requirement::ExistentialPreconditions,
                Requirement::UniversalPreconditions,
            ],
            Requirement::Fluents => vec![Requirement::NumericFluents, Requirement::ObjectFluents],
            Requirement::Adl => vec![
                Requirement::Strips,
                Requirement::Typing,
                Requirement::NegativePreconditions,
                Requirement::DisjunctivePreconditions,
                Requirement::Equality,
                Requirement::ExistentialPreconditions, // added via Requirement::QuantifiedPreconditions,
                Requirement::UniversalPreconditions, // added via Requirement::QuantifiedPreconditions,
                Requirement::ConditionalEffects,
            ],
            r => vec![*r],
        }
    }
}

impl TryFrom<&str> for Requirement {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::STRIPS => Ok(Requirement::Strips),
            names::TYPING => Ok(Requirement::Typing),
            names::NEGATIVE_PRECONDITIONS => Ok(Requirement::NegativePreconditions),
            names::DISJUNCTIVE_PRECONDITIONS => Ok(Requirement::DisjunctivePreconditions),
            names::EQUALITY => Ok(Requirement::Equality),
            names::EXISTENTIAL_PRECONDITIONS => Ok(Requirement::ExistentialPreconditions),
            names::UNIVERSAL_PRECONDITIONS => Ok(Requirement::UniversalPreconditions),
            names::QUANTIFIED_PRECONDITIONS => Ok(Requirement::QuantifiedPreconditions),
            names::CONDITIONAL_EFFECTS => Ok(Requirement::ConditionalEffects),
            names::FLUENTS => Ok(Requirement::Fluents),
            names::NUMERIC_FLUENTS => Ok(Requirement::NumericFluents),
            names::OBJECT_FLUENTS => Ok(Requirement::ObjectFluents),
            names::ADL => Ok(Requirement::Adl),
            names::DURATIVE_ACTIONS => Ok(Requirement::DurativeActions),
            names::DURATION_INEQUALITIES => Ok(Requirement::DurationInequalities),
            names::CONTINUOUS_EFFECTS => Ok(Requirement::ContinuousEffects),
            names::DERIVED_PREDICATES => Ok(Requirement::DerivedPredicates),
            names::TIMED_INITIAL_LITERALS => Ok(Requirement::TimedInitialLiterals),
            names::PREFERENCES => Ok(Requirement::Preferences),
            names::CONSTRAINTS => Ok(Requirement::Constraints),
            names::ACTION_COSTS => Ok(Requirement::ActionCosts),
            _ => Err(ParseError::InvalidRequirement),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid requirement")]
    InvalidRequirement,
}

impl Default for Requirement {
    fn default() -> Self {
        Requirement::Strips
    }
}

impl Borrow<str> for Requirement {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Requirement {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for Requirement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl PartialEq<Requirement> for &Requirement {
    #[inline(always)]
    fn eq(&self, other: &Requirement) -> bool {
        other.eq(*self)
    }
}

impl PartialEq<&Requirement> for Requirement {
    #[inline(always)]
    fn eq(&self, other: &&Requirement) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<String> for Requirement {
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        other.eq(self.as_ref())
    }
}

impl PartialEq<&str> for Requirement {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        (*other).eq(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_equals_works() {
        assert_eq!(Requirement::Strips, ":strips");
        assert_eq!(Requirement::Typing, ":typing");
        assert_eq!(
            Requirement::NegativePreconditions,
            ":negative-preconditions"
        );
        assert_eq!(
            Requirement::DisjunctivePreconditions,
            ":disjunctive-preconditions"
        );
        assert_eq!(Requirement::Equality, ":equality");
        assert_eq!(
            Requirement::ExistentialPreconditions,
            ":existential-preconditions"
        );
        assert_eq!(
            Requirement::UniversalPreconditions,
            ":universal-preconditions"
        );
        assert_eq!(
            Requirement::QuantifiedPreconditions,
            ":quantified-preconditions"
        );
        assert_eq!(Requirement::ConditionalEffects, ":conditional-effects");
        assert_eq!(Requirement::Fluents, ":fluents");
        assert_eq!(Requirement::NumericFluents, ":numeric-fluents");
        assert_eq!(Requirement::Adl, ":adl");
        assert_eq!(Requirement::DurativeActions, ":durative-actions");
        assert_eq!(Requirement::DurationInequalities, ":duration-inequalities");
        assert_eq!(Requirement::ContinuousEffects, ":continuous-effects");
        assert_eq!(Requirement::DerivedPredicates, ":derived-predicates");
        assert_eq!(Requirement::TimedInitialLiterals, ":timed-initial-literals");
        assert_eq!(Requirement::Preferences, ":preferences");
        assert_eq!(Requirement::Constraints, ":constraints");
        assert_eq!(Requirement::ActionCosts, ":action-costs");
    }

    #[test]
    fn contains_adl_works() {
        for req in [
            Requirement::Strips,
            Requirement::Typing,
            Requirement::NegativePreconditions,
            Requirement::DisjunctivePreconditions,
            Requirement::Equality,
            Requirement::QuantifiedPreconditions,
            Requirement::ContinuousEffects,
        ] {
            assert!(Requirement::Adl.contains(req));
        }

        // Check for self.
        assert!(Requirement::Adl.contains(Requirement::Adl));
    }

    #[test]
    fn contains_quantified_preconditions_works() {
        for req in [
            Requirement::ExistentialPreconditions,
            Requirement::UniversalPreconditions,
        ] {
            assert!(Requirement::QuantifiedPreconditions.contains(req));
        }

        // Check for self.
        assert!(Requirement::QuantifiedPreconditions.contains(Requirement::QuantifiedPreconditions));
    }

    #[test]
    fn contains_fluents_works() {
        for req in [Requirement::NumericFluents, Requirement::ObjectFluents] {
            assert!(Requirement::Fluents.contains(req));
        }

        // Check for self.
        assert!(Requirement::Fluents.contains(Requirement::Fluents));
    }
}
