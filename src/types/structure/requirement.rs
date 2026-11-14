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
/// to declare a requirement for [`STRIPS`](Self::Strips).
///
/// ## References
/// - "Complete BNF description of PDDL 3.1 (completely corrected)", Daniel L. Kovacs (`dkovacs@mit.bme.hu`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum Requirement {
    /// Basic STRIPS-style adds and deletes.
    #[default]
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
    /// Allow functions (fluents) whose values are objects rather than numbers.
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

/// The PDDL keywords for [`Requirement`].
pub mod names {
    /// The `:strips` requirement key.
    pub const STRIPS: &str = ":strips";
    /// The `:typing` requirement key.
    pub const TYPING: &str = ":typing";
    /// The `:negative-preconditions` requirement key.
    pub const NEGATIVE_PRECONDITIONS: &str = ":negative-preconditions";
    /// The `:disjunctive-preconditions` requirement key.
    pub const DISJUNCTIVE_PRECONDITIONS: &str = ":disjunctive-preconditions";
    /// The `:equality` requirement key.
    pub const EQUALITY: &str = ":equality";
    /// The `:existential-preconditions` requirement key.
    pub const EXISTENTIAL_PRECONDITIONS: &str = ":existential-preconditions";
    /// The `:universal-preconditions` requirement key.
    pub const UNIVERSAL_PRECONDITIONS: &str = ":universal-preconditions";
    /// The `:quantified-preconditions` requirement key.
    pub const QUANTIFIED_PRECONDITIONS: &str = ":quantified-preconditions";
    /// The `:conditional-effects` requirement key.
    pub const CONDITIONAL_EFFECTS: &str = ":conditional-effects";
    /// The `:fluents` requirement key.
    pub const FLUENTS: &str = ":fluents";
    /// The `:numeric-fluents` requirement key.
    pub const NUMERIC_FLUENTS: &str = ":numeric-fluents";
    /// The `:object-fluents` requirement key.
    pub const OBJECT_FLUENTS: &str = ":object-fluents";
    /// The `:adl` requirement key.
    pub const ADL: &str = ":adl";
    /// The `:durative-actions` requirement key.
    pub const DURATIVE_ACTIONS: &str = ":durative-actions";
    /// The `:duration-inequalities` requirement key.
    pub const DURATION_INEQUALITIES: &str = ":duration-inequalities";
    /// The `:continuous-effects` requirement key.
    pub const CONTINUOUS_EFFECTS: &str = ":continuous-effects";
    /// The `:derived-predicates` requirement key.
    pub const DERIVED_PREDICATES: &str = ":derived-predicates";
    /// The `:timed-initial-literals` requirement key.
    pub const TIMED_INITIAL_LITERALS: &str = ":timed-initial-literals";
    /// The `:preferences` requirement key.
    pub const PREFERENCES: &str = ":preferences";
    /// The `:constraints` requirement key.
    pub const CONSTRAINTS: &str = ":constraints";
    /// The `:action-costs` requirement key.
    pub const ACTION_COSTS: &str = ":action-costs";
}

impl Requirement {
    /// Returns whether this requirement is `other` or implies it, e.g. `:fluents` contains `:numeric-fluents`.
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
                    || other == Self::ExistentialPreconditions
                    || other == Self::UniversalPreconditions
                    || other == Self::ConditionalEffects
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

    /// Returns the PDDL requirement key, e.g. `:strips`.
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

    /// Expands a requirement into the requirements it stands for.
    /// This can be helpful when dealing with requirements such as `:quantified-preconditions`,
    /// `:fluents` and `:adl` that are shorthands for a set of other requirements; any other
    /// requirement expands to itself.
    pub fn expand(&self) -> &'static [Requirement] {
        match self {
            Self::QuantifiedPreconditions => {
                &[Self::ExistentialPreconditions, Self::UniversalPreconditions]
            }
            Self::Fluents => &[Self::NumericFluents, Self::ObjectFluents],
            Self::Adl => &[
                Self::Strips,
                Self::Typing,
                Self::NegativePreconditions,
                Self::DisjunctivePreconditions,
                Self::Equality,
                Self::ExistentialPreconditions, // added via Requirement::QuantifiedPreconditions,
                Self::UniversalPreconditions,   // added via Requirement::QuantifiedPreconditions,
                Self::ConditionalEffects,
            ],
            Self::Strips => &[Self::Strips],
            Self::Typing => &[Self::Typing],
            Self::NegativePreconditions => &[Self::NegativePreconditions],
            Self::DisjunctivePreconditions => &[Self::DisjunctivePreconditions],
            Self::Equality => &[Self::Equality],
            Self::ExistentialPreconditions => &[Self::ExistentialPreconditions],
            Self::UniversalPreconditions => &[Self::UniversalPreconditions],
            Self::ConditionalEffects => &[Self::ConditionalEffects],
            Self::NumericFluents => &[Self::NumericFluents],
            Self::ObjectFluents => &[Self::ObjectFluents],
            Self::DurativeActions => &[Self::DurativeActions],
            Self::DurationInequalities => &[Self::DurationInequalities],
            Self::ContinuousEffects => &[Self::ContinuousEffects],
            Self::DerivedPredicates => &[Self::DerivedPredicates],
            Self::TimedInitialLiterals => &[Self::TimedInitialLiterals],
            Self::Preferences => &[Self::Preferences],
            Self::Constraints => &[Self::Constraints],
            Self::ActionCosts => &[Self::ActionCosts],
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
/// An error when parsing a [`Requirement`] from a string.
pub enum ParseError {
    #[error("Invalid requirement")]
    /// The string is not a known requirement key.
    InvalidRequirement,
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
    use assert2::check;

    #[test]
    fn str_equals_works() {
        check!(Requirement::Strips == ":strips");
        check!(Requirement::Typing == ":typing");
        check!(Requirement::NegativePreconditions == ":negative-preconditions");
        check!(Requirement::DisjunctivePreconditions == ":disjunctive-preconditions");
        check!(Requirement::Equality == ":equality");
        check!(Requirement::ExistentialPreconditions == ":existential-preconditions");
        check!(Requirement::UniversalPreconditions == ":universal-preconditions");
        check!(Requirement::QuantifiedPreconditions == ":quantified-preconditions");
        check!(Requirement::ConditionalEffects == ":conditional-effects");
        check!(Requirement::Fluents == ":fluents");
        check!(Requirement::NumericFluents == ":numeric-fluents");
        check!(Requirement::Adl == ":adl");
        check!(Requirement::DurativeActions == ":durative-actions");
        check!(Requirement::DurationInequalities == ":duration-inequalities");
        check!(Requirement::ContinuousEffects == ":continuous-effects");
        check!(Requirement::DerivedPredicates == ":derived-predicates");
        check!(Requirement::TimedInitialLiterals == ":timed-initial-literals");
        check!(Requirement::Preferences == ":preferences");
        check!(Requirement::Constraints == ":constraints");
        check!(Requirement::ActionCosts == ":action-costs");
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
            Requirement::ExistentialPreconditions,
            Requirement::UniversalPreconditions,
            Requirement::ConditionalEffects,
        ] {
            check!(Requirement::Adl.contains(req));
        }
        check!(!Requirement::Adl.contains(Requirement::ContinuousEffects));

        // `contains` agrees with `expand`.
        for req in Requirement::Adl.expand() {
            check!(Requirement::Adl.contains(req), "{req:?}");
        }

        // Check for self.
        check!(Requirement::Adl.contains(Requirement::Adl));
    }

    #[test]
    fn contains_quantified_preconditions_works() {
        for req in [
            Requirement::ExistentialPreconditions,
            Requirement::UniversalPreconditions,
        ] {
            check!(Requirement::QuantifiedPreconditions.contains(req));
        }

        // Check for self.
        check!(Requirement::QuantifiedPreconditions.contains(Requirement::QuantifiedPreconditions));
    }

    #[test]
    fn contains_fluents_works() {
        for req in [Requirement::NumericFluents, Requirement::ObjectFluents] {
            check!(Requirement::Fluents.contains(req));
        }

        // Check for self.
        check!(Requirement::Fluents.contains(Requirement::Fluents));
    }
}
