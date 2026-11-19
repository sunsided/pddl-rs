//! Cross-checks of one task implementation against another.

use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;

use pddl_runtime::Task;

/// Walks two tasks for the same problem in lockstep, breadth first, and
/// checks that every state has the same applicable actions in both, by name.
/// Visits at most `limit` states and returns how many it visited.
///
/// Actions that lead back to the same state are left out on both sides: the
/// grounder drops operators whose effects can't change the state, as Fast
/// Downward's translator does, while generated code keeps them.
///
/// `name_a` and `name_b` render an action as its PDDL name, e.g. `pick b1 rooma left`.
///
/// # Panics
/// If the successors differ, with the path to the state where they do.
pub fn lockstep<A: Task, B: Task>(
    a: &A,
    name_a: impl Fn(A::Action) -> String,
    b: &B,
    name_b: impl Fn(B::Action) -> String,
    limit: usize,
) -> usize
where
    A::State: Debug,
{
    let mut seen = HashSet::from([a.initial_state()]);
    let mut queue = VecDeque::from([(a.initial_state(), b.initial_state(), Vec::<String>::new())]);
    let (mut out_a, mut out_b) = (Vec::new(), Vec::new());
    let mut visited = 0;
    while let Some((sa, sb, path)) = queue.pop_front() {
        if visited == limit {
            break;
        }
        visited += 1;
        assert_eq!(
            a.is_goal(&sa),
            b.is_goal(&sb),
            "goal test differs after {path:?}"
        );
        out_a.clear();
        out_b.clear();
        a.successors(&sa, &mut out_a);
        b.successors(&sb, &mut out_b);
        let mut next_a: Vec<(String, A::State)> = out_a
            .drain(..)
            .filter(|(_, s)| *s != sa)
            .map(|(x, s)| (name_a(x), s))
            .collect();
        let mut next_b: Vec<(String, B::State)> = out_b
            .drain(..)
            .filter(|(_, s)| *s != sb)
            .map(|(x, s)| (name_b(x), s))
            .collect();
        next_a.sort_by(|x, y| x.0.cmp(&y.0));
        next_b.sort_by(|x, y| x.0.cmp(&y.0));
        let listed_a: Vec<&str> = next_a.iter().map(|(n, _)| n.as_str()).collect();
        let listed_b: Vec<&str> = next_b.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(listed_a, listed_b, "successors differ after {path:?}");
        for ((name, na), (_, nb)) in next_a.into_iter().zip(next_b) {
            if seen.insert(na.clone()) {
                let mut path = path.clone();
                path.push(name);
                queue.push_back((na, nb, path));
            }
        }
    }
    visited
}
