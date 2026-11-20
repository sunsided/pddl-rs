//! Level B2: a domain and problem grounded when the crate compiles, as
//! constant mask tables.

use pddl_core::sas_plus::fdr::SasFile;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::{Error, Limits};

/// One operator's masks, one bit per variable.
struct Masks {
    /// Variables that must be true.
    pre_true: Vec<u64>,
    /// Variables that must be false.
    pre_false: Vec<u64>,
    /// Variables set to true.
    add: Vec<u64>,
    /// Variables set to false.
    del: Vec<u64>,
}

/// A mask as an array literal.
fn words(mask: &[u64]) -> TokenStream {
    let words = mask.iter().map(|&w| Literal::u64_suffixed(w));
    quote!([#(#words),*])
}

/// Generates the items of level B2 for a grounded task.
pub fn generate(file: &SasFile, limits: &Limits) -> Result<TokenStream, Error> {
    if file.operators.len() > limits.max_operators {
        return Err(Error::new(format!(
            "the grounded task has {} operators, more than max_operators = {}; \
             raise the limit or plan with the lifted form (no `problem`)",
            file.operators.len(),
            limits.max_operators
        )));
    }
    if file.variables.len() > limits.max_facts {
        return Err(Error::new(format!(
            "the grounded task has {} facts, more than max_facts = {}; \
             raise the limit or plan with the lifted form (no `problem`)",
            file.variables.len(),
            limits.max_facts
        )));
    }
    // `ground` writes binary variables: value 0 means the atom holds.
    let count = file.variables.len().div_ceil(64).max(1);
    let empty = || vec![0u64; count];
    let set = |mask: &mut Vec<u64>, var: usize| mask[var / 64] |= 1 << (var % 64);

    let mut operators = Vec::new();
    for op in &file.operators {
        let mut masks = Masks {
            pre_true: empty(),
            pre_false: empty(),
            add: empty(),
            del: empty(),
        };
        for fact in &op.prevail {
            set(
                if fact.value == 0 {
                    &mut masks.pre_true
                } else {
                    &mut masks.pre_false
                },
                fact.var,
            );
        }
        for effect in &op.effects {
            if !effect.conditions.is_empty() {
                return Err(Error::new(format!(
                    "unsupported in a grounded task: conditional effects (in `{}`); \
                     plan with the lifted form (no `problem`)",
                    op.name
                )));
            }
            if let Some(pre) = effect.pre {
                set(
                    if pre == 0 {
                        &mut masks.pre_true
                    } else {
                        &mut masks.pre_false
                    },
                    effect.var,
                );
            }
            set(
                if effect.post == 0 {
                    &mut masks.add
                } else {
                    &mut masks.del
                },
                effect.var,
            );
        }
        operators.push(masks);
    }
    let mut initial = empty();
    for (var, &value) in file.initial.iter().enumerate() {
        if value == 0 {
            set(&mut initial, var);
        }
    }
    let (mut goal_true, mut goal_false) = (empty(), empty());
    for fact in &file.goal {
        set(
            if fact.value == 0 {
                &mut goal_true
            } else {
                &mut goal_false
            },
            fact.var,
        );
    }

    Ok(items(
        file,
        &operators,
        count,
        [&initial, &goal_true, &goal_false],
    ))
}

/// The items for the masks: the state type, the tables and the task.
fn items(
    file: &SasFile,
    operators: &[Masks],
    count: usize,
    [initial, goal_true, goal_false]: [&[u64]; 3],
) -> TokenStream {
    let tables = operators.iter().zip(&file.operators).map(|(m, op)| {
        let (pre_true, pre_false, add, del) =
            (words(&m.pre_true), words(&m.pre_false), words(&m.add), words(&m.del));
        let cost = op.cost;
        quote!(Operator { pre_true: #pre_true, pre_false: #pre_false, add: #add, del: #del, cost: #cost })
    });
    let names = file.operators.iter().map(|op| format!("({})", op.name));
    let facts = file
        .variables
        .iter()
        .map(|v| v.values[0].trim_start_matches("Atom ").to_string());
    let (initial, goal_true, goal_false) = (words(initial), words(goal_true), words(goal_false));
    let operator_count = file.operators.len();
    let fact_count = file.variables.len();

    quote! {
        /// The number of `u64` words in a state.
        pub const WORDS: usize = #count;

        /// A state: one bit per ground atom that some action changes.
        pub type State = [u64; WORDS];

        /// A ground action, as its index in [`OPERATORS`].
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct Action(pub u32);

        /// A ground action as masks over [`State`].
        #[derive(Debug)]
        pub struct Operator {
            /// The atoms that must hold.
            pub pre_true: State,
            /// The atoms that must not hold.
            pub pre_false: State,
            /// The atoms it adds.
            pub add: State,
            /// The atoms it deletes.
            pub del: State,
            /// The cost: the action's cost under the problem's metric, else 1.
            pub cost: u32,
        }

        /// Every ground action.
        pub static OPERATORS: [Operator; #operator_count] = [#(#tables),*];

        /// Each operator in PDDL syntax, e.g. `(pick ball1 rooma left)`.
        pub static NAMES: [&str; #operator_count] = [#(#names),*];

        /// Each bit's atom, e.g. `at(ball1, rooma)`.
        pub static FACTS: [&str; #fact_count] = [#(#facts),*];

        /// The initial state.
        pub const INITIAL: State = #initial;
        /// The atoms the goal needs.
        pub const GOAL_TRUE: State = #goal_true;
        /// The atoms the goal forbids.
        pub const GOAL_FALSE: State = #goal_false;

        /// The task, grounded when the crate compiled.
        #[derive(Clone, Copy, Debug, Default)]
        pub struct Task;

        #[allow(clippy::all, clippy::pedantic)]
        impl ::pddl::runtime::Task for Task {
            type State = State;
            type Action = Action;

            fn initial_state(&self) -> State {
                INITIAL
            }

            fn successors(&self, state: &State, out: &mut ::std::vec::Vec<(Action, State)>) {
                for (i, op) in OPERATORS.iter().enumerate() {
                    if (0..WORDS).all(|w| state[w] & op.pre_true[w] == op.pre_true[w] && state[w] & op.pre_false[w] == 0) {
                        out.push((Action(i as u32), self.apply(state, Action(i as u32))));
                    }
                }
            }

            fn apply(&self, state: &State, action: Action) -> State {
                let op = &OPERATORS[action.0 as usize];
                ::core::array::from_fn(|w| (state[w] & !op.del[w]) | op.add[w])
            }

            fn is_goal(&self, state: &State) -> bool {
                (0..WORDS).all(|w| state[w] & GOAL_TRUE[w] == GOAL_TRUE[w] && state[w] & GOAL_FALSE[w] == 0)
            }

            fn cost(&self, action: Action) -> u32 {
                OPERATORS[action.0 as usize].cost
            }

            fn unsatisfied_goals(&self, state: &State) -> u32 {
                (0..WORDS)
                    .map(|w| (GOAL_TRUE[w] & !state[w]).count_ones() + (GOAL_FALSE[w] & state[w]).count_ones())
                    .sum()
            }
        }
    }
}
