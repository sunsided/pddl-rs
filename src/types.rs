//! Type definitions.

pub(crate) mod actions;
pub(crate) mod durative;
pub(crate) mod expressions;
pub(crate) mod goals;
pub(crate) mod structure;
pub(crate) mod terms;

pub use actions::*;
pub use durative::*;
pub use expressions::*;
pub use goals::*;
pub use structure::*;
pub use terms::*;

mod iterators;

/// A literal over names.
pub type NameLiteral = Literal<Name>;

/// A literal over terms.
pub type TermLiteral = Literal<Term>;

/// A list of typed variables, e.g. action parameters `?x - truck ?l - location`.
pub type TypedVariables = TypedList<Variable>;

/// A list of typed names, e.g. objects or constants `t1 t2 - truck`.
pub type TypedNames = TypedList<Name>;

// The names before PDDL's "goal description" (GD) was spelled out correctly.
// When removing these aliases, add `#[doc(alias = "<old name>")]` to each renamed
// type so that rustdoc search still finds it by its old name.
/// Former name of [`GoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `GoalDescription`")]
pub type GoalDefinition = GoalDescription;

/// Former name of [`ConstraintGoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `ConstraintGoalDescription`")]
pub type ConstraintGoalDefinition = ConstraintGoalDescription;

/// Former name of [`ConstraintGoalDescriptionInner`].
#[deprecated(since = "0.4.0", note = "renamed to `ConstraintGoalDescriptionInner`")]
pub type ConstraintGoalDefinitionInner = ConstraintGoalDescriptionInner;

/// Former name of [`DurativeActionGoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `DurativeActionGoalDescription`")]
pub type DurativeActionGoalDefinition = DurativeActionGoalDescription;

/// Former name of [`PreconditionGoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `PreconditionGoalDescription`")]
pub type PreconditionGoalDefinition = PreconditionGoalDescription;

/// Former name of [`PreconditionGoalDescriptions`].
#[deprecated(since = "0.4.0", note = "renamed to `PreconditionGoalDescriptions`")]
pub type PreconditionGoalDefinitions = PreconditionGoalDescriptions;

/// Former name of [`PreferenceConstraintGoalDescription`].
#[deprecated(
    since = "0.4.0",
    note = "renamed to `PreferenceConstraintGoalDescription`"
)]
pub type PreferenceConstraintGoalDefinition = PreferenceConstraintGoalDescription;

/// Former name of [`PreferenceConstraintGoalDescriptions`].
#[deprecated(
    since = "0.4.0",
    note = "renamed to `PreferenceConstraintGoalDescriptions`"
)]
pub type PreferenceConstraintGoalDefinitions = PreferenceConstraintGoalDescriptions;

/// Former name of [`PreferenceGoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `PreferenceGoalDescription`")]
pub type PreferenceGoalDefinition = PreferenceGoalDescription;

/// Former name of [`PreferenceTimedGoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `PreferenceTimedGoalDescription`")]
pub type PreferenceTimedGoalDefinition = PreferenceTimedGoalDescription;

/// Former name of [`ProblemGoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `ProblemGoalDescription`")]
pub type ProblemGoalDefinition = ProblemGoalDescription;

/// Former name of [`TimedGoalDescription`].
#[deprecated(since = "0.4.0", note = "renamed to `TimedGoalDescription`")]
pub type TimedGoalDefinition = TimedGoalDescription;
