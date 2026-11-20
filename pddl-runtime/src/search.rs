//! Reference searches over any [`Task`]: blind breadth-first search, A* and
//! IDA*, and plan replay.
//!
//! These check that a task's successors are right and measure how fast a task
//! expands states. They are not a production planner.
//!
//! ```
//! use pddl_runtime::search::{astar, breadth_first, replay};
//! # use pddl_runtime::{BitSet, Task};
//! # struct Switches(usize);
//! # impl Task for Switches {
//! #     type State = BitSet<usize>;
//! #     type Action = usize;
//! #     fn initial_state(&self) -> Self::State { BitSet::new(self.0) }
//! #     fn successors(&self, s: &Self::State, out: &mut Vec<(usize, Self::State)>) {
//! #         for a in (0..self.0).filter(|&a| !s.contains(a)) { out.push((a, self.apply(s, a))); }
//! #     }
//! #     fn apply(&self, s: &Self::State, a: usize) -> Self::State { let mut n = s.clone(); n.insert(a); n }
//! #     fn is_goal(&self, s: &Self::State) -> bool { s.count() == self.0 }
//! #     fn unsatisfied_goals(&self, s: &Self::State) -> u32 { (self.0 - s.count()) as u32 }
//! # }
//!
//! let task = Switches(3); // turn on three switches, one per action
//! let plan = breadth_first(&task).plan.unwrap();
//! assert_eq!(plan.cost, 3);
//! assert_eq!(replay(&task, &plan.actions), Some(3));
//!
//! let plan = astar(&task, |s| task.unsatisfied_goals(s)).plan.unwrap();
//! assert_eq!(plan.cost, 3);
//! ```

use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::Task;

/// A sequence of actions that leads from the initial state to a goal state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan<A> {
    /// The actions, in order.
    pub actions: Vec<A>,
    /// The sum of their costs.
    pub cost: u64,
}

/// What a search found and how much work it took.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome<A> {
    /// The plan, or `None` if the task has none.
    pub plan: Option<Plan<A>>,
    /// The number of states whose successors the search generated.
    pub expanded: usize,
}

/// One state the search reached, with how it got there.
struct Node<S, A> {
    /// The state.
    state: S,
    /// The parent node and the action from it, or `None` for the initial state.
    parent: Option<(usize, A)>,
    /// The cost from the initial state.
    g: u64,
}

/// Follows parent links from node `goal` back to the initial state.
fn extract<S, A: Copy>(nodes: &[Node<S, A>], goal: usize) -> Plan<A> {
    let mut actions = Vec::new();
    let mut at = goal;
    while let Some((parent, action)) = nodes[at].parent {
        actions.push(action);
        at = parent;
    }
    actions.reverse();
    Plan {
        actions,
        cost: nodes[goal].g,
    }
}

/// Blind breadth-first search. Finds a plan with the fewest actions, which is
/// an optimal plan when every action costs the same.
pub fn breadth_first<T: Task>(task: &T) -> Outcome<T::Action> {
    let initial = task.initial_state();
    let mut seen: HashSet<T::State> = HashSet::from([initial.clone()]);
    let mut nodes = vec![Node {
        state: initial,
        parent: None,
        g: 0,
    }];
    let mut queue = VecDeque::from([0]);
    let mut expanded = 0;
    let mut successors = Vec::new();
    while let Some(at) = queue.pop_front() {
        if task.is_goal(&nodes[at].state) {
            return Outcome {
                plan: Some(extract(&nodes, at)),
                expanded,
            };
        }
        expanded += 1;
        successors.clear();
        task.successors(&nodes[at].state, &mut successors);
        let g = nodes[at].g;
        for (action, state) in successors.drain(..) {
            if seen.insert(state.clone()) {
                queue.push_back(nodes.len());
                nodes.push(Node {
                    state,
                    parent: Some((at, action)),
                    g: g + u64::from(task.cost(action)),
                });
            }
        }
    }
    Outcome {
        plan: None,
        expanded,
    }
}

/// A* with the heuristic `h`. The plan is optimal if `h` is admissible: it
/// never overestimates the cost to a goal. `|_| 0` makes it uniform-cost
/// search.
pub fn astar<T: Task>(task: &T, h: impl Fn(&T::State) -> u32) -> Outcome<T::Action> {
    let initial = task.initial_state();
    let mut best: HashMap<T::State, usize> = HashMap::from([(initial.clone(), 0)]);
    let mut nodes = vec![Node {
        state: initial,
        parent: None,
        g: 0,
    }];
    // Ordered by f, then by higher g, then by insertion, so ties go deep and
    // the search is deterministic.
    let mut open = BinaryHeap::from([Reverse((u64::from(h(&nodes[0].state)), Reverse(0), 0))]);
    let mut closed = HashSet::new();
    let mut expanded = 0;
    let mut successors = Vec::new();
    while let Some(Reverse((_, _, at))) = open.pop() {
        if !closed.insert(at) {
            continue;
        }
        if task.is_goal(&nodes[at].state) {
            return Outcome {
                plan: Some(extract(&nodes, at)),
                expanded,
            };
        }
        expanded += 1;
        successors.clear();
        task.successors(&nodes[at].state, &mut successors);
        let g = nodes[at].g;
        for (action, state) in successors.drain(..) {
            let g = g + u64::from(task.cost(action));
            let next = nodes.len();
            match best.entry(state) {
                Entry::Occupied(mut entry) => {
                    let old = *entry.get();
                    if g >= nodes[old].g {
                        continue;
                    }
                    // A cheaper path to a known state: a new node replaces it.
                    entry.insert(next);
                    nodes.push(Node {
                        state: nodes[old].state.clone(),
                        parent: Some((at, action)),
                        g,
                    });
                }
                Entry::Vacant(entry) => {
                    nodes.push(Node {
                        state: entry.key().clone(),
                        parent: Some((at, action)),
                        g,
                    });
                    entry.insert(next);
                }
            }
            let f = g + u64::from(h(&nodes[next].state));
            open.push(Reverse((f, Reverse(g), next)));
        }
    }
    Outcome {
        plan: None,
        expanded,
    }
}

/// Iterative-deepening A* with the heuristic `h`: depth-first searches with a
/// growing bound on `g + h`, in memory linear in the plan length. The plan is
/// optimal if `h` is admissible. It never revisits a state on the current path,
/// so it terminates on finite tasks.
pub fn ida_star<T: Task>(task: &T, h: impl Fn(&T::State) -> u32) -> Outcome<T::Action> {
    /// The state of one depth-first search.
    struct Dfs<'a, T: Task, H> {
        /// The task.
        task: &'a T,
        /// The heuristic.
        h: H,
        /// The states on the current path.
        path: HashSet<T::State>,
        /// The actions on the current path.
        actions: Vec<T::Action>,
        /// A successor buffer per depth.
        buffers: Vec<Vec<(T::Action, T::State)>>,
        /// The number of expanded states.
        expanded: usize,
    }

    impl<T: Task, H: Fn(&T::State) -> u32> Dfs<'_, T, H> {
        /// Searches below `state`; returns `Ok(cost)` on a goal, or the
        /// smallest `f` that exceeded `bound`.
        fn search(&mut self, state: &T::State, g: u64, bound: u64) -> Result<u64, Option<u64>> {
            let f = g + u64::from((self.h)(state));
            if f > bound {
                return Err(Some(f));
            }
            if self.task.is_goal(state) {
                return Ok(g);
            }
            self.expanded += 1;
            let depth = self.actions.len();
            if self.buffers.len() <= depth {
                self.buffers.push(Vec::new());
            }
            let mut successors = std::mem::take(&mut self.buffers[depth]);
            successors.clear();
            self.task.successors(state, &mut successors);
            let mut next_bound: Option<u64> = None;
            for (action, next) in &successors {
                if !self.path.insert(next.clone()) {
                    continue;
                }
                self.actions.push(*action);
                let result = self.search(next, g + u64::from(self.task.cost(*action)), bound);
                if result.is_ok() {
                    self.buffers[depth] = successors;
                    return result;
                }
                self.actions.pop();
                self.path.remove(next);
                if let Err(Some(f)) = result {
                    next_bound = Some(next_bound.map_or(f, |b| b.min(f)));
                }
            }
            self.buffers[depth] = successors;
            Err(next_bound)
        }
    }

    let initial = task.initial_state();
    let mut dfs = Dfs {
        task,
        h,
        path: HashSet::from([initial.clone()]),
        actions: Vec::new(),
        buffers: Vec::new(),
        expanded: 0,
    };
    let mut bound = u64::from((dfs.h)(&initial));
    loop {
        match dfs.search(&initial, 0, bound) {
            Ok(cost) => {
                return Outcome {
                    plan: Some(Plan {
                        actions: dfs.actions,
                        cost,
                    }),
                    expanded: dfs.expanded,
                }
            }
            Err(Some(next)) => bound = next,
            Err(None) => {
                return Outcome {
                    plan: None,
                    expanded: dfs.expanded,
                }
            }
        }
    }
}

/// Replays `plan` from the initial state. Returns its cost if every action is
/// applicable where it is taken and the plan ends in a goal state, and `None`
/// otherwise.
///
/// An action counts as applicable if it is among the state's successors, so
/// this checks `successors` against `apply`.
pub fn replay<T: Task>(task: &T, plan: &[T::Action]) -> Option<u64> {
    let mut state = task.initial_state();
    let mut cost = 0;
    let mut successors = Vec::new();
    for &action in plan {
        successors.clear();
        task.successors(&state, &mut successors);
        let (_, next) = successors.iter().find(|(a, _)| *a == action)?;
        if *next != task.apply(&state, action) {
            return None;
        }
        state = next.clone();
        cost += u64::from(task.cost(action));
    }
    task.is_goal(&state).then_some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A weighted graph: from each node, the edges `(target, cost)`; reach the
    /// last node from node 0.
    struct Graph(Vec<Vec<(usize, u32)>>);

    impl Task for Graph {
        type State = usize;
        type Action = (usize, usize);

        fn initial_state(&self) -> usize {
            0
        }

        fn successors(&self, state: &usize, out: &mut Vec<((usize, usize), usize)>) {
            out.extend(self.0[*state].iter().map(|&(to, _)| ((*state, to), to)));
        }

        fn apply(&self, _: &usize, (_, to): (usize, usize)) -> usize {
            to
        }

        fn is_goal(&self, state: &usize) -> bool {
            *state == self.0.len() - 1
        }

        fn cost(&self, (from, to): (usize, usize)) -> u32 {
            self.0[from].iter().find(|(t, _)| *t == to).unwrap().1
        }
    }

    /// 0 -> 3 directly costs 10; 0 -> 1 -> 2 -> 3 costs 3. 1 -> 0 is a cycle.
    fn graph() -> Graph {
        Graph(vec![
            vec![(3, 10), (1, 1)],
            vec![(0, 1), (2, 1)],
            vec![(3, 1)],
            vec![],
        ])
    }

    #[test]
    fn breadth_first_finds_the_fewest_actions() {
        let outcome = breadth_first(&graph());
        let plan = outcome.plan.unwrap();
        assert_eq!(plan.actions, [(0, 3)]);
        assert_eq!(plan.cost, 10);
        assert_eq!(outcome.expanded, 1);
    }

    #[test]
    fn astar_and_ida_star_find_the_cheapest_plan() {
        let task = graph();
        for outcome in [astar(&task, |_| 0), ida_star(&task, |_| 0)] {
            let plan = outcome.plan.unwrap();
            assert_eq!(plan.actions, [(0, 1), (1, 2), (2, 3)]);
            assert_eq!(plan.cost, 3);
            assert_eq!(replay(&task, &plan.actions), Some(3));
        }
    }

    #[test]
    fn unsolvable_tasks_have_no_plan() {
        let task = Graph(vec![vec![(1, 1)], vec![(0, 1)], vec![]]);
        assert_eq!(breadth_first(&task).plan, None);
        assert_eq!(astar(&task, |_| 0).plan, None);
        assert_eq!(ida_star(&task, |_| 0).plan, None);
    }

    #[test]
    fn replay_rejects_inapplicable_and_incomplete_plans() {
        let task = graph();
        assert_eq!(replay(&task, &[(0, 3)]), Some(10));
        assert_eq!(replay(&task, &[(0, 1)]), None);
        assert_eq!(replay(&task, &[(1, 2)]), None);
    }

    #[test]
    fn default_heuristic_is_zero_only_on_goals() {
        let task = graph();
        assert_eq!(task.unsatisfied_goals(&0), 1);
        assert_eq!(task.unsatisfied_goals(&3), 0);
        assert_eq!(task.cost((0, 3)), 10);
    }
}
