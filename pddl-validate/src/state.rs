//! The world state a plan runs through.

use alloc::collections::BTreeSet;

use crate::atom::Atom;

/// A world state under the closed-world assumption: every atom not listed is false.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct State {
    /// The true atoms of basic (non-derived) predicates.
    pub facts: BTreeSet<Atom>,
    /// The true atoms of derived predicates, computed from `facts` by
    /// [`derived::close`](crate::derived::close) after every change.
    pub derived: BTreeSet<Atom>,
}
