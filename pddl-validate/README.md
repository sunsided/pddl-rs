# pddl-validate

An independent plan validator for PDDL: a second opinion next to
[VAL](https://github.com/KCL-Planning/VAL), with precise, machine-readable
diagnostics.

It is part of the [pddl-rs](https://github.com/sunsided/pddl-rs) workspace and
depends only on the `pddl` syntax tree. It implements the validation semantics
itself and shares no grounding or search code with any planner, so it does not
share their bugs either.

```rust
use pddl::{Domain, Parser, Problem};
use pddl_validate::{ValidationStatus, Validator};

let domain = Domain::from_str(
    "(define (domain switch) (:predicates (on))
       (:action flip :parameters () :precondition (not (on)) :effect (on)))",
).unwrap();
let problem = Problem::from_str(
    "(define (problem light) (:domain switch) (:init) (:goal (on)))",
).unwrap();
let validator = Validator::new(&domain, &problem).unwrap();

let report = validator.validate_text("(flip)\n(flip)");
assert_eq!(report.status(), ValidationStatus::Invalid);
assert_eq!(
    report.violation().unwrap().advice(),
    "(flip) has an unsatisfied precondition at step 2: (Set (on) to false)"
);
```

## Scope

| Level | Constructs | Status |
| --- | --- | --- |
| 1. Classical | STRIPS, typing, negative and disjunctive preconditions, equality, `exists`, `forall`, `imply`, conditional effects, derived predicates | Supported |
| 2. Numeric | numeric fluents, `assign`, `increase`, `decrease`, `scale-up`, `scale-down`, action costs, metrics | Supported |
| 3. Temporal | durative actions, timestamped plans, duration inequalities, linear continuous effects, timed initial literals, epsilon separation, makespan | Supported |
| 4. PDDL+ | processes and events with linear dynamics, event cascades | Supported |

A plan that uses a construct outside the supported levels is reported as
`Unsupported`, never as valid. So are continuous change whose rate reads a
changing fluent, `over all` conditions that are not linear in time, derived
predicates in temporal plans, preferences, constraints, object fluents,
PDDL 1.2 `:extends` and `:timeless`, requirement flags the parser does not know,
and derived predicates that depend on their own negation.

## Numbers

Values are `f64`, as the `pddl` parser stores them. Comparisons treat `a` and
`b` as equal when `|a - b| <= t * max(1, |a|, |b|)`, with a relative tolerance
`t` of `1e-12` by default (`Validator::with_tolerance`), so exactly one of `<`,
`=` and `>` holds for any pair. A comparison that reads a fluent without a
value, divides by zero or overflows is unknown, and conditions follow
three-valued (Kleene) logic: `(or (p) (> (f) 0))` holds when `(p)` does, and
`(and (q) (> (f) 0))` fails on `(q)` when `(q)` is false. A precondition or goal
that stays unknown makes the plan invalid, as does an effect or `when`
condition that reads an undefined value. A derived predicate whose rule is
unknown is not derived; a metric whose value is undefined leaves the plan
valid, without a value. A nullary `total-cost` without an initial value starts at 0, as in
Fast Downward. Updates of one fluent in one step add up when all are
`increase` or `decrease`; any other combination is a conflict. In a metric,
`total-time` is the number of steps. `Report::metric` returns the metric's
value for a valid plan.

## Temporal plans

A temporal plan times every step and gives each durative action its duration,
as VAL reads it: `0.000: (drive r1 base hill) [2.000]`. A durative action
splits into a start and an end event; events at the same time form a
happening. In each happening, every `at start` and `at end` condition,
duration constraint and precondition is checked in the state before it, and
then all effects apply at once. Between happenings, continuous effects change
fluents linearly, and the `over all` conditions of the running actions must hold
on the whole open interval: the validator checks them at every root of their
comparisons and between them, which is exact for linear change. Two events that
interfere (one changes what the other reads or changes) must be at least
epsilon apart, 0.01 by default as in VAL (`Validator::with_epsilon`); an
action's own start and end never interfere. Timed initial literals are events
too. An untimed plan for a temporal domain runs at times 1, 2, 3, and so on;
`total-time` is the makespan.

## PDDL+

Processes run while their precondition holds and add their `#t` rates to those
of the running durative actions; events fire as soon as their precondition
holds, all enabled ones together, and then the validator checks again, so
events cascade (at most 1000 rounds at one time, else `EventCascade`). Between
plan happenings, the validator ends each stretch of fixed dynamics at the first
root of a process or event precondition where one switches, so the switching
times are exact for linear change. Rates that read changing fluents, as in
`(increase (speed) (* #t 9.8))` with `(decrease (height) (* #t (speed)))`, are
nonlinear and `Unsupported`. Processes and events run until the plan's last
step; the `:time` requirement is accepted.

## Diagnostics

Validation stops at the first violation, as VAL's does. The `Violation` names:

- the step, numbered from 1 as VAL counts, with its line in the plan file;
- what fails: an unknown action or object, a wrong number of arguments, an
  object of the wrong type, an unsatisfied precondition or an unmet goal, an
  undefined fluent, a division by zero, an overflow, conflicting updates, a
  missing timestamp or duration, a duration constraint, an `at start` or
  `at end` condition, an `over all` condition with the time it fails, or
  interference with another event;
- for a condition, the most specific part to blame (`Unmet`): the first failing
  conjunct or `forall` instance, every alternative of an `or` or `exists`, or a
  comparison with the values of both sides;
- a repair hint in the style of VAL's "Plan Repair Advice".

The report also traces each step's added and deleted atoms and updated
fluents, with their values before and after.

A domain or problem with an undeclared type, predicate, object or variable, or a
symbol applied to the wrong number of arguments, is rejected with a
`ModelError` before any plan runs. VAL is more lenient there.

## License

Licensed under either of

- the European Union Public Licence 1.2 ([LICENSE-EUPL-1.2.md](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-EUPL-1.2.md)), or
- the Mozilla Public License 2.0 ([LICENSE-MPL-2.0.txt](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-MPL-2.0.txt)),

at your option.
