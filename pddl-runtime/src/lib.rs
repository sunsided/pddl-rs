//! The planning-task trait and the fact storage that code generated from PDDL
//! builds on.
//!
//! - [`Task`] is what planners search: an initial state, successors, a goal
//!   test and action costs. Planners are generic over it and monomorphize per
//!   domain; there is no `dyn` on the hot path.
//! - [`Id`] numbers the objects of one PDDL type. [`BitSet`], [`BitRelation`]
//!   and [`BitTable`] store the facts of unary, binary and n-ary predicates over
//!   such ids, and [`Values`] stores numeric fluents. All of them are cheap to
//!   clone and hash, as states in a search must be.
//! - The `search` feature adds reference searches over any [`Task`]: blind
//!   breadth-first search, A* and IDA*, and plan replay.
//! - [`Formula`] and [`Expr`] are ground goals that a task evaluates at run
//!   time, since a problem's goal is only known when the problem loads.
//!
//! This crate does not depend on the PDDL parser.
//!
//! ## Example
//! ```
//! use pddl_runtime::{BitSet, Task};
//!
//! /// Switches that can be turned on, one at a time; the goal is all on.
//! struct Switches(usize);
//!
//! impl Task for Switches {
//!     type State = BitSet<usize>;
//!     type Action = usize;
//!
//!     fn initial_state(&self) -> Self::State {
//!         BitSet::new(self.0)
//!     }
//!
//!     fn successors(&self, state: &Self::State, out: &mut Vec<(usize, Self::State)>) {
//!         for switch in (0..self.0).filter(|&s| !state.contains(s)) {
//!             out.push((switch, self.apply(state, switch)));
//!         }
//!     }
//!
//!     fn apply(&self, state: &Self::State, switch: usize) -> Self::State {
//!         let mut next = state.clone();
//!         next.insert(switch);
//!         next
//!     }
//!
//!     fn is_goal(&self, state: &Self::State) -> bool {
//!         state.count() == self.0
//!     }
//! }
//!
//! let task = Switches(3);
//! let mut out = Vec::new();
//! task.successors(&task.initial_state(), &mut out);
//! assert_eq!(out.len(), 3);
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

use std::fmt::Debug;
use std::hash::Hash;

mod bits;
mod formula;
#[cfg(feature = "search")]
pub mod search;
mod values;

pub use bits::{BitRelation, BitSet, BitTable};
pub use formula::{Comparison, Expr, Formula};
pub use values::Values;

/// A planning task: states, actions that lead from one state to the next, and
/// a goal.
///
/// Planners are generic over it, so each domain gets its own monomorphized
/// search loop. States must be cheap to clone, compare and hash, because a
/// search keeps sets of visited states.
pub trait Task {
    /// A state of the world.
    type State: Clone + Eq + Hash;
    /// A ground action, e.g. `Action::Stack { x: Block(0), y: Block(1) }`.
    type Action: Copy + Eq + Hash + Debug;

    /// The state the task starts in.
    fn initial_state(&self) -> Self::State;

    /// Appends every action applicable in `state`, with the state it leads
    /// to, to `out`.
    fn successors(&self, state: &Self::State, out: &mut Vec<(Self::Action, Self::State)>);

    /// The state that `action` leads to from `state`. The action must be
    /// applicable in `state`.
    fn apply(&self, state: &Self::State, action: Self::Action) -> Self::State;

    /// Whether `state` satisfies the goal.
    fn is_goal(&self, state: &Self::State) -> bool;

    /// The cost of `action`; 1 unless the task has action costs.
    fn cost(&self, action: Self::Action) -> u32 {
        let _ = action;
        1
    }

    /// The number of goal conditions that `state` does not satisfy, as a
    /// heuristic. Not admissible in general: one action can satisfy several.
    ///
    /// Defaults to 0 for a goal state and 1 otherwise.
    fn unsatisfied_goals(&self, state: &Self::State) -> u32 {
        u32::from(!self.is_goal(state))
    }
}

/// The number of an object among the objects of one type, used to index fact
/// storage. Generated code defines one id type per PDDL type, e.g.
/// `struct Block(u32)`, so ids of different types don't mix.
pub trait Id: Copy + Eq + Ord + Hash + Debug {
    /// The id's index, from 0.
    fn index(self) -> usize;

    /// The id with the given index.
    fn from_index(index: usize) -> Self;
}

impl Id for usize {
    fn index(self) -> usize {
        self
    }

    fn from_index(index: usize) -> Self {
        index
    }
}

impl Id for u32 {
    fn index(self) -> usize {
        self as usize
    }

    /// # Panics
    /// If `index` does not fit in a `u32`.
    fn from_index(index: usize) -> Self {
        u32::try_from(index).expect("an id fits in a u32")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One switch to turn on; nothing overrides the defaults.
    struct OneSwitch;

    impl Task for OneSwitch {
        type State = bool;
        type Action = ();

        fn initial_state(&self) -> bool {
            false
        }

        fn successors(&self, state: &bool, out: &mut Vec<((), bool)>) {
            if !state {
                out.push(((), true));
            }
        }

        fn apply(&self, _: &bool, (): ()) -> bool {
            true
        }

        fn is_goal(&self, state: &bool) -> bool {
            *state
        }
    }

    #[test]
    fn defaults_cost_one_and_count_the_goal_once() {
        assert_eq!(OneSwitch.cost(()), 1);
        assert_eq!(OneSwitch.unsatisfied_goals(&false), 1);
        assert_eq!(OneSwitch.unsatisfied_goals(&true), 0);
    }

    #[test]
    fn integer_ids_are_their_own_index() {
        assert_eq!(u32::from_index(7).index(), 7);
        assert_eq!(usize::from_index(7).index(), 7);
    }

    #[test]
    #[should_panic(expected = "an id fits in a u32")]
    fn u32_ids_reject_large_indices() {
        let _ = u32::from_index(usize::MAX);
    }
}
