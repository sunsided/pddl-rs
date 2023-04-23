//! Provides function definitions.

use crate::types::{AtomicFunctionSkeleton, FunctionTyped, FunctionTypedList};
use std::ops::Deref;

/// A set of functions.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Functions<'a>(FunctionTypedList<'a, AtomicFunctionSkeleton<'a>>);

impl<'a> Functions<'a> {
    pub const fn new(functions: FunctionTypedList<'a, AtomicFunctionSkeleton<'a>>) -> Self {
        Self(functions)
    }

    /// Gets the values.
    pub fn values(&self) -> &FunctionTypedList<'a, AtomicFunctionSkeleton<'a>> {
        &self.0
    }
}

impl<'a> Deref for Functions<'a> {
    type Target = FunctionTypedList<'a, AtomicFunctionSkeleton<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<FunctionTypedList<'a, AtomicFunctionSkeleton<'a>>> for Functions<'a> {
    fn from(value: FunctionTypedList<'a, AtomicFunctionSkeleton<'a>>) -> Self {
        Functions::new(value)
    }
}

impl<'a> FromIterator<FunctionTyped<'a, AtomicFunctionSkeleton<'a>>> for Functions<'a> {
    fn from_iter<T: IntoIterator<Item = FunctionTyped<'a, AtomicFunctionSkeleton<'a>>>>(
        iter: T,
    ) -> Self {
        Functions::new(FunctionTypedList::from_iter(iter))
    }
}
