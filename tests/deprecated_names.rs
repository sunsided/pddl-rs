//! The pre-0.4 `...GoalDefinition` names still resolve, as deprecated aliases.

#![allow(deprecated)]

use assert2::check;

use pddl::*;

#[test]
fn old_goal_definition_names_still_work() {
    let gd: GoalDefinition = GoalDescription::from_str("(p)").unwrap();
    check!(let GoalDefinition::AtomicFormula(_) = gd);
}

/// Each alias names the same type as its replacement: a function from the new type
/// to the old one only compiles if they are the same.
#[test]
fn every_deprecated_alias_names_its_replacement() {
    macro_rules! same_type {
        ($($old:ident = $new:ident),+ $(,)?) => {$(
            let _: fn($new) -> $old = |value| value;
        )+};
    }
    same_type!(
        GoalDefinition = GoalDescription,
        ConstraintGoalDefinition = ConstraintGoalDescription,
        ConstraintGoalDefinitionInner = ConstraintGoalDescriptionInner,
        DurativeActionGoalDefinition = DurativeActionGoalDescription,
        PreconditionGoalDefinition = PreconditionGoalDescription,
        PreconditionGoalDefinitions = PreconditionGoalDescriptions,
        PreferenceConstraintGoalDefinition = PreferenceConstraintGoalDescription,
        PreferenceConstraintGoalDefinitions = PreferenceConstraintGoalDescriptions,
        PreferenceGoalDefinition = PreferenceGoalDescription,
        PreferenceTimedGoalDefinition = PreferenceTimedGoalDescription,
        ProblemGoalDefinition = ProblemGoalDescription,
        TimedGoalDefinition = TimedGoalDescription,
    );
}
