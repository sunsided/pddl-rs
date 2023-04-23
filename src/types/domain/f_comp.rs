use crate::types::domain::{BinaryComp, FExp};

/// An f-comp.
#[derive(Debug, Clone, PartialEq)]
pub struct FComp<'a>(BinaryComp, FExp<'a>, FExp<'a>);

impl<'a> FComp<'a> {
    pub const fn new(comp: BinaryComp, lhs: FExp<'a>, rhs: FExp<'a>) -> Self {
        Self(comp, lhs, rhs)
    }
}

impl<'a> From<(BinaryComp, FExp<'a>, FExp<'a>)> for FComp<'a> {
    fn from(value: (BinaryComp, FExp<'a>, FExp<'a>)) -> Self {
        FComp::new(value.0, value.1, value.2)
    }
}
