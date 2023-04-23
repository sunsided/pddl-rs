use crate::types::function_term::FunctionTerm;
use crate::types::Name;
use crate::types::Variable;

/// A term, i.e. a [`Name`], [`Variable`] or [`FunctionTerm`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term<'a> {
    Name(Name<'a>),
    Variable(Variable<'a>),
    Function(FunctionTerm<'a>),
}

impl<'a> Term<'a> {
    pub const fn new_name(name: Name<'a>) -> Self {
        Self::Name(name)
    }

    pub const fn new_variable(var: Variable<'a>) -> Self {
        Self::Variable(var)
    }

    pub const fn new_function(fun: FunctionTerm<'a>) -> Self {
        Self::Function(fun)
    }
}

impl<'a> From<Name<'a>> for Term<'a> {
    fn from(value: Name<'a>) -> Self {
        Self::Name(value)
    }
}

impl<'a> From<Variable<'a>> for Term<'a> {
    fn from(value: Variable<'a>) -> Self {
        Self::Variable(value)
    }
}

impl<'a> From<FunctionTerm<'a>> for Term<'a> {
    fn from(value: FunctionTerm<'a>) -> Self {
        Self::Function(value)
    }
}
