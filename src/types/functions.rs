//! Provides function definitions via the [`Functions`] type.

use crate::types::{AtomicFunctionSkeleton, FunctionTyped, FunctionTypedList};
use std::ops::Deref;

/// A set of functions.
///
/// ## Requirements
/// Requires [Fluents](crate::Requirement::Fluents).
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Functions(FunctionTypedList<AtomicFunctionSkeleton>);

impl Functions {
    pub const fn new(functions: FunctionTypedList<AtomicFunctionSkeleton>) -> Self {
        Self(functions)
    }

    /// Gets the values.
    pub fn values(&self) -> &FunctionTypedList<AtomicFunctionSkeleton> {
        &self.0
    }
}

impl Deref for Functions {
    type Target = FunctionTypedList<AtomicFunctionSkeleton>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<FunctionTypedList<AtomicFunctionSkeleton>> for Functions {
    fn from(value: FunctionTypedList<AtomicFunctionSkeleton>) -> Self {
        Functions::new(value)
    }
}

impl FromIterator<FunctionTyped<AtomicFunctionSkeleton>> for Functions {
    fn from_iter<T: IntoIterator<Item = FunctionTyped<AtomicFunctionSkeleton>>>(iter: T) -> Self {
        Functions::new(FunctionTypedList::from_iter(iter))
    }
}
