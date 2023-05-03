//! Type definitions.

mod action_definition;
mod action_symbols;
pub(crate) mod assign_op;
pub(crate) mod assign_op_t;
mod atomic_formula;
mod atomic_formula_skeleton;
mod atomic_function_skeleton;
mod basic_function_term;
pub(crate) mod binary_comp;
pub(crate) mod binary_op;
mod c_effect;
mod con_gd;
mod conditional_effect;
mod constants;
pub(crate) mod d_op;
mod d_value;
mod da_def;
mod da_effect;
mod da_gd;
mod da_symbol;
mod derived_predicate;
mod domain;
mod domain_constraints_def;
mod duration_constraint;
mod effect;
mod f_assign_da;
mod f_comp;
mod f_exp;
mod f_exp_da;
mod f_exp_t;
mod f_head;
mod function_symbols;
mod function_term;
mod function_type;
mod function_typed;
mod function_typed_list;
mod functions;
mod gd;
mod goal_def;
mod init_el;
mod init_els;
pub(crate) mod interval;
mod iterators;
mod length_spec;
mod literal;
mod metric_f_exp;
mod metric_spec;
pub(crate) mod multi_op;
mod name;
mod number;
mod objects;
pub(crate) mod optimization;
mod p_effect;
mod pre_gd;
mod predicate;
mod predicate_definitions;
mod pref_con_gd;
mod pref_gd;
mod pref_name;
mod pref_timed_gd;
mod preference;
mod problem;
mod problem_constraints_def;
pub(crate) mod requirement;
mod requirements;
mod simple_duration_constraint;
mod structure_def;
mod structure_defs;
mod term;
pub(crate) mod time_specifier;
mod timed_effect;
mod timed_gd;
mod timeless;
mod r#type;
mod typed;
mod typed_list;
mod types;
mod variable;

pub use action_definition::ActionDefinition;
pub use action_symbols::ActionSymbol;
pub use assign_op::AssignOp;
pub use assign_op_t::AssignOpT;
pub use atomic_formula::{AtomicFormula, EqualityAtomicFormula, PredicateAtomicFormula};
pub use atomic_formula_skeleton::AtomicFormulaSkeleton;
pub use atomic_function_skeleton::AtomicFunctionSkeleton;
pub use basic_function_term::BasicFunctionTerm;
pub use binary_comp::BinaryComp;
pub use binary_op::BinaryOp;
pub use c_effect::CEffect;
pub use con_gd::{Con2GD, ConGD};
pub use conditional_effect::ConditionalEffect;
pub use constants::Constants;
pub use d_op::DOp;
pub use d_value::DurationValue;
pub use da_def::DurativeActionDefinition;
pub use da_effect::DurativeActionEffect;
pub use da_gd::DurativeActionGoalDefinition;
pub use da_symbol::DurativeActionSymbol;
pub use derived_predicate::DerivedPredicate;
pub use domain::Domain;
pub use domain_constraints_def::DomainConstraintsDef;
pub use duration_constraint::DurationConstraint;
pub use effect::Effect;
pub use f_assign_da::FAssignDa;
pub use f_comp::FComp;
pub use f_exp::FExp;
pub use f_exp_da::FExpDa;
pub use f_exp_t::FExpT;
pub use f_head::FHead;
pub use function_symbols::FunctionSymbol;
pub use function_term::FunctionTerm;
pub use function_type::FunctionType;
pub use function_typed::FunctionTyped;
pub use function_typed_list::FunctionTypedList;
pub use functions::Functions;
pub use gd::GoalDefinition;
pub use goal_def::GoalDef;
pub use init_el::InitElement;
pub use init_els::InitElements;
pub use interval::Interval;
pub use length_spec::LengthSpec;
pub use literal::Literal;
pub use metric_f_exp::MetricFExp;
pub use metric_spec::MetricSpec;
pub use multi_op::MultiOp;
pub use name::Name;
pub use number::Number;
pub use objects::Objects;
pub use optimization::Optimization;
pub use p_effect::PEffect;
pub use pre_gd::PreconditionGoalDefinition;
pub use predicate::Predicate;
pub use predicate_definitions::PredicateDefinitions;
pub use pref_con_gd::PrefConGD;
pub use pref_gd::PreferenceGD;
pub use pref_name::PreferenceName;
pub use pref_timed_gd::PrefTimedGD;
pub use preference::Preference;
pub use problem::Problem;
pub use problem_constraints_def::ProblemConstraintsDef;
pub use r#type::{PrimitiveType, Type};
pub use requirement::Requirement;
pub use requirements::Requirements;
pub use simple_duration_constraint::SimpleDurationConstraint;
pub use structure_def::StructureDef;
pub use structure_defs::StructureDefs;
pub use term::Term;
pub use time_specifier::TimeSpecifier;
pub use timed_effect::TimedEffect;
pub use timed_gd::TimedGD;
pub use timeless::Timeless;
pub use typed::{ToTyped, Typed};
pub use typed_list::TypedList;
pub use types::Types;
pub use variable::Variable;

// Internal re-exports.
#[allow(unused_imports)]
pub(crate) use r#type::{TYPE_NUMBER, TYPE_OBJECT};

pub type NameLiteral<'a> = Literal<'a, Name<'a>>;
pub type TermLiteral<'a> = Literal<'a, Term<'a>>;

pub type TypedVariables<'a> = TypedList<'a, Variable<'a>>;
pub type TypedNames<'a> = TypedList<'a, Name<'a>>;
