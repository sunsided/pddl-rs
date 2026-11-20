//! Differential test of `pddl-validate` against the runtime grounder.
//!
//! Proptest generates small random STRIPS domains with negative preconditions,
//! conditional effects and repeated parameters, plus a problem. The grounder
//! ([`GroundTask`]) is an independent implementation of the same semantics, so for
//! each task:
//!
//! The goal is a few atoms of a state that a walk over the applicable operators
//! reaches, preferring those the walk changed, each required true or false as it
//! is there, so every task is solvable.
//!
//! - a breadth-first plan validates, and dropping any of its steps does not,
//! - a random walk over the ground operators gets the verdict the grounder
//!   predicts: the first inapplicable step fails its precondition, otherwise the
//!   plan is valid exactly when the final state satisfies the goal.
//!
//! Both sides share the parser, so this checks semantics, not parsing.

use std::fmt::Write as _;

use pddl::sas_plus::fdr::SasFile;
use pddl::sas_plus::ground;
use pddl::{Domain, Parser, Problem};
use pddl_bench::GroundTask;
use pddl_runtime::search::breadth_first;
use pddl_runtime::Task;
use pddl_validate::{Plan, PlanStep, ValidationStatus, Validator, ViolationKind};
use proptest::prelude::*;

/// A literal before it is resolved against a predicate list and a set of terms:
/// negated, predicate index and argument indices, each taken modulo the bound.
type RawLiteral = (bool, u8, [u8; 2]);

/// A generated action schema.
#[derive(Debug, Clone)]
struct ActionSpec {
    /// The number of parameters.
    params: usize,
    /// The precondition literals.
    pre: Vec<RawLiteral>,
    /// The unconditional effects; a negated literal is a delete.
    effects: Vec<RawLiteral>,
    /// An optional `when` effect: condition and effect.
    when: Option<(RawLiteral, RawLiteral)>,
}

/// A generated domain, problem and random walk.
#[derive(Debug, Clone)]
struct Spec {
    /// The arity of each predicate.
    arities: Vec<usize>,
    /// The number of objects.
    objects: usize,
    /// The action schemas.
    actions: Vec<ActionSpec>,
    /// Which ground atoms hold initially, by index into all ground atoms.
    init: Vec<bool>,
    /// The goal: which variables of the walked-to state it fixes.
    goal: Vec<u16>,
    /// The walk: whether to pick among the applicable operators, and which.
    walk: Vec<(bool, u16)>,
}

/// Returns a strategy for one raw literal.
fn literal() -> impl Strategy<Value = RawLiteral> {
    (any::<bool>(), any::<u8>(), any::<[u8; 2]>())
}

/// Returns a strategy for one action schema.
fn action() -> impl Strategy<Value = ActionSpec> {
    (
        0..=2usize,
        prop::collection::vec(literal(), 0..=2),
        prop::collection::vec(literal(), 1..=3),
        prop::option::of((literal(), literal())),
    )
        .prop_map(|(params, pre, effects, when)| ActionSpec {
            params,
            pre,
            effects,
            when,
        })
}

/// Returns a strategy for a whole spec.
fn spec() -> impl Strategy<Value = Spec> {
    (
        prop::collection::vec(0..=2usize, 1..=3),
        1..=3usize,
        prop::collection::vec(action(), 1..=3),
        prop::collection::vec(any::<bool>(), 21),
        prop::collection::vec(any::<u16>(), 1..=3),
        prop::collection::vec((any::<bool>(), any::<u16>()), 1..=8),
    )
        .prop_map(|(arities, objects, actions, init, goal, walk)| Spec {
            arities,
            objects,
            actions,
            init,
            goal,
            walk,
        })
}

/// Renders `(p1 t0 t1)` for a raw literal, with `term(i)` naming argument `i`
/// modulo `terms`, or `None` if the predicate needs arguments and there are no
/// terms. The negation is left to the caller.
fn atom(
    arities: &[usize],
    (_, pred, args): RawLiteral,
    terms: usize,
    term: &dyn Fn(usize) -> String,
) -> Option<String> {
    let pred = usize::from(pred) % arities.len();
    let arity = arities[pred];
    if arity > 0 && terms == 0 {
        return None;
    }
    let mut out = format!("(p{pred}");
    for &arg in &args[..arity] {
        write!(out, " {}", term(usize::from(arg) % terms)).unwrap();
    }
    out.push(')');
    Some(out)
}

/// Renders a literal, negated as `(not …)` if its flag is set.
fn literal_text(
    arities: &[usize],
    lit: RawLiteral,
    terms: usize,
    term: &dyn Fn(usize) -> String,
) -> Option<String> {
    let atom = atom(arities, lit, terms, term)?;
    Some(if lit.0 { format!("(not {atom})") } else { atom })
}

/// Renders `(and …)` of the literals that resolve.
fn and(parts: impl IntoIterator<Item = Option<String>>) -> String {
    let parts: Vec<String> = parts.into_iter().flatten().collect();
    format!("(and {})", parts.join(" "))
}

impl Spec {
    /// Renders the domain.
    fn domain(&self) -> String {
        let a = &self.arities;
        let mut out = String::from(
            "(define (domain d) (:requirements :strips :negative-preconditions :conditional-effects) (:predicates",
        );
        for (i, &arity) in a.iter().enumerate() {
            write!(out, " (p{i}").unwrap();
            for v in 0..arity {
                write!(out, " ?v{v}").unwrap();
            }
            out.push(')');
        }
        out.push(')');
        let param = |i: usize| format!("?x{i}");
        for (i, act) in self.actions.iter().enumerate() {
            let n = act.params;
            let params: Vec<String> = (0..n).map(param).collect();
            let mut effects: Vec<Option<String>> = act
                .effects
                .iter()
                .map(|&l| literal_text(a, l, n, &param))
                .collect();
            if let Some((cond, eff)) = act.when {
                if let (Some(c), Some(e)) = (
                    literal_text(a, cond, n, &param),
                    literal_text(a, eff, n, &param),
                ) {
                    effects.push(Some(format!("(when {c} {e})")));
                }
            }
            write!(
                out,
                " (:action a{i} :parameters ({}) :precondition {} :effect {})",
                params.join(" "),
                and(act.pre.iter().map(|&l| literal_text(a, l, n, &param))),
                and(effects),
            )
            .unwrap();
        }
        out.push(')');
        out
    }

    /// Renders the problem with `goal`.
    fn problem(&self, goal: &str) -> String {
        let a = &self.arities;
        let object = |i: usize| format!("o{i}");
        let objects: Vec<String> = (0..self.objects).map(object).collect();
        // Every ground atom, in a fixed order; `init` picks among the first ones.
        let mut atoms = Vec::new();
        for (pred, &arity) in a.iter().enumerate() {
            let tuples = self.objects.pow(u32::try_from(arity).unwrap());
            for t in 0..tuples {
                let words: Vec<String> = std::iter::once(format!("p{pred}"))
                    .chain((0..arity).map(|k| {
                        object(t / self.objects.pow(u32::try_from(k).unwrap()) % self.objects)
                    }))
                    .collect();
                atoms.push(format!("({})", words.join(" ")));
            }
        }
        let init: Vec<&String> = atoms
            .iter()
            .zip(&self.init)
            .filter(|(_, on)| **on)
            .map(|(x, _)| x)
            .collect();
        format!(
            "(define (problem p) (:domain d) (:objects {}) (:init {}) (:goal {}))",
            objects.join(" "),
            init.iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(" "),
            goal,
        )
    }
}

/// Renders the variable `var` of `file`, named `Atom p0(o1, o0)`, as `(p0 o1 o0)`,
/// negated unless `holds`.
fn goal_literal(file: &SasFile, var: usize, holds: bool) -> String {
    let name = &file.variables[var].values[0];
    let (pred, args) = name.strip_prefix("Atom ").unwrap().split_once('(').unwrap();
    let args = args.trim_end_matches(')');
    let atom = if args.is_empty() {
        format!("({pred})")
    } else {
        format!("({pred} {})", args.replace(", ", " "))
    };
    if holds {
        atom
    } else {
        format!("(not {atom})")
    }
}

/// Grounds `domain` and `problem`, panicking with `context` on failure.
fn task(domain: &Domain, problem: &Problem, context: &str) -> (SasFile, GroundTask) {
    let file = ground(domain, problem).unwrap_or_else(|e| panic!("{e}\n{context}"));
    (file.clone(), GroundTask::from_sas(file))
}

/// Whether variable `var` holds in `state`.
fn holds(state: &[u64], var: usize) -> bool {
    state[var / 64] >> (var % 64) & 1 == 1
}

/// The plan step of ground operator `op`, whose name is `a0 o1 o0`.
fn step(task: &GroundTask, op: u32) -> PlanStep {
    let mut words = task.name(op).split(' ');
    let action = words.next().unwrap().to_owned();
    PlanStep::new(action, words)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2048))]

    #[test]
    fn validator_agrees_with_grounder(spec in spec()) {
        let domain_text = spec.domain();
        let domain = Domain::from_str(&domain_text).unwrap();

        // Walk the applicable operators without a goal, and pick the goal from where it ends.
        let free = spec.problem("(and)");
        let (file, free_task) = task(&domain, &Problem::from_str(&free).unwrap(), &format!("{domain_text}\n{free}"));
        let mut state = free_task.initial_state();
        let mut successors = Vec::new();
        for &(_, pick) in &spec.walk {
            successors.clear();
            free_task.successors(&state, &mut successors);
            if successors.is_empty() {
                break;
            }
            state = successors.swap_remove(usize::from(pick) % successors.len()).1;
        }
        // Prefer the atoms the walk changed, so the goal needs some steps.
        let initial = free_task.initial_state();
        let mut candidates: Vec<usize> = (0..file.variables.len()).filter(|&v| holds(&state, v) != holds(&initial, v)).collect();
        if candidates.is_empty() {
            candidates = (0..file.variables.len()).collect();
        }
        let goal: Vec<String> = if candidates.is_empty() {
            Vec::new()
        } else {
            spec.goal.iter().map(|&g| candidates[usize::from(g) % candidates.len()]).map(|v| goal_literal(&file, v, holds(&state, v))).collect()
        };

        let problem_text = spec.problem(&format!("(and {})", goal.join(" ")));
        let context = format!("{domain_text}\n{problem_text}");
        let problem = Problem::from_str(&problem_text).unwrap();
        let validator = Validator::new(&domain, &problem).unwrap();
        let (_, task) = task(&domain, &problem, &context);

        let plan = breadth_first(&task).plan;
        prop_assert!(plan.is_some(), "the walk reaches the goal\n{}", context);
        if let Some(plan) = plan {
            let steps: Vec<PlanStep> = plan.actions.iter().map(|&op| step(&task, op)).collect();
            let report = validator.validate(&Plan::new(steps.clone()));
            prop_assert_eq!(report.status(), ValidationStatus::Valid, "{:?}\n{}", report, context);
            // Every action costs 1, so the plan is shortest and no shorter one is valid.
            for skip in 0..steps.len() {
                let mut dropped = steps.clone();
                dropped.remove(skip);
                let report = validator.validate(&Plan::new(dropped));
                prop_assert_ne!(report.status(), ValidationStatus::Valid, "dropped {}\n{}", skip, context);
            }
        }

        if task.is_empty() {
            return Ok(());
        }
        let ops = u32::try_from(task.len()).unwrap();
        let mut state = task.initial_state();
        let mut successors = Vec::new();
        let mut steps = Vec::new();
        // The 1-based step that fails its precondition, if any.
        let mut failing = None;
        for &(applicable, pick) in &spec.walk {
            successors.clear();
            task.successors(&state, &mut successors);
            let op = if applicable && !successors.is_empty() {
                successors[usize::from(pick) % successors.len()].0
            } else {
                u32::from(pick) % ops
            };
            steps.push(step(&task, op));
            let Some((_, next)) = successors.iter().find(|(o, _)| *o == op) else {
                failing = Some(steps.len());
                break;
            };
            state = next.clone();
        }

        let plan_text: Vec<String> = steps.iter().map(ToString::to_string).collect();
        let plan_text = plan_text.join(" ");
        let report = validator.validate(&Plan::new(steps));
        match failing {
            Some(number) => {
                let violation = report.violation();
                prop_assert!(
                    violation.is_some_and(|v| v.step.as_ref().is_some_and(|s| s.number == number)
                        && matches!(v.kind, ViolationKind::PreconditionUnsatisfied(_))),
                    "expected step {} to fail its precondition: {:?}\n{}\n{}", number, report, plan_text, context
                );
            }
            None if task.is_goal(&state) => {
                prop_assert_eq!(report.status(), ValidationStatus::Valid, "{:?}\n{}\n{}", report, plan_text, context);
            }
            None => {
                prop_assert!(
                    report.violation().is_some_and(|v| v.step.is_none()
                        && matches!(v.kind, ViolationKind::GoalUnsatisfied(_))),
                    "expected the goal to fail: {:?}\n{}\n{}", report, plan_text, context
                );
            }
        }
    }
}
