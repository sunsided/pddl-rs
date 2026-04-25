//! Provides function requirements.

use crate::types::Requirement;
use std::collections::HashSet;
use std::ops::Deref;

/// A set of domain requirements.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain) and [`Problem`](crate::Problem).
///
/// ## Example
/// ```
/// # use pddl::{Requirement, Requirements};
/// let requirements = Requirements::new([
///     Requirement::Adl,
///     Requirement::Strips,
///     Requirement::Constraints]);
///
/// // Three requirements were actively specified.
/// assert_eq!(requirements.len(), 3);
///
/// // The effective set of requirements contains eight values:
/// //  :adl expands to eight values, including :strips;
/// //  :constraints is not part of :adl and therefore added.
/// let effective = requirements.to_effective();
/// assert_eq!(effective.len(), 9);
///
/// // These are the effective values:
/// assert!(effective.contains(&Requirement::Strips));
/// assert!(effective.contains(&Requirement::Typing));
/// assert!(effective.contains(&Requirement::NegativePreconditions));
/// assert!(effective.contains(&Requirement::DisjunctivePreconditions));
/// assert!(effective.contains(&Requirement::Equality));
/// assert!(effective.contains(&Requirement::ExistentialPreconditions));
/// assert!(effective.contains(&Requirement::UniversalPreconditions));
/// assert!(effective.contains(&Requirement::ConditionalEffects));
/// assert!(effective.contains(&Requirement::Constraints));
/// ```
///
/// If no requirements are specified, [`Requirements::to_effective`] implicitly adds `:strips`:
/// ```
/// # use pddl::{Requirement, Requirements};
/// let requirements = Requirements::default();
/// assert_eq!(requirements.len(), 0);
///
/// let effective = requirements.to_effective();
/// assert_eq!(effective.len(), 1);
/// assert!(effective.contains(&Requirement::Strips));
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Requirements(Vec<Requirement>);

impl Requirements {
    /// Constructs a new set of requirements from the specified values.
    ///
    /// ## Requirement Expansion
    /// The values will be taken as-is, no deduplication is performed and the
    /// implicit `:strips` requirement will not be added when no values are provided.
    /// To get the effective set of requirements, call [`Requirements::to_effective`].
    pub fn new<I: IntoIterator<Item = Requirement>>(requirements: I) -> Self {
        Self(requirements.into_iter().collect())
    }

    /// Returns the set of effective requirements.
    /// This expands all shorthand requirements such as `:adl` and adds the implicit
    /// `:strips` requirement if no requirements are specified.
    pub fn to_effective(&self) -> HashSet<Requirement> {
        let mut set = HashSet::from_iter(self.iter().flat_map(Requirement::expand));

        // :strips is an implicit requirement that is always given
        // when no requirement is specified.
        if set.is_empty() {
            set.insert(Requirement::Strips);
        }
        set
    }
}

impl Deref for Requirements {
    type Target = Vec<Requirement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Requirement>> for Requirements {
    fn from(value: Vec<Requirement>) -> Self {
        Requirements::new(value)
    }
}

impl FromIterator<Requirement> for Requirements {
    fn from_iter<T: IntoIterator<Item = Requirement>>(iter: T) -> Self {
        Requirements::new(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let reqs = Requirements::new([]);
        assert!(reqs.is_empty());
    }

    #[test]
    fn new_with_values() {
        let reqs = Requirements::new([Requirement::Strips, Requirement::Typing]);
        assert_eq!(reqs.len(), 2);
    }

    #[test]
    fn default_is_empty() {
        let reqs = Requirements::default();
        assert!(reqs.is_empty());
    }

    #[test]
    fn from_vec() {
        let reqs = Requirements::from(vec![Requirement::Adl]);
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0], Requirement::Adl);
    }

    #[test]
    fn from_iterator() {
        let reqs: Requirements = [Requirement::Strips, Requirement::Equality]
            .into_iter()
            .collect();
        assert_eq!(reqs.len(), 2);
    }

    #[test]
    fn deref_to_vec() {
        let reqs = Requirements::new([Requirement::Strips]);
        let vec: &Vec<Requirement> = &reqs;
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn to_effective_empty_adds_strips() {
        let reqs = Requirements::default();
        let effective = reqs.to_effective();
        assert_eq!(effective.len(), 1);
        assert!(effective.contains(&Requirement::Strips));
    }

    #[test]
    fn to_effective_strips_only() {
        let reqs = Requirements::new([Requirement::Strips]);
        let effective = reqs.to_effective();
        assert_eq!(effective.len(), 1);
    }

    #[test]
    fn to_effective_adl_expands() {
        let reqs = Requirements::new([Requirement::Adl]);
        let effective = reqs.to_effective();
        assert!(effective.contains(&Requirement::Strips));
        assert!(effective.contains(&Requirement::Typing));
        assert!(effective.contains(&Requirement::NegativePreconditions));
        assert!(effective.contains(&Requirement::DisjunctivePreconditions));
        assert!(effective.contains(&Requirement::Equality));
        assert!(effective.contains(&Requirement::ExistentialPreconditions));
        assert!(effective.contains(&Requirement::UniversalPreconditions));
        assert!(effective.contains(&Requirement::ConditionalEffects));
    }

    #[test]
    fn to_effective_fluents_expands() {
        let reqs = Requirements::new([Requirement::Fluents]);
        let effective = reqs.to_effective();
        assert!(effective.contains(&Requirement::NumericFluents));
        assert!(effective.contains(&Requirement::ObjectFluents));
    }

    #[test]
    fn to_effective_quantified_expands() {
        let reqs = Requirements::new([Requirement::QuantifiedPreconditions]);
        let effective = reqs.to_effective();
        assert!(effective.contains(&Requirement::ExistentialPreconditions));
        assert!(effective.contains(&Requirement::UniversalPreconditions));
    }

    #[test]
    fn to_effective_combined() {
        let reqs = Requirements::new([Requirement::Adl, Requirement::Constraints]);
        let effective = reqs.to_effective();
        assert_eq!(effective.len(), 9);
        assert!(effective.contains(&Requirement::Constraints));
    }

    #[test]
    fn to_effective_deduplicates() {
        let reqs = Requirements::new([Requirement::Strips, Requirement::Adl, Requirement::Strips]);
        let effective = reqs.to_effective();
        assert!(effective.contains(&Requirement::Strips));
        assert!(effective.contains(&Requirement::Typing));
    }

    #[test]
    fn clone_works() {
        let reqs = Requirements::new([Requirement::Strips]);
        let clone = reqs.clone();
        assert_eq!(reqs, clone);
    }

    #[test]
    fn eq_works() {
        let a = Requirements::new([Requirement::Strips]);
        let b = Requirements::new([Requirement::Strips]);
        let c = Requirements::new([Requirement::Typing]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
