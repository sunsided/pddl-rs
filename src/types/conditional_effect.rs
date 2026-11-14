//! Contains effect conditions via the [`EffectCondition`] type.

use crate::types::iterators::FlatteningIntoIterator;
use crate::types::PrimitiveEffect;

/// An effect condition as used by [`WhenConditionalEffect::effect`](crate::types::WhenConditionalEffect::effect) and [`TimedEffect::Conditional`](crate::types::TimedEffect::Conditional).
///
/// # BNF
/// Corresponds to `<conditional-effect>` in the PDDL 3.1 specification.
///
/// ## Usage
/// Used by [`WhenConditionalEffect`](crate::WhenConditionalEffect) and [`TimedEffect`](crate::types::TimedEffect).
#[doc(alias("conditional-effect"))]
#[doc(alias("ConditionalEffect"))]
#[derive(Debug, Clone, PartialEq)]
pub enum EffectCondition {
    /// Exactly the specified effect applies.
    Single(PrimitiveEffect), // TODO: Unify with `All`; vector is allowed to be empty.
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<PrimitiveEffect>),
}

impl EffectCondition {
    /// Creates an effect condition consisting of exactly one primitive effect.
    pub const fn new(effect: PrimitiveEffect) -> Self {
        Self::Single(effect)
    }

    /// Creates a conjunction of primitive effects, `(and <p-effect>*)`.
    pub const fn all(effect: Vec<PrimitiveEffect>) -> Self {
        Self::All(effect)
    }

    /// Creates a conjunction of primitive effects, `(and <p-effect>*)`.
    #[deprecated(since = "0.2.0", note = "Use `all` instead")]
    pub const fn new_and(effect: Vec<PrimitiveEffect>) -> Self {
        Self::all(effect)
    }
}

impl IntoIterator for EffectCondition {
    type Item = PrimitiveEffect;
    type IntoIter = FlatteningIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            EffectCondition::Single(item) => FlatteningIntoIterator::new(item),
            EffectCondition::All(vec) => FlatteningIntoIterator::new_vec(vec),
        }
    }
}

impl From<PrimitiveEffect> for EffectCondition {
    fn from(value: PrimitiveEffect) -> Self {
        EffectCondition::new(value)
    }
}

impl From<Vec<PrimitiveEffect>> for EffectCondition {
    fn from(value: Vec<PrimitiveEffect>) -> Self {
        EffectCondition::all(value)
    }
}

impl FromIterator<PrimitiveEffect> for EffectCondition {
    fn from_iter<T: IntoIterator<Item = PrimitiveEffect>>(iter: T) -> Self {
        EffectCondition::all(iter.into_iter().collect())
    }
}

#[cfg(all(test, feature = "parser"))]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn flatten_with_single_element_works() {
        let effect_a = PrimitiveEffect::from_str("(= x y)").unwrap();

        let mut iter = EffectCondition::new(effect_a).into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn flatten_with_many_elements_works() {
        let effect_a = PrimitiveEffect::from_str("(= x y)").unwrap();
        let effect_b = PrimitiveEffect::from_str("(assign fun-sym 1.23)").unwrap();

        let mut iter = EffectCondition::from_iter([effect_a, effect_b]).into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }
}
