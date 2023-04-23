use crate::types::domain::function_term::FunctionTerm;
use crate::types::domain::Variable;
use crate::types::utility::Name;

/// A term.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Term<'a> {
    Name(Name<'a>),
    Variable(Variable<'a>),
    FunctionTerm(FunctionTerm<'a>),
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
        Self::FunctionTerm(value)
    }
}
