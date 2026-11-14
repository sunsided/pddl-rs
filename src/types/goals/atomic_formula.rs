//! Contains atomic formulae via the [`AtomicFormula`] type.

use crate::types::Predicate;
use std::ops::Deref;

/// An atomic formula.
///
/// ## Usage
/// Used by [`Literal`](crate::Literal), [`GoalDescription`](crate::GoalDescription) and
/// [`PrimitiveEffect`](crate::PrimitiveEffect).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AtomicFormula<T> {
    /// An equality between two values, `(= <t> <t>)`.
    ///
    /// ## Requirements
    /// Requires [Equality](crate::Requirement::Equality).
    Equality(EqualityAtomicFormula<T>),
    /// A predicate applied to its arguments, `(<predicate> <t>*)`.
    Predicate(PredicateAtomicFormula<T>),
}

impl<T> AtomicFormula<T> {
    /// Creates an equality formula `(= first second)`.
    #[doc(alias = "new_equality")]
    #[must_use]
    pub const fn equality(first: T, second: T) -> Self {
        Self::Equality(EqualityAtomicFormula::new(first, second))
    }

    /// Alias for [`Self::equality`].
    #[must_use]
    pub fn new_equality(first: T, second: T) -> Self {
        Self::equality(first, second)
    }

    /// Creates a predicate formula `(<predicate> <values>*)`.
    #[doc(alias = "new_predicate")]
    #[must_use]
    pub fn predicate<V: IntoIterator<Item = T>>(predicate: Predicate, values: V) -> Self {
        Self::Predicate(PredicateAtomicFormula::new(
            predicate,
            values.into_iter().collect(),
        ))
    }

    /// Alias for [`Self::predicate`].
    #[must_use]
    pub fn new_predicate<V: IntoIterator<Item = T>>(predicate: Predicate, values: V) -> Self {
        Self::predicate(predicate, values)
    }
}

/// An equality atomic formula, `(= <t> <t>)`.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct EqualityAtomicFormula<T> {
    /// The left-hand operand.
    first: T,
    /// The right-hand operand.
    second: T,
}

/// A predicate atomic formula, `(<predicate> <t>*)`.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct PredicateAtomicFormula<T> {
    /// The predicate.
    predicate: Predicate,
    /// The arguments the predicate is applied to.
    values: Vec<T>,
}

impl<T> EqualityAtomicFormula<T> {
    /// Creates an equality formula `(= first second)`.
    #[must_use]
    pub const fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    /// Gets a reference to the first element.
    #[must_use]
    pub const fn first(&self) -> &T {
        &self.first
    }

    /// Gets a reference to the second element.
    #[must_use]
    pub const fn second(&self) -> &T {
        &self.second
    }
}

impl<T> PredicateAtomicFormula<T> {
    /// Creates a predicate formula from a predicate and its arguments.
    #[must_use]
    pub const fn new(predicate: Predicate, values: Vec<T>) -> Self {
        Self { predicate, values }
    }

    /// Returns the predicate.
    #[must_use]
    pub const fn predicate(&self) -> &Predicate {
        &self.predicate
    }

    /// Gets a reference to the values.
    #[must_use]
    pub fn values(&self) -> &[T] {
        self.values.as_slice()
    }
}

impl<T> From<EqualityAtomicFormula<T>> for AtomicFormula<T> {
    fn from(value: EqualityAtomicFormula<T>) -> Self {
        AtomicFormula::Equality(value)
    }
}

impl<T> From<PredicateAtomicFormula<T>> for AtomicFormula<T> {
    fn from(value: PredicateAtomicFormula<T>) -> Self {
        AtomicFormula::Predicate(value)
    }
}

impl<T> From<(T, T)> for EqualityAtomicFormula<T> {
    fn from(value: (T, T)) -> Self {
        EqualityAtomicFormula::new(value.0, value.1)
    }
}

impl<T> From<(Predicate, Vec<T>)> for PredicateAtomicFormula<T> {
    fn from(value: (Predicate, Vec<T>)) -> Self {
        PredicateAtomicFormula::new(value.0, value.1)
    }
}

impl<T> Deref for PredicateAtomicFormula<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.values()
    }
}
