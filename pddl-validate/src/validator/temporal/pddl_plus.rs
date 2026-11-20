//! PDDL+ (level 4): processes and events on the timeline.
//!
//! A process runs while its precondition holds and adds its linear rates to those of
//! the running durative actions. An event fires as soon as its precondition holds;
//! all enabled events fire together, and the validator re-checks after every round,
//! so events can trigger each other in a cascade, up to [`CASCADE_LIMIT`] rounds at one
//! time.
//!
//! Between two plan happenings the validator splits time into segments. In a segment
//! the set of running processes is fixed, so every fluent changes linearly and the
//! truth of every process and event precondition changes only at the roots of its
//! comparisons. The validator finds those roots as for `over all` conditions, checks the
//! preconditions at each root and between them, and ends the segment at the first root
//! where a process switches or an event becomes enabled:
//!
//! ```mermaid
//! flowchart TD
//!     settle["fix the running processes just after t"] --> fire{"event enabled at t or just after?"}
//!     fire -- yes --> cascade["fire them at t, those enabled at t first"] --> settle
//!     fire -- no --> next["find the first switch after t, or the next happening"]
//!     next --> invariants["check over all conditions on (t, switch) and at the switch"]
//!     invariants --> advance["advance to the switch"] --> settle
//! ```
//!
//! An event that is enabled only just after a root, such as `(< (h) 0)` when `h`
//! reaches 0, fires at the root. Nonlinear dynamics stay unsupported.
//!
//! The work is bounded. Each interval between two plan happenings may hold at most
//! [`SEGMENT_LIMIT`] segments, that is process switches and event firings, and each
//! time at most [`CASCADE_LIMIT`] rounds of events. A model that goes past a bound is
//! reported [`Construct::ZenoDynamics`] as unsupported, never valid or invalid, even if
//! it is legal but only very busy, such as an event that fires every 0.001 time units
//! between happenings 1000 apart.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use pddl::{ActionDefinition, PreconditionGoalDescriptions, ProcessDefinition};

use super::continuous::{self, Flow, Rates};
use super::timed::{condition, Moment};
use super::{failure, Failure, Instance, Timeline, SAME_TIME};
use crate::effects::Changes;
use crate::eval::{EvalError, Evaluator, Observer, Scope};
use crate::report::{Construct, StepTrace, ViolationKind};
use crate::state::State;
use crate::validator::Stop;

/// The most rounds of events that may fire at one time.
pub const CASCADE_LIMIT: usize = 1000;

/// The most segments between two plan happenings before the dynamics count as Zeno.
///
/// A segment ends wherever a process switches on or off or an event fires, so this
/// bounds the switches and firings per interval between plan happenings. Past it, the
/// validator stops with [`Construct::ZenoDynamics`]: the plan is reported unsupported,
/// not validated, even when the model is legal and only very busy.
const SEGMENT_LIMIT: usize = 100_000;

/// A process or event with its parameters bound.
struct Ground<'v, T> {
    /// The definition.
    definition: &'v T,
    /// The bindings of its parameters.
    scope: Scope,
    /// Its name in traces, such as `(fall b1)`.
    label: String,
}

/// Every ground process and event of the task.
pub struct Dynamics<'v> {
    /// The ground processes.
    processes: Vec<Ground<'v, ProcessDefinition>>,
    /// The ground events.
    events: Vec<Ground<'v, ActionDefinition>>,
}

/// Which processes run and which events are enabled, in the order of
/// [`Dynamics::processes`] and [`Dynamics::events`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct Switches {
    /// `true` for each running process.
    processes: Vec<bool>,
    /// `true` for each enabled event.
    events: Vec<bool>,
}

impl Switches {
    /// Returns `true` if any event is enabled.
    fn any_event(&self) -> bool {
        self.events.iter().any(|enabled| *enabled)
    }
}

impl<'v> Dynamics<'v> {
    /// Grounds every process and event over the objects of their parameter types.
    pub fn new(evaluator: &Evaluator<'_, 'v>) -> Self {
        let model = evaluator.model;
        let mut processes = Vec::new();
        for (name, process) in &model.processes {
            for scope in bindings(evaluator, process.parameters()) {
                let label = label(name, &scope);
                processes.push(Ground {
                    definition: *process,
                    scope,
                    label,
                });
            }
        }

        let mut events = Vec::new();
        for (name, event) in &model.events {
            for scope in bindings(evaluator, event.parameters()) {
                let label = label(name, &scope);
                events.push(Ground {
                    definition: *event,
                    scope,
                    label,
                });
            }
        }

        Self { processes, events }
    }

    /// Returns `true` if the task has no processes and no events.
    pub fn is_empty(&self) -> bool {
        self.processes.is_empty() && self.events.is_empty()
    }

    /// Evaluates every process and event precondition.
    fn switches(&self, evaluator: &Evaluator<'_, '_>) -> Result<Switches, EvalError> {
        let holds = |precondition: &PreconditionGoalDescriptions, scope: &Scope| {
            let mut scope = scope.clone();
            Ok::<_, EvalError>(evaluator.preconditions(precondition, &mut scope)?.is_none())
        };
        let processes = self
            .processes
            .iter()
            .map(|p| holds(p.definition.precondition(), &p.scope))
            .collect::<Result<_, _>>()?;
        let events = self
            .events
            .iter()
            .map(|e| holds(e.definition.precondition(), &e.scope))
            .collect::<Result<_, _>>()?;

        Ok(Switches { processes, events })
    }
}

/// Returns every binding of `variables`.
fn bindings(evaluator: &Evaluator<'_, '_>, variables: &pddl::TypedVariables) -> Vec<Scope> {
    let mut out = Vec::new();
    let _ = evaluator.each_binding(variables, &mut Scope::new(), |scope| {
        out.push(scope.clone());
        Ok(())
    });
    out
}

/// Names a ground process or event, such as `(fall b1)`.
fn label(name: &str, scope: &Scope) -> String {
    let mut text = format!("({name}");
    for (_, object) in scope {
        text.push(' ');
        text.push_str(object);
    }

    text.push(')');
    text
}

/// One segment of time with a fixed set of running processes.
struct Segment<'s> {
    /// The state at the start of the segment.
    state: &'s State,
    /// The start of the segment.
    start: f64,
    /// The latest end: the next plan happening.
    end: f64,
    /// The running processes.
    running: Vec<bool>,
    /// The rates of change, from running durative actions and processes.
    rates: Rates,
}

impl<'v> Timeline<'v, '_> {
    /// Fires the events enabled in `state` at `time`, round after round, until none is.
    pub(super) fn cascade(
        &self,
        dynamics: &Dynamics<'v>,
        state: &mut State,
        time: f64,
        trace: &mut Vec<StepTrace>,
    ) -> Result<(), Failure> {
        for _ in 0..CASCADE_LIMIT {
            let switches = dynamics
                .switches(&self.evaluator(state, None))
                .map_err(|error| failure(None, Stop::from(error)))?;
            if !switches.any_event() {
                return Ok(());
            }

            self.fire(dynamics, state, &switches.events, time, trace)?;
        }

        let kind = ViolationKind::EventCascade {
            time,
            limit: CASCADE_LIMIT,
        };
        Err(failure(None, Stop::Invalid(kind)))
    }

    /// Fires the `enabled` events together in `state` at `time`.
    fn fire(
        &self,
        dynamics: &Dynamics<'v>,
        state: &mut State,
        enabled: &[bool],
        time: f64,
        trace: &mut Vec<StepTrace>,
    ) -> Result<(), Failure> {
        let stop = |error: EvalError| failure(None, Stop::from(error));
        let mut changes = Changes::default();
        let mut labels = Vec::new();
        {
            let evaluator = self.evaluator(state, None);
            let fired = dynamics.events.iter().zip(enabled).filter(|(_, on)| **on);
            for (event, _) in fired {
                labels.push(format!("event {}", event.label));
                if let Some(effects) = event.definition.effect() {
                    let mut scope = event.scope.clone();
                    changes
                        .collect(&evaluator, effects, &mut scope)
                        .map_err(stop)?;
                }
            }
        }

        let mut happening = changes.apply(state).map_err(stop)?;
        happening.time = Some(time);
        happening.events = labels;
        trace.push(happening);

        Ok(())
    }

    /// Runs processes and events from `start` to `end`, checking the `over all`
    /// conditions of the `running` durative actions. Returns the state at `end`.
    pub(super) fn hybrid_interval(
        &self,
        dynamics: &Dynamics<'v>,
        state: &State,
        running: &[&Instance<'v>],
        start: f64,
        end: f64,
        trace: &mut Vec<StepTrace>,
    ) -> Result<State, Failure> {
        let stop = |error: EvalError| failure(None, Stop::from(error));
        let mut state = state.clone();
        let mut time = start;
        let mut rounds_here = 0;
        for _ in 0..SEGMENT_LIMIT {
            if end - time <= SAME_TIME {
                // At the next plan happening: what triggers now fires before it.
                self.cascade(dynamics, &mut state, end, trace)?;
                return Ok(state);
            }

            let segment = self
                .settle(dynamics, &state, running, time, end)
                .map_err(stop)?;
            let points = self.breakpoints(dynamics, &segment).map_err(stop)?;
            let first = points.first().copied().unwrap_or(end);

            if let Some(enabled) = self
                .events_to_fire(dynamics, &segment, first)
                .map_err(stop)?
            {
                rounds_here += 1;
                if rounds_here > CASCADE_LIMIT {
                    let kind = ViolationKind::EventCascade {
                        time,
                        limit: CASCADE_LIMIT,
                    };
                    return Err(failure(None, Stop::Invalid(kind)));
                }

                self.fire(dynamics, &mut state, &enabled, time, trace)?;
                continue;
            }

            let switch = self
                .next_switch(dynamics, &segment, &points)
                .map_err(stop)?;
            let until = switch.unwrap_or(end);
            self.check_invariants(running, &state, &segment.rates, time, until)?;
            state = continuous::advance(&state, &segment.rates, until - time);
            if switch.is_none() {
                return Ok(state);
            }

            // The switch point lies inside the actions' open interval, so their
            // invariants must hold there too, before any event fires.
            if end - until > SAME_TIME {
                self.check_invariants_at(running, &state, until)?;
            }

            time = until;
            rounds_here = 0;
        }

        Err(failure(None, Stop::Unsupported(Construct::ZenoDynamics)))
    }

    /// Returns the events to fire at the start of `segment`, or `None` if none is
    /// enabled there or just after it, before `first`, its first breakpoint.
    ///
    /// Events enabled now fire alone; the next round re-checks what is enabled just
    /// after in the state they leave. Events enabled only just after the start fire at
    /// the start.
    fn events_to_fire(
        &self,
        dynamics: &Dynamics<'v>,
        segment: &Segment<'_>,
        first: f64,
    ) -> Result<Option<Vec<bool>>, EvalError> {
        let now = dynamics.switches(&self.evaluator(segment.state, None))?;
        if now.any_event() {
            return Ok(Some(now.events));
        }

        let after = self.switches_at(dynamics, segment, segment.start.midpoint(first))?;
        Ok(after.any_event().then_some(after.events))
    }

    /// Checks the `over all` conditions of the `running` durative actions in `state`
    /// at the single point `time`.
    fn check_invariants_at(
        &self,
        running: &[&Instance<'v>],
        state: &State,
        time: f64,
    ) -> Result<(), Failure> {
        let evaluator = self.evaluator(state, None);
        for instance in running {
            let Some(invariant) = instance.action.condition() else {
                continue;
            };

            let at = || Some(instance.step.clone());
            let mut scope = instance.scope.clone();
            match condition(&evaluator, invariant, &mut scope, Moment::Over) {
                Ok(None) => {}
                Ok(Some(unmet)) => {
                    let kind = ViolationKind::InvariantViolated { time, unmet };
                    return Err(failure(at(), Stop::Invalid(kind)));
                }
                Err(error) => return Err(failure(at(), Stop::from(error))),
            }
        }

        Ok(())
    }

    /// Finds the processes that run just after `start`: those whose precondition holds
    /// on the first stretch of the segment under their own rates, a fixed point.
    fn settle<'s>(
        &self,
        dynamics: &Dynamics<'v>,
        state: &'s State,
        running: &[&Instance<'v>],
        start: f64,
        end: f64,
    ) -> Result<Segment<'s>, EvalError> {
        let mut processes = dynamics.switches(&self.evaluator(state, None))?.processes;
        for _ in 0..=dynamics.processes.len() {
            let segment = self.segment(dynamics, state, running, start, end, processes.clone())?;
            let points = self.breakpoints(dynamics, &segment)?;
            let first = points.first().copied().unwrap_or(end);
            let after = self.switches_at(dynamics, &segment, start.midpoint(first))?;
            if after.processes == processes {
                return Ok(segment);
            }

            processes = after.processes;
        }

        Err(EvalError::Unsupported(Construct::ZenoDynamics))
    }

    /// Builds the segment from `start` with the processes in `processes` running.
    fn segment<'s>(
        &self,
        dynamics: &Dynamics<'v>,
        state: &'s State,
        running: &[&Instance<'v>],
        start: f64,
        end: f64,
        processes: Vec<bool>,
    ) -> Result<Segment<'s>, EvalError> {
        let evaluator = self.evaluator(state, None);
        let mut flows: Vec<Flow<'v>> = running
            .iter()
            .flat_map(|i| i.flows.iter().cloned())
            .collect();
        for (process, _) in dynamics
            .processes
            .iter()
            .zip(&processes)
            .filter(|(_, on)| **on)
        {
            if let Some(effect) = process.definition.effect() {
                let mut scope = process.scope.clone();
                continuous::flows(&evaluator, effect, &mut scope, &mut flows)?;
            }
        }

        let refs: Vec<&Flow<'v>> = flows.iter().collect();
        let rates = continuous::rates(&evaluator, &refs)?;

        Ok(Segment {
            state,
            start,
            end,
            running: processes,
            rates,
        })
    }

    /// Evaluates the process and event preconditions at `time` within `segment`.
    fn switches_at(
        &self,
        dynamics: &Dynamics<'v>,
        segment: &Segment<'_>,
        time: f64,
    ) -> Result<Switches, EvalError> {
        let state = continuous::advance(segment.state, &segment.rates, time - segment.start);
        dynamics.switches(&self.evaluator(&state, None))
    }

    /// Returns the roots inside the segment at which a process or event precondition may
    /// change, sorted, followed by the segment's end.
    ///
    /// # Errors
    ///
    /// [`Construct::NonlinearDynamics`] for a precondition that is not linear in time.
    fn breakpoints(
        &self,
        dynamics: &Dynamics<'v>,
        segment: &Segment<'_>,
    ) -> Result<Vec<f64>, EvalError> {
        let differences = |time: f64| {
            let state = continuous::advance(segment.state, &segment.rates, time - segment.start);
            let observer = RefCell::new(Observer {
                moving: segment.rates.keys().cloned().collect(),
                ..Observer::default()
            });
            dynamics.switches(&self.evaluator(&state, Some(&observer)))?;
            let observer = observer.into_inner();
            if observer.nonlinear {
                return Err(EvalError::Unsupported(Construct::NonlinearDynamics));
            }

            Ok(observer.differences)
        };
        let first = differences(segment.start)?;
        let last = differences(segment.end)?;
        if first.len() != last.len() {
            return Err(EvalError::Unsupported(Construct::NonlinearDynamics));
        }

        let length = segment.end - segment.start;
        let mut points: Vec<f64> = first
            .iter()
            .zip(&last)
            .filter(|(a, b)| *a - *b != 0.0)
            .map(|(a, b)| segment.start + length * a / (a - b))
            .filter(|t| *t > segment.start + SAME_TIME && *t < segment.end)
            .collect();
        points.sort_by(f64::total_cmp);
        points.dedup_by(|a, b| *a - *b <= SAME_TIME);
        points.push(segment.end);

        Ok(points)
    }

    /// Returns the first breakpoint after the segment's first stretch at which a process
    /// switches or an event becomes enabled, or `None` if none does before the end.
    fn next_switch(
        &self,
        dynamics: &Dynamics<'v>,
        segment: &Segment<'_>,
        points: &[f64],
    ) -> Result<Option<f64>, EvalError> {
        let mut previous: Option<f64> = None;
        for &point in points {
            let samples = previous
                .map(|p| p.midpoint(point))
                .into_iter()
                .chain([point]);
            for (index, sample) in samples.enumerate() {
                let switches = self.switches_at(dynamics, segment, sample)?;
                if switches.any_event() || switches.processes != segment.running {
                    // A change between two roots starts at the earlier one.
                    let at = if index == 0 && previous.is_some() {
                        previous
                    } else {
                        Some(point)
                    };
                    return Ok(at);
                }
            }

            previous = Some(point);
        }

        Ok(None)
    }
}
