//! Benchmarks and cross-checks for planning code built from PDDL.
//!
//! [`GroundTask`] is the runtime baseline that generated code is measured
//! against: [`pddl::sas_plus::ground`] grounds a domain and problem the way
//! Fast Downward's translator does, and `GroundTask` runs the result on
//! bit-packed states with precomputed masks. [`fixture`] loads the shared
//! test inputs, and [`domains`] holds the fixture domains as generated code.
//!
//! This crate is not published. See `docs/design/codegen.md` for why the
//! baseline lives here and not in `miniplan`.

pub mod check;
pub mod domains;

use pddl::sas_plus::fdr::SasFile;
use pddl::sas_plus::{ground, GroundError};
use pddl::{Domain, Parser, Problem};
use pddl_runtime::Task;

/// The domain and problem of a fixture in `tests/data/sas-plus`, e.g.
/// `"gripper-2"`.
///
/// # Panics
/// If the fixture doesn't exist or doesn't parse.
#[must_use]
pub fn fixture(name: &str) -> (Domain, Problem) {
    let dir = format!(
        "{}/../tests/data/sas-plus/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let read = |file: &str| {
        std::fs::read_to_string(format!("{dir}/{file}"))
            .unwrap_or_else(|e| panic!("{dir}/{file}: {e}"))
    };
    (
        Domain::from_str(&read("domain.pddl")).expect("the fixture's domain parses"),
        Problem::from_str(&read("problem.pddl")).expect("the fixture's problem parses"),
    )
}

/// A bit mask over the binary variables of a task, one bit per variable.
type Mask = Box<[u64]>;

/// A conditional effect: where its conditions hold in the state before the
/// action, it sets one variable.
struct ConditionalEffect {
    /// Variables that must be true.
    when_true: Mask,
    /// Variables that must be false.
    when_false: Mask,
    /// The variable it sets.
    var: usize,
    /// Whether it sets the variable to true.
    value: bool,
}

/// A ground action as masks.
struct Operator {
    /// Variables that must be true.
    pre_true: Mask,
    /// Variables that must be false.
    pre_false: Mask,
    /// Variables the action sets to true, unconditionally.
    add: Mask,
    /// Variables the action sets to false, unconditionally.
    del: Mask,
    /// The conditional effects.
    conditional: Vec<ConditionalEffect>,
    /// The cost.
    cost: u32,
}

/// A grounded planning task on bit-packed states: the runtime baseline.
///
/// Every variable of the [`SasFile`] that [`ground`] writes is binary, with
/// value 0 for "the atom holds", so a state is one bit per variable.
/// Successors come from one pass over all operators, each tested with a few
/// mask operations.
pub struct GroundTask {
    /// The grounded task, for operator names.
    file: SasFile,
    /// The operators, in the order of `file.operators`.
    operators: Vec<Operator>,
    /// The initial state.
    initial: Mask,
    /// Variables the goal needs true.
    goal_true: Mask,
    /// Variables the goal needs false.
    goal_false: Mask,
}

/// Whether every bit of `mask` is set in `state`.
fn all(state: &[u64], mask: &[u64]) -> bool {
    state.iter().zip(mask).all(|(s, m)| s & m == *m)
}

/// Whether no bit of `mask` is set in `state`.
fn none(state: &[u64], mask: &[u64]) -> bool {
    state.iter().zip(mask).all(|(s, m)| s & m == 0)
}

impl GroundTask {
    /// Grounds `domain` and `problem` with [`ground`].
    ///
    /// # Errors
    /// The [`GroundError`] of [`ground`].
    pub fn new(domain: &Domain, problem: &Problem) -> Result<Self, GroundError> {
        Ok(Self::from_sas(ground(domain, problem)?))
    }

    /// Packs a task whose variables are all binary, as [`ground`] writes them.
    ///
    /// # Panics
    /// If a variable has more than two values.
    #[must_use]
    pub fn from_sas(file: SasFile) -> Self {
        assert!(
            file.variables.iter().all(|v| v.values.len() == 2),
            "every variable must be binary"
        );
        let words = file.variables.len().div_ceil(64);
        let empty = || vec![0u64; words].into_boxed_slice();
        let set = |mask: &mut Mask, var: usize| mask[var / 64] |= 1 << (var % 64);
        // Value 0 is "the atom holds".
        let split = |facts: &mut dyn Iterator<Item = (usize, u32)>| {
            let (mut t, mut f) = (empty(), empty());
            for (var, value) in facts {
                set(if value == 0 { &mut t } else { &mut f }, var);
            }
            (t, f)
        };

        let operators = file
            .operators
            .iter()
            .map(|op| {
                let mut pre = op
                    .prevail
                    .iter()
                    .map(|f| (f.var, f.value))
                    .chain(op.effects.iter().filter_map(|e| Some((e.var, e.pre?))));
                let (pre_true, pre_false) = split(&mut pre);
                let (mut add, mut del) = (empty(), empty());
                let mut conditional = Vec::new();
                for effect in &op.effects {
                    if effect.conditions.is_empty() {
                        set(
                            if effect.post == 0 { &mut add } else { &mut del },
                            effect.var,
                        );
                    } else {
                        let (when_true, when_false) =
                            split(&mut effect.conditions.iter().map(|f| (f.var, f.value)));
                        conditional.push(ConditionalEffect {
                            when_true,
                            when_false,
                            var: effect.var,
                            value: effect.post == 0,
                        });
                    }
                }
                Operator {
                    pre_true,
                    pre_false,
                    add,
                    del,
                    conditional,
                    cost: op.cost,
                }
            })
            .collect();
        let (initial, _) = split(&mut file.initial.iter().copied().enumerate());
        let (goal_true, goal_false) = split(&mut file.goal.iter().map(|f| (f.var, f.value)));
        Self {
            file,
            operators,
            initial,
            goal_true,
            goal_false,
        }
    }

    /// The name of operator `op`, e.g. `pick ball1 rooma left`.
    #[must_use]
    pub fn name(&self, op: u32) -> &str {
        &self.file.operators[op as usize].name
    }

    /// The number of operators.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operators.len()
    }

    /// Whether the task has no operators.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operators.is_empty()
    }
}

impl Task for GroundTask {
    type State = Mask;
    type Action = u32;

    fn initial_state(&self) -> Mask {
        self.initial.clone()
    }

    fn successors(&self, state: &Mask, out: &mut Vec<(u32, Mask)>) {
        for (i, op) in self.operators.iter().enumerate() {
            if all(state, &op.pre_true) && none(state, &op.pre_false) {
                let i = u32::try_from(i).expect("fewer than 2^32 operators");
                out.push((i, self.apply(state, i)));
            }
        }
    }

    fn apply(&self, state: &Mask, action: u32) -> Mask {
        let op = &self.operators[action as usize];
        let mut next: Mask = state
            .iter()
            .zip(op.add.iter().zip(op.del.iter()))
            .map(|(s, (a, d))| (s & !d) | a)
            .collect();
        // Conditions read the state before the action.
        for effect in &op.conditional {
            if all(state, &effect.when_true) && none(state, &effect.when_false) {
                let bit = 1 << (effect.var % 64);
                if effect.value {
                    next[effect.var / 64] |= bit;
                } else {
                    next[effect.var / 64] &= !bit;
                }
            }
        }
        next
    }

    fn is_goal(&self, state: &Mask) -> bool {
        all(state, &self.goal_true) && none(state, &self.goal_false)
    }

    fn cost(&self, action: u32) -> u32 {
        self.operators[action as usize].cost
    }

    fn unsatisfied_goals(&self, state: &Mask) -> u32 {
        let missing = |s: u64, t: u64, f: u64| (t & !s).count_ones() + (f & s).count_ones();
        state
            .iter()
            .zip(self.goal_true.iter().zip(self.goal_false.iter()))
            .map(|(&s, (&t, &f))| missing(s, t, f))
            .sum()
    }
}
