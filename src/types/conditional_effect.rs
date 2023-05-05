//! Contains conditional effects via the [`ConditionalEffect`] type.

use crate::types::iterators::FlatteningIntoIterator;
use crate::types::PEffect;

/// A conditional effect as used by [`CEffect::When`](crate::types::CEffect::When) and [`TimedEffect::Conditional`](crate::types::TimedEffect::Conditional).
///
/// ## Usage
/// Used by [`CEffect`](crate::CEffect) and [`TimedEffect`](crate::types::TimedEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalEffect {
    /// Exactly the specified effect applies.
    Single(PEffect), // TODO: Unify with `All`; vector is allowed to be empty.
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<PEffect>),
}

impl ConditionalEffect {
    pub const fn new(effect: PEffect) -> Self {
        Self::Single(effect)
    }
    pub const fn new_and(effect: Vec<PEffect>) -> Self {
        Self::All(effect)
    }
}

impl IntoIterator for ConditionalEffect {
    type Item = PEffect;
    type IntoIter = FlatteningIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ConditionalEffect::Single(item) => FlatteningIntoIterator::new(item),
            ConditionalEffect::All(vec) => FlatteningIntoIterator::new_vec(vec),
        }
    }
}

impl From<PEffect> for ConditionalEffect {
    fn from(value: PEffect) -> Self {
        ConditionalEffect::new(value)
    }
}

impl From<Vec<PEffect>> for ConditionalEffect {
    fn from(value: Vec<PEffect>) -> Self {
        ConditionalEffect::new_and(value)
    }
}

impl FromIterator<PEffect> for ConditionalEffect {
    fn from_iter<T: IntoIterator<Item = PEffect>>(iter: T) -> Self {
        ConditionalEffect::new_and(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::Span;
    use crate::Parser;

    #[test]
    fn flatten_with_single_element_works() {
        let (_, effect_a) = PEffect::parse(Span::new("(= x y)")).unwrap();

        let mut iter = ConditionalEffect::new(effect_a).into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn flatten_with_many_elements_works() {
        let (_, effect_a) = PEffect::parse(Span::new("(= x y)")).unwrap();
        let (_, effect_b) = PEffect::parse(Span::new("(assign fun-sym 1.23)")).unwrap();

        let mut iter = ConditionalEffect::from_iter([effect_a, effect_b]).into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }
}
