//! Provides function requirements.

use crate::types::Requirement;
use std::collections::HashSet;
use std::ops::Deref;

/// A set of domain requirements.
///
/// ## Example
/// ```
/// # use pddl::types::{Requirement, Requirements};
/// let requirements = Requirements::new([
///     Requirement::Adl,
///     Requirement::Strips,
///     Requirement::Constraints]);
///
/// // Three requirements were actively specified.
/// assert_eq!(requirements.len(), 3);
///
/// // The effective set of requirements contains eight values:
/// //  :adl expands to seven values, including :strips;
/// //  :constraints is not part of :adl and therefore added.
/// let effective = requirements.to_effective();
/// assert_eq!(effective.len(), 8);
///
/// // These are the effective values:
/// assert!(effective.contains(&Requirement::Strips));
/// assert!(effective.contains(&Requirement::Typing));
/// assert!(effective.contains(&Requirement::NegativePreconditions));
/// assert!(effective.contains(&Requirement::DisjunctivePreconditions));
/// assert!(effective.contains(&Requirement::Equality));
/// assert!(effective.contains(&Requirement::QuantifiedPreconditions));
/// assert!(effective.contains(&Requirement::ConditionalEffects));
/// assert!(effective.contains(&Requirement::Constraints));
/// ```
///
/// If no requirements are specified, [`Requirements::to_effective`] implicitly adds `:strips`:
/// ```
/// # use pddl::types::{Requirement, Requirements};
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
