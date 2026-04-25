//! Contains effects via the [`Effects`] type.

use crate::types::ConditionalEffect;
use std::ops::Deref;

/// An effect. Occurs e.g. in a [`ActionDefinition`](crate::types::ActionDefinition).
///
/// This represents the `(and ...)` variant of the PDDL definition,
/// modeling cases of zero, one or many effects.
///
/// ## Usage
/// Used by [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Effects(Vec<ConditionalEffect>);

impl Effects {
    /// Constructs a new instance from the value.
    pub fn new(effect: ConditionalEffect) -> Self {
        Self(vec![effect])
    }

    /// Constructs a new instance from the provided vector of values.
    #[doc(alias = "new_and")]
    pub fn and(effects: Vec<ConditionalEffect>) -> Self {
        Self(effects)
    }

    pub fn new_and(effects: Vec<ConditionalEffect>) -> Self {
        Self::and(effects)
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of elements in the list, also referred to
    /// as its 'length'.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the list.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> std::slice::Iter<'_, ConditionalEffect> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<ConditionalEffect> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl IntoIterator for Effects {
    type Item = ConditionalEffect;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Effects {
    type Target = [ConditionalEffect];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[ConditionalEffect]> for Effects {
    fn as_ref(&self) -> &[ConditionalEffect] {
        self.0.as_slice()
    }
}

impl From<ConditionalEffect> for Effects {
    fn from(value: ConditionalEffect) -> Self {
        Effects::new(value)
    }
}

impl From<Vec<ConditionalEffect>> for Effects {
    fn from(value: Vec<ConditionalEffect>) -> Self {
        Effects::and(value)
    }
}

impl FromIterator<ConditionalEffect> for Effects {
    fn from_iter<T: IntoIterator<Item = ConditionalEffect>>(iter: T) -> Self {
        Effects::and(iter.into_iter().collect())
    }
}

impl TryInto<ConditionalEffect> for Effects {
    type Error = ();

    fn try_into(self) -> Result<ConditionalEffect, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AtomicFormula, PrimitiveEffect, Term};

    #[test]
    fn effects_try_get_single_with_one() {
        let pe = PrimitiveEffect::new(AtomicFormula::new_equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));
        let effects = Effects::new(ConditionalEffect::new_primitive_effect(pe));
        assert!(effects.try_get_single().is_some());
    }

    #[test]
    fn effects_try_get_single_with_zero() {
        let effects = Effects::default();
        assert!(effects.try_get_single().is_none());
    }

    #[test]
    fn effects_try_get_single_with_many() {
        let pe1 = PrimitiveEffect::new(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));
        let pe2 = PrimitiveEffect::new(AtomicFormula::equality(
            Term::Name("a".into()),
            Term::Name("b".into()),
        ));
        let effects = Effects::and(vec![
            ConditionalEffect::new_primitive_effect(pe1),
            ConditionalEffect::new_primitive_effect(pe2),
        ]);
        assert!(effects.try_get_single().is_none());
    }

    #[test]
    fn effects_try_into() {
        let pe = PrimitiveEffect::new(AtomicFormula::new_equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));
        let effects = Effects::new(ConditionalEffect::new_primitive_effect(pe));
        let result: Result<ConditionalEffect, ()> = effects.try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn effects_from_vec() {
        let _ = Effects::from(Vec::<ConditionalEffect>::new());
    }
}
