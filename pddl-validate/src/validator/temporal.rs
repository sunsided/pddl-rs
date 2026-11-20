//! Temporal plans (level 3): durative actions, happenings, invariants, interference and
//! timed initial literals, after PDDL 2.1 and VAL.
//!
//! A durative action starting at `t` with duration `d` splits into a start event at `t`
//! and an end event at `t + d`. Instantaneous actions and timed initial literals are
//! events too. Events at the same time (within [`SAME_TIME`]) form a happening, and
//! happenings run in time order:
//!
//! ```mermaid
//! flowchart TD
//!     schedule["split steps into events, add timed literals"] --> next{"next happening?"}
//!     next -- yes --> interval["advance continuous change; check over all conditions"]
//!     interval --> conditions["check at start / at end conditions, preconditions, durations"]
//!     conditions --> collect["collect each event's effects in the state before"]
//!     collect --> mutex["reject interfering events less than epsilon apart"]
//!     mutex --> apply["apply all effects: deletes, adds, numeric updates"]
//!     apply --> next
//!     next -- no --> goal["check the goal and the metric"]
//! ```
//!
//! An untimed plan for a temporal task runs its steps at times 1, 2, 3, and so on. In a
//! metric, `total-time` is the makespan: the time of the last event of the plan. Timed
//! initial literals after the makespan do not happen.

mod continuous;
mod mutex;
mod timed;

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use pddl::{
    ActionDefinition, AtomicFormula, DurativeActionDefinition, InitElement, Literal, Problem,
};

use super::{bind_parameters, finish, step_ref, stopped, Stop};
use crate::atom::{lower, Atom};
use crate::effects::Changes;
use crate::eval::{Evaluator, Observer, Scope};
use crate::model::Model;
use crate::plan::{Plan, PlanStep};
use crate::report::{Report, StepRef, StepTrace, ViolationKind};
use crate::state::State;
use continuous::{Flow, Interval};
use mutex::Footprint;
use timed::{Collector, Memo, Moment};

/// Events closer in time than this belong to one happening.
pub const SAME_TIME: f64 = 1e-9;

/// Runs plans on the timeline of a temporal task.
pub struct Timeline<'v, 'a> {
    /// The indexed domain and problem.
    pub model: &'v Model<'a>,
    /// The problem, for its timed initial literals, goal and metric.
    pub problem: &'a Problem,
    /// The relative tolerance of numeric comparisons.
    pub tolerance: f64,
    /// The least time between interfering events.
    pub epsilon: f64,
}

/// A durative action of the plan, between its start and its end.
struct Instance<'v> {
    /// The action.
    action: &'v DurativeActionDefinition,
    /// Its parameters, bound to the step's objects.
    scope: Scope,
    /// The duration, positive.
    duration: f64,
    /// The step that starts it.
    step: StepRef,
    /// The `at start` conditions of its `at end` conditional effects.
    memo: Memo,
    /// Its continuous effects, ground when it starts.
    flows: Vec<Flow<'v>>,
}

/// What happens at an event.
enum Kind<'v> {
    /// An instantaneous action, with its parameters bound.
    Instant(&'v ActionDefinition, Scope),
    /// The start of an [`Instance`], by index.
    Start(usize),
    /// The end of an [`Instance`], by index.
    End(usize),
    /// A timed initial literal: the atom, and `true` to add it or `false` to delete it.
    Literal(Atom, bool),
}

/// Something that happens at one time.
struct Event<'v> {
    /// When.
    time: f64,
    /// What.
    kind: Kind<'v>,
    /// The plan step it belongs to; `None` for a timed initial literal.
    step: Option<StepRef>,
    /// What it is called in traces and diagnostics.
    label: String,
}

/// An event that already happened, kept while it may interfere with later ones.
struct Past {
    /// When it happened.
    time: f64,
    /// What it is called.
    label: String,
    /// What it read and wrote.
    footprint: Footprint,
    /// The durative instance it starts or ends, if any. An action's own start and end
    /// never interfere: the domain orders them.
    owner: Option<usize>,
}

/// Why the timeline stopped, and at which step.
type Failure = Box<(Option<StepRef>, Stop)>;

/// Builds a [`Failure`].
fn failure(step: Option<StepRef>, reason: Stop) -> Failure {
    Box::new((step, reason))
}

impl<'v> Timeline<'v, '_> {
    /// Runs `plan` from `initial` and checks the goal at the end.
    pub fn run(&self, initial: &State, plan: &Plan) -> Report {
        let (mut instances, mut events) = match self.schedule(plan) {
            Ok(scheduled) => scheduled,
            Err(failed) => {
                let (step, reason) = *failed;
                return stopped(reason, step, String::from("plan"), Vec::new());
            }
        };

        let makespan = events.iter().map(|event| event.time).fold(0.0, f64::max);
        events.extend(self.timed_literals(makespan));
        events.sort_by(|a, b| a.time.total_cmp(&b.time));

        match self.happenings(initial, &mut instances, &events) {
            Ok((state, trace)) => {
                let evaluator = self.evaluator(&state, None);
                finish(&evaluator, self.problem, makespan, trace)
            }
            Err((failed, trace)) => {
                let (step, reason) = *failed;
                let location = step.as_ref().map_or_else(
                    || String::from("timeline"),
                    |s| format!("step {}", s.number),
                );
                stopped(reason, step, location, trace)
            }
        }
    }

    /// Builds an evaluator for `state`.
    const fn evaluator<'s>(
        &'s self,
        state: &'s State,
        observer: Option<&'s RefCell<Observer>>,
    ) -> Evaluator<'s, 'v> {
        Evaluator {
            model: self.model,
            state,
            tolerance: self.tolerance,
            observer,
        }
    }

    /// Splits the plan's steps into events and durative instances.
    fn schedule(&self, plan: &Plan) -> Result<(Vec<Instance<'v>>, Vec<Event<'v>>), Failure> {
        let timed = plan.steps().iter().any(|step| step.time().is_some());
        let mut instances = Vec::new();
        let mut events = Vec::new();
        for (index, step) in plan.steps().iter().enumerate() {
            let at = step_ref(index, step);
            let fail = |kind| Err(failure(Some(at.clone()), Stop::Invalid(kind)));
            let time = match step.time() {
                Some(time) => time,
                None if timed => return fail(ViolationKind::MissingTimestamp),
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "plans of more than 2^53 steps do not fit in memory"
                )]
                None => (index + 1) as f64,
            };

            if let Some(action) = self.model.durative.get(step.action()) {
                let instance = self.instance(action, step, &at)?;
                let label = at.action.clone();
                let end = time + instance.duration;
                let number = instances.len();
                events.push(event(
                    time,
                    Kind::Start(number),
                    &at,
                    format!("start {label}"),
                ));
                events.push(event(end, Kind::End(number), &at, format!("end {label}")));
                instances.push(instance);
            } else {
                let (action, scope) =
                    super::bind(self.model, step).map_err(|s| failure(Some(at.clone()), s))?;
                if step.duration().is_some() {
                    return fail(ViolationKind::UnexpectedDuration);
                }

                let label = at.action.clone();
                events.push(event(time, Kind::Instant(action, scope), &at, label));
            }
        }

        Ok((instances, events))
    }

    /// Binds a durative action of `step` and checks its duration.
    fn instance(
        &self,
        action: &'v DurativeActionDefinition,
        step: &PlanStep,
        at: &StepRef,
    ) -> Result<Instance<'v>, Failure> {
        let fail = |kind| Err(failure(Some(at.clone()), Stop::Invalid(kind)));
        let scope = bind_parameters(self.model, action.parameters(), step)
            .map_err(|reason| failure(Some(at.clone()), reason))?;
        let Some(duration) = step.duration() else {
            return fail(ViolationKind::MissingDuration);
        };

        if duration <= 0.0 {
            return fail(ViolationKind::NonPositiveDuration(duration));
        }

        Ok(Instance {
            action,
            scope,
            duration,
            step: at.clone(),
            memo: Memo::new(),
            flows: Vec::new(),
        })
    }

    /// Returns the timed initial literals up to `makespan` as events.
    fn timed_literals(&self, makespan: f64) -> Vec<Event<'v>> {
        let mut events = Vec::new();
        for element in self.problem.init().iter() {
            let InitElement::At(time, literal) = element else {
                continue;
            };

            let (formula, positive) = match literal {
                Literal::AtomicFormula(formula) => (formula, true),
                Literal::NotAtomicFormula(formula) => (formula, false),
            };
            // The static check rejects timed equalities.
            let AtomicFormula::Predicate(predicate) = formula else {
                continue;
            };

            let time = **time;
            if time > makespan + SAME_TIME {
                continue;
            }

            let arguments = predicate.values().iter().map(|name| lower(name)).collect();
            let atom = Atom::lowered(lower(predicate.predicate()), arguments);
            let label = if positive {
                format!("timed literal {atom} at {time}")
            } else {
                format!("timed literal (not {atom}) at {time}")
            };
            events.push(Event {
                time,
                kind: Kind::Literal(atom, positive),
                step: None,
                label,
            });
        }

        events
    }

    /// Runs the happenings of the sorted `events`. Returns the final state and the trace,
    /// or the failure with the trace so far.
    fn happenings(
        &self,
        initial: &State,
        instances: &mut [Instance<'v>],
        events: &[Event<'v>],
    ) -> Result<(State, Vec<StepTrace>), (Failure, Vec<StepTrace>)> {
        let mut state = initial.clone();
        let mut trace = Vec::new();
        let mut active = BTreeSet::new();
        let mut recent: Vec<Past> = Vec::new();
        let mut clock = 0.0;
        let mut rest = events;
        while let Some(first) = rest.first() {
            let size = rest
                .iter()
                .take_while(|event| event.time - first.time <= SAME_TIME)
                .count();
            let (happening, later) = rest.split_at(size);
            rest = later;

            if first.time - clock > SAME_TIME {
                state = match self.interval(&state, instances, &active, clock, first.time) {
                    Ok(next) => next,
                    Err(failure) => return Err((failure, trace)),
                };
                clock = first.time;
            }

            // Events exactly epsilon apart are far enough, despite rounding.
            recent.retain(|past| first.time - past.time < self.epsilon - SAME_TIME);
            match self.happen(&mut state, instances, happening, &mut recent) {
                Ok(step) => trace.push(step),
                Err(failure) => return Err((failure, trace)),
            }

            for event in happening {
                match event.kind {
                    Kind::Start(index) => {
                        active.insert(index);
                    }
                    Kind::End(index) => {
                        active.remove(&index);
                    }
                    Kind::Instant(..) | Kind::Literal(..) => {}
                }
            }
        }

        Ok((state, trace))
    }

    /// Advances continuous change from `start` to `end` and checks the `over all`
    /// conditions of the actions running throughout. Returns the state at `end`.
    fn interval(
        &self,
        state: &State,
        instances: &[Instance<'v>],
        active: &BTreeSet<usize>,
        start: f64,
        end: f64,
    ) -> Result<State, Failure> {
        let running: Vec<&Instance<'v>> = active.iter().filter_map(|&i| instances.get(i)).collect();
        let Some(first) = running.first() else {
            return Ok(state.clone());
        };

        let flows: Vec<&Flow<'v>> = running.iter().flat_map(|i| &i.flows).collect();
        let rates = continuous::rates(&self.evaluator(state, None), &flows)
            .map_err(|error| failure(Some(first.step.clone()), Stop::from(error)))?;
        let interval = Interval {
            start,
            end,
            state,
            rates: &rates,
        };
        for instance in running {
            let Some(invariant) = instance.action.condition() else {
                continue;
            };

            let at = || Some(instance.step.clone());
            let mut scope = instance.scope.clone();
            match interval.check(self.model, self.tolerance, invariant, &mut scope) {
                Ok(None) => {}
                Ok(Some((time, unmet))) => {
                    let kind = ViolationKind::InvariantViolated { time, unmet };
                    return Err(failure(at(), Stop::Invalid(kind)));
                }
                Err(error) => return Err(failure(at(), Stop::from(error))),
            }
        }

        Ok(continuous::advance(state, &rates, end - start))
    }

    /// Runs one happening on `state`: checks every event's conditions in the state
    /// before it, rejects interference, and applies all effects at once.
    fn happen(
        &self,
        state: &mut State,
        instances: &mut [Instance<'v>],
        happening: &[Event<'v>],
        recent: &mut Vec<Past>,
    ) -> Result<StepTrace, Failure> {
        let mut merged = Changes::default();
        let mut current: Vec<Past> = Vec::new();
        for event in happening {
            let fail = |reason| failure(event.step.clone(), reason);
            let observer = RefCell::new(Observer::default());
            let evaluator = self.evaluator(state, Some(&observer));
            // A failed condition still records what it read, and interference is the
            // better diagnostic: the condition may fail only because of the order.
            let result = run_event(&evaluator, event, instances);
            let footprint = Footprint::new(
                &observer.borrow(),
                result.as_ref().unwrap_or(&Changes::default()),
            );
            let clash = recent.iter().chain(&current).find(|past| {
                (past.owner.is_none() || past.owner != owner(event))
                    && past.footprint.interferes(&footprint)
            });
            if let Some(past) = clash {
                let kind = ViolationKind::Interference {
                    event: event.label.clone(),
                    other: past.label.clone(),
                    time: event.time,
                    separation: event.time - past.time,
                };
                return Err(fail(Stop::Invalid(kind)));
            }

            merged.merge(result.map_err(fail)?);
            current.push(Past {
                time: event.time,
                label: event.label.clone(),
                footprint,
                owner: owner(event),
            });
        }

        let first_step = happening.first().and_then(|event| event.step.clone());
        let mut step = merged
            .apply(state)
            .map_err(|error| failure(first_step, Stop::from(error)))?;
        step.time = happening.first().map(|event| event.time);
        step.events = happening.iter().map(|event| event.label.clone()).collect();
        recent.extend(current);

        Ok(step)
    }
}

/// Returns the durative instance an event starts or ends.
const fn owner(event: &Event<'_>) -> Option<usize> {
    match event.kind {
        Kind::Start(index) | Kind::End(index) => Some(index),
        Kind::Instant(..) | Kind::Literal(..) => None,
    }
}

/// Builds a plan event.
fn event<'v>(time: f64, kind: Kind<'v>, step: &StepRef, label: String) -> Event<'v> {
    Event {
        time,
        kind,
        step: Some(step.clone()),
        label,
    }
}

/// Checks the conditions of one event and collects its effects.
fn run_event<'v>(
    evaluator: &Evaluator<'_, '_>,
    event: &Event<'v>,
    instances: &mut [Instance<'v>],
) -> Result<Changes, Stop> {
    let mut changes = Changes::default();
    match &event.kind {
        Kind::Instant(action, scope) => {
            let mut scope = scope.clone();
            if let Some(unmet) = evaluator.preconditions(action.precondition(), &mut scope)? {
                return Err(Stop::Invalid(ViolationKind::PreconditionUnsatisfied(unmet)));
            }

            if let Some(effects) = action.effect() {
                changes.collect(evaluator, effects, &mut scope)?;
            }
        }
        Kind::Start(index) | Kind::End(index) => {
            let moment = if matches!(event.kind, Kind::Start(_)) {
                Moment::Start
            } else {
                Moment::End
            };
            if let Some(instance) = instances.get_mut(*index) {
                durative_event(evaluator, instance, moment, &mut changes)?;
            }
        }
        Kind::Literal(atom, true) => {
            changes.add.insert(atom.clone());
        }
        Kind::Literal(atom, false) => {
            changes.delete.insert(atom.clone());
        }
    }

    Ok(changes)
}

/// Checks the duration and the conditions of a durative action at `moment` and
/// collects its effects then; at the start, also grounds its continuous effects.
fn durative_event(
    evaluator: &Evaluator<'_, '_>,
    instance: &mut Instance<'_>,
    moment: Moment,
    changes: &mut Changes,
) -> Result<(), Stop> {
    let action = instance.action;
    let mut scope = instance.scope.clone();
    if let Some(constraint) = action.duration() {
        let unmet = timed::duration(evaluator, constraint, &scope, instance.duration, moment)?;
        if let Some(unmet) = unmet {
            return Err(Stop::Invalid(ViolationKind::DurationConstraint(unmet)));
        }
    }

    if let Some(condition) = action.condition() {
        if let Some(unmet) = timed::condition(evaluator, condition, &mut scope, moment)? {
            let kind = match moment {
                Moment::End => ViolationKind::EndConditionUnsatisfied(unmet),
                Moment::Start | Moment::Over => ViolationKind::StartConditionUnsatisfied(unmet),
            };
            return Err(Stop::Invalid(kind));
        }
    }

    let Some(effect) = action.effect() else {
        return Ok(());
    };

    let collector = Collector {
        evaluator,
        moment,
        duration: instance.duration,
    };
    collector.collect(effect, &mut scope, &mut instance.memo, changes)?;
    if moment == Moment::Start {
        continuous::flows(evaluator, effect, &mut scope, &mut instance.flows)?;
    }

    Ok(())
}
