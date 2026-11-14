//! The pre-0.4 `...GoalDefinition` names still resolve, as deprecated aliases.

#![allow(deprecated)]

use pddl::{GoalDefinition, GoalDescription, Parser};

#[test]
fn old_goal_definition_names_still_work() {
    let gd: GoalDefinition = GoalDescription::from_str("(p)").unwrap();
    assert!(matches!(gd, GoalDefinition::AtomicFormula(_)));
}
