//! Contains atomic function skeletons via the [`AtomicFunctionSkeleton`] type.

use crate::types::Typed;
use crate::types::{FunctionSymbol, TypedVariables, Variable};

/// A numeric fluent, similar to a predicate, is a variable which applies to zero or more objects
/// and maintains a value throughout the duration of the plan.
///
/// It is declared with a name followed by the object type to which it applies.
/// For example, the function declaration
///
/// ```pddl
/// (battery-level ?r - rover)
/// ```
///
/// means that every `rover` object in the domain has a variable which maintains a value for
/// `battery-level`. A function can apply to zero or more objects, meaning we could also use it
/// to represent a numeric value between two values, for example a distance.
///
/// ```pddl
/// (distance ?wp1 - waypoint ?wp2 - waypoint)
/// ```
///
/// Numeric fluents can be altered through the effects of both actions
/// ([ActionDefinition](crate::types::ActionDefinition)) and durative actions
/// ([DurativeActionDefinition](crate::types::DurativeActionDefinition)).
///
/// There are a number of supported effects for numeric fluents, e.g.
/// [`BinaryOp`](crate::types::BinaryOp) and [`AssignOp`](crate::types::AssignOp).
///
/// ## Usage
/// Used by [`Functions`](crate::Functions).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomicFunctionSkeleton {
    /// The name of the fluent, e.g. `battery-level`.
    symbol: FunctionSymbol,
    /// The list of parameters to the fluent, e.g. `?r - rover`.
    variables: TypedVariables,
}

impl AtomicFunctionSkeleton {
    pub const fn new(symbol: FunctionSymbol, variables: TypedVariables) -> Self {
        Self { symbol, variables }
    }

    /// Gets a reference to the function symbol.
    pub const fn symbol(&self) -> &FunctionSymbol {
        &self.symbol
    }

    /// Gets a reference to the variables.
    pub fn variables(&self) -> &TypedVariables {
        &self.variables
    }
}

impl From<(FunctionSymbol, TypedVariables)> for AtomicFunctionSkeleton {
    fn from(value: (FunctionSymbol, TypedVariables)) -> Self {
        AtomicFunctionSkeleton::new(value.0, value.1)
    }
}

impl From<(FunctionSymbol, Vec<Typed<Variable>>)> for AtomicFunctionSkeleton {
    fn from(value: (FunctionSymbol, Vec<Typed<Variable>>)) -> Self {
        AtomicFunctionSkeleton::new(value.0, value.1.into())
    }
}
