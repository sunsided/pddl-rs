//! Type definitions.

pub(crate) mod actions;
pub(crate) mod durative;
pub(crate) mod expressions;
pub(crate) mod goals;
pub(crate) mod structure;
pub(crate) mod terms;

// structure: domain and problem definitions and their sections
pub use structure::atomic_formula_skeleton::AtomicFormulaSkeleton;
pub use structure::atomic_function_skeleton::AtomicFunctionSkeleton;
pub use structure::constants::Constants;
pub use structure::definition::Definition;
pub use structure::domain::Domain;
pub use structure::domain_constraints_def::DomainConstraintsDef;
pub use structure::functions::Functions;
#[allow(deprecated)]
pub use structure::goal_def::{GoalDef, ProblemGoalDescription};
pub use structure::init_el::InitElement;
pub use structure::init_els::InitElements;
pub use structure::length_spec::LengthSpec;
pub use structure::metric_spec::MetricSpec;
pub use structure::objects::Objects;
pub use structure::pddl_file::PddlFile;
pub use structure::predicate_definitions::PredicateDefinitions;
pub use structure::problem::Problem;
pub use structure::problem_constraints_def::ProblemConstraintsDef;
pub use structure::requirement::Requirement;
pub use structure::requirements::Requirements;
pub use structure::structure_def::StructureDef;
pub use structure::structure_defs::StructureDefs;
pub use structure::timeless::Timeless;
pub use structure::types::Types;

// actions: actions, derived predicates and their effects
pub use actions::action_definition::ActionDefinition;
pub use actions::action_symbols::ActionSymbol;
#[allow(deprecated)]
pub use actions::c_effect::{
    CEffect, ConditionalEffect, ForallCEffect, ForallConditionalEffect, WhenCEffect,
    WhenConditionalEffect,
};
pub use actions::conditional_effect::EffectCondition;
pub use actions::derived_predicate::DerivedPredicate;
pub use actions::effects::Effects;
#[allow(deprecated)]
pub use actions::p_effect::PEffect;
pub use actions::p_effect::PrimitiveEffect;

// goals: goal descriptions, literals, atomic formulas and preferences
pub use goals::atomic_formula::{AtomicFormula, EqualityAtomicFormula, PredicateAtomicFormula};
#[allow(deprecated)]
pub use goals::con_gd::{Con2GD, ConGD, ConstraintGoalDescription, ConstraintGoalDescriptionInner};
pub use goals::gd::GoalDescription;
pub use goals::literal::Literal;
pub use goals::pre_gd::{PreconditionGoalDescription, PreconditionGoalDescriptions};
#[allow(deprecated)]
pub use goals::pref_con_gd::{
    PrefConGD, PrefConGDs, PreferenceConstraintGoalDescription,
    PreferenceConstraintGoalDescriptions,
};
#[allow(deprecated)]
pub use goals::pref_gd::{PreferenceGD, PreferenceGoalDescription};
pub use goals::pref_name::PreferenceName;
pub use goals::preference::Preference;

// durative: durative actions: their conditions, effects, durations and timing
#[allow(deprecated)]
pub use durative::assign_op_t::{AssignOpT, TimedAssignOperator};
#[allow(deprecated)]
pub use durative::d_op::{DOp, DurationOperator};
pub use durative::d_value::DurationValue;
pub use durative::da_def::DurativeActionDefinition;
pub use durative::da_effect::DurativeActionEffect;
pub use durative::da_gd::DurativeActionGoalDescription;
pub use durative::da_symbol::DurativeActionSymbol;
pub use durative::duration_constraint::DurationConstraint;
#[allow(deprecated)]
pub use durative::f_assign_da::{DurativeActionFunctionAssignment, FAssignDa};
#[allow(deprecated)]
pub use durative::f_exp_da::{DurativeActionFluentExpression, FExpDa};
#[allow(deprecated)]
pub use durative::f_exp_t::{FExpT, TimedFluentExpression};
pub use durative::interval::Interval;
#[allow(deprecated)]
pub use durative::pref_timed_gd::{PrefTimedGD, PreferenceTimedGoalDescription};
pub use durative::simple_duration_constraint::SimpleDurationConstraint;
pub use durative::time_specifier::TimeSpecifier;
pub use durative::timed_effect::TimedEffect;
#[allow(deprecated)]
pub use durative::timed_gd::{TimedGD, TimedGoalDescription};

// expressions: numeric fluent expressions, comparisons and operators
pub use expressions::assign_op::AssignOp;
#[allow(deprecated)]
pub use expressions::binary_comp::{BinaryComp, BinaryComparison};
pub use expressions::binary_op::BinaryOp;
#[allow(deprecated)]
pub use expressions::f_comp::{FComp, FluentComparison};
#[allow(deprecated)]
pub use expressions::f_exp::{FExp, FluentExpression};
#[allow(deprecated)]
pub use expressions::f_head::{FHead, FunctionHead};
#[allow(deprecated)]
pub use expressions::metric_f_exp::MetricFExp;
pub use expressions::metric_f_exp::MetricFluentExpression;
pub use expressions::multi_op::MultiOp;
pub use expressions::optimization::Optimization;

// terms: names, variables, numbers, terms and types
pub use terms::basic_function_term::BasicFunctionTerm;
pub use terms::function_symbols::FunctionSymbol;
pub use terms::function_term::FunctionTerm;
pub use terms::function_type::FunctionType;
pub use terms::function_typed::FunctionTyped;
pub use terms::function_typed_list::FunctionTypedList;
pub use terms::name::Name;
pub use terms::number::Number;
pub use terms::predicate::Predicate;
pub use terms::r#type::{PrimitiveType, Type};
#[allow(unused_imports)]
pub(crate) use terms::r#type::{TYPE_NUMBER, TYPE_OBJECT};
pub use terms::term::Term;
pub use terms::typed::{ToTyped, Typed};
pub use terms::typed_list::TypedList;
pub use terms::variable::Variable;

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
