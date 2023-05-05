use crate::types::function_term::FunctionTerm;
use crate::types::Name;
use crate::types::Variable;

/// A term, i.e. a [`Name`], [`Variable`] or [`FunctionTerm`].
///
/// ## Usage
/// Used by [`GoalDefinition`](crate::GoalDefinition), [`FunctionTerm`](FunctionTerm),
/// [`FHead`](crate::FHead), [`PEffect`](crate::PEffect) and [`InitElement`](crate::InitElement).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term {
    Name(Name),
    Variable(Variable),
    Function(FunctionTerm),
}

impl Term {
    pub const fn new_name(name: Name) -> Self {
        Self::Name(name)
    }

    pub const fn new_variable(var: Variable) -> Self {
        Self::Variable(var)
    }

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
