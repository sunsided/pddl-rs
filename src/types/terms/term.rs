use crate::types::FunctionTerm;
use crate::types::Name;
use crate::types::Variable;

/// A term, i.e. a [`Name`], [`Variable`] or [`FunctionTerm`].
///
/// ## Usage
/// Used by [`GoalDescription`](crate::GoalDescription), [`FunctionTerm`],
/// [`FunctionHead`](crate::FunctionHead), [`PrimitiveEffect`](crate::PrimitiveEffect) and [`InitElement`](crate::InitElement).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term {
    /// An object or constant name.
    Name(Name),
    /// A variable, e.g. `?x`.
    Variable(Variable),
    /// A function term, `(<function-symbol> <term>*)`.
    Function(FunctionTerm),
}

impl Term {
    /// Creates a name term.
    pub const fn new_name(name: Name) -> Self {
        Self::Name(name)
    }

    /// Creates a variable term.
    pub const fn new_variable(var: Variable) -> Self {
        Self::Variable(var)
    }

    /// Creates a function term.
    pub const fn new_function(fun: FunctionTerm) -> Self {
        Self::Function(fun)
    }
}

impl From<Name> for Term {
    fn from(value: Name) -> Self {
        Self::Name(value)
    }
}

impl From<Variable> for Term {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

impl From<FunctionTerm> for Term {
    fn from(value: FunctionTerm) -> Self {
        Self::Function(value)
    }
}
