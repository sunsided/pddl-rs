//! Interference between events (PDDL 2.1 mutex).
//!
//! Two events interfere if one changes an atom or fluent that the other reads, if one
//! adds an atom the other deletes, or if both update one fluent other than by
//! `increase`/`decrease`. Interfering events must be at least epsilon apart. A durative
//! action's start and end update the fluents of its continuous effects like an
//! `increase`, since the change starts or stops there.

use alloc::collections::{BTreeMap, BTreeSet};

use pddl::AssignOp;

use crate::atom::Atom;
use crate::effects::Changes;
use crate::eval::Observer;

/// What one event reads and writes.
#[derive(Debug, Default, Clone)]
pub struct Footprint {
    /// The atoms its conditions read.
    atoms_read: BTreeSet<Atom>,
    /// The fluents its conditions and right-hand sides read.
    fluents_read: BTreeSet<Atom>,
    /// The atoms it adds.
    adds: BTreeSet<Atom>,
    /// The atoms it deletes.
    deletes: BTreeSet<Atom>,
    /// The fluents it updates, with `true` if every update is `increase` or `decrease`.
    updates: BTreeMap<Atom, bool>,
}

impl Footprint {
    /// Builds the footprint of an event that read what `observer` recorded and makes
    /// `changes`.
    pub fn new(observer: &Observer, changes: &Changes) -> Self {
        let mut updates: BTreeMap<Atom, bool> = changes
            .updates
            .iter()
            .map(|(fluent, ops)| {
                let additive = ops
                    .iter()
                    .all(|(op, _)| matches!(op, AssignOp::Increase | AssignOp::Decrease));
                (fluent.clone(), additive)
            })
            .collect();
        for fluent in &changes.continuous {
            updates.entry(fluent.clone()).or_insert(true);
        }

        Self {
            atoms_read: observer.atoms.clone(),
            fluents_read: observer.fluents.clone(),
            adds: changes.add.clone(),
            deletes: changes.delete.clone(),
            updates,
        }
    }

    /// Returns `true` if the two events interfere.
    pub fn interferes(&self, other: &Self) -> bool {
        let shared_update = self.updates.iter().any(|(fluent, additive)| {
            other
                .updates
                .get(fluent)
                .is_some_and(|other_additive| !(*additive && *other_additive))
        });

        shared_update || self.disturbs(other) || other.disturbs(self)
    }

    /// Returns `true` if this event changes what `other` reads, or undoes what it adds.
    fn disturbs(&self, other: &Self) -> bool {
        let changed = self.adds.iter().chain(&self.deletes);
        changed.clone().any(|atom| other.atoms_read.contains(atom))
            || self.updates.keys().any(|f| other.fluents_read.contains(f))
            || self.deletes.iter().any(|atom| other.adds.contains(atom))
    }
}
