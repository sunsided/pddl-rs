//! Goal descriptions, preferences and constraints.

use crate::serde::{
    conjunction, deserialize_from_pddl, formula, preference, serialize_deref, Head, Keyed,
};
use crate::{
    AtomicFormula, ConstraintGoalDescription, ConstraintGoalDescriptionInner,
    EqualityAtomicFormula, GoalDescription, Literal, PreconditionGoalDescription,
    PreconditionGoalDescriptions, PredicateAtomicFormula, Preference,
    PreferenceConstraintGoalDescription, PreferenceConstraintGoalDescriptions,
    PreferenceGoalDescription, PreferenceName,
};
use ::serde::{Serialize, Serializer};

serialize_deref!(PreferenceName);

impl<T: Serialize> Serialize for AtomicFormula<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AtomicFormula::Equality(formula) => formula.serialize(serializer),
            AtomicFormula::Predicate(formula) => formula.serialize(serializer),
        }
    }
}

impl<T: Serialize> Serialize for EqualityAtomicFormula<T> {
    /// `["=", first, second]`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Head("=", &[self.first(), self.second()]).serialize(serializer)
    }
}

impl<T: Serialize> Serialize for PredicateAtomicFormula<T> {
    /// `["predicate", values...]`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Head(self.predicate(), self.values()).serialize(serializer)
    }
}

impl<T: Serialize> Serialize for Literal<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Literal::AtomicFormula(formula) => formula.serialize(serializer),
            Literal::NotAtomicFormula(formula) => Keyed("not", formula).serialize(serializer),
        }
    }
}

impl Serialize for GoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            GoalDescription::AtomicFormula(formula) => formula.serialize(serializer),
            GoalDescription::Literal(literal) => literal.serialize(serializer),
            GoalDescription::And(gds) => Keyed("and", gds).serialize(serializer),
            GoalDescription::Or(gds) => Keyed("or", gds).serialize(serializer),
            GoalDescription::Not(gd) => Keyed("not", gd).serialize(serializer),
            GoalDescription::Imply(a, b) => Keyed("imply", &(a, b)).serialize(serializer),
            GoalDescription::Exists(variables, gd) => {
                Keyed("exists", &(variables, gd)).serialize(serializer)
            }
            GoalDescription::ForAll(variables, gd) => {
                Keyed("forall", &(variables, gd)).serialize(serializer)
            }
            GoalDescription::FluentComparison(comparison) => comparison.serialize(serializer),
        }
    }
}

impl Serialize for Preference {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        preference(serializer, self.name().as_ref(), self.goal())
    }
}

impl Serialize for PreferenceGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PreferenceGoalDescription::Goal(gd) => gd.serialize(serializer),
            PreferenceGoalDescription::Preference(pref) => pref.serialize(serializer),
        }
    }
}

impl Serialize for PreconditionGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PreconditionGoalDescription::Preference(gd) => gd.serialize(serializer),
            PreconditionGoalDescription::Forall(variables, gds) => {
                Keyed("forall", &(variables, gds)).serialize(serializer)
            }
        }
    }
}

impl Serialize for PreconditionGoalDescriptions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        conjunction(serializer, self)
    }
}

impl Serialize for ConstraintGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        type C = ConstraintGoalDescription;
        match self {
            C::And(gds) => Keyed("and", gds).serialize(serializer),
            C::Forall(variables, gd) => Keyed("forall", &(variables, gd)).serialize(serializer),
            C::AtEnd(gd) => Keyed("at end", gd).serialize(serializer),
            C::Always(gd) => Keyed("always", gd).serialize(serializer),
            C::Sometime(gd) => Keyed("sometime", gd).serialize(serializer),
            C::Within(time, gd) => Keyed("within", &(time, gd)).serialize(serializer),
            C::AtMostOnce(gd) => Keyed("at-most-once", gd).serialize(serializer),
            C::SometimeAfter(a, b) => Keyed("sometime-after", &(a, b)).serialize(serializer),
            C::SometimeBefore(a, b) => Keyed("sometime-before", &(a, b)).serialize(serializer),
            C::AlwaysWithin(time, a, b) => {
                Keyed("always-within", &(time, a, b)).serialize(serializer)
            }
            C::HoldDuring(start, end, gd) => {
                Keyed("hold-during", &(start, end, gd)).serialize(serializer)
            }
            C::HoldAfter(time, gd) => Keyed("hold-after", &(time, gd)).serialize(serializer),
        }
    }
}

impl Serialize for ConstraintGoalDescriptionInner {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ConstraintGoalDescriptionInner::Goal(gd) => gd.serialize(serializer),
            ConstraintGoalDescriptionInner::Nested(gd) => gd.serialize(serializer),
        }
    }
}

impl Serialize for PreferenceConstraintGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        type P = PreferenceConstraintGoalDescription;
        match self {
            P::Goal(gd) => gd.serialize(serializer),
            P::Forall(variables, gds) => Keyed("forall", &(variables, gds)).serialize(serializer),
            P::Preference(name, gd) => preference(serializer, name.as_ref(), gd),
        }
    }
}

impl Serialize for PreferenceConstraintGoalDescriptions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        conjunction(serializer, self)
    }
}

deserialize_from_pddl!(
    PreferenceName => formula,
    GoalDescription => formula,
    PreferenceGoalDescription => formula,
    PreconditionGoalDescriptions => formula,
    ConstraintGoalDescription => formula,
    ConstraintGoalDescriptionInner => formula,
    PreferenceConstraintGoalDescriptions => formula,
);
