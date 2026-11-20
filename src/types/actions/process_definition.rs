//! Contains PDDL+ process definitions via the [`ProcessDefinition`] type.

use crate::types::TypedVariables;
use crate::types::{ActionSymbol, DurativeActionEffect};
use crate::PreconditionGoalDescriptions;

/// A PDDL+ process: continuous change that runs whenever its precondition holds.
///
/// `(:process <name> :parameters (...) :precondition <pre-GD> :effect <effect>)`. The
/// effect is read like a durative action's, so continuous effects such as
/// `(increase (height) (* #t (velocity)))` keep their `#t`; a process's effect should
/// contain only continuous effects. A PDDL+ event has the form of an action and is an
/// [`ActionDefinition`](crate::ActionDefinition) in [`StructureDef::Event`](crate::StructureDef::Event).
///
/// ## Requirements
/// Requires PDDL+ (`:time`), which the parser keeps as an unknown requirement.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessDefinition {
    /// The process's name, from `(:process <name> ...)`.
    symbol: ActionSymbol,
    /// The process's parameters, from `:parameters`.
    parameters: TypedVariables,
    /// The condition under which the process runs, from `:precondition`; empty if it
    /// always runs.
    precondition: PreconditionGoalDescriptions,
    /// The process's continuous effects, from `:effect`; `None` if it has none.
    effect: Option<DurativeActionEffect>,
}

impl ProcessDefinition {
    /// Creates a process definition from its name, parameters, precondition and effect.
    #[must_use]
    pub const fn new(
        symbol: ActionSymbol,
        parameters: TypedVariables,
        precondition: PreconditionGoalDescriptions,
        effect: Option<DurativeActionEffect>,
    ) -> Self {
        Self {
            symbol,
            parameters,
            precondition,
            effect,
        }
    }

    /// Gets the process's name.
    #[must_use]
    pub const fn symbol(&self) -> &ActionSymbol {
        &self.symbol
    }

    /// Gets the process's parameters, from `:parameters`.
    #[must_use]
    pub const fn parameters(&self) -> &TypedVariables {
        &self.parameters
    }

    /// Gets the condition under which the process runs; empty if it always runs.
    #[must_use]
    pub const fn precondition(&self) -> &PreconditionGoalDescriptions {
        &self.precondition
    }

    /// Gets the process's effect; `None` if it has none.
    #[must_use]
    pub const fn effect(&self) -> &Option<DurativeActionEffect> {
        &self.effect
    }
}

impl AsRef<ActionSymbol> for ProcessDefinition {
    fn as_ref(&self) -> &ActionSymbol {
        &self.symbol
    }
}
