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
| 2. Numeric | numeric fluents, `assign`, `increase`, `decrease`, `scale-up`, `scale-down`, metrics | Not yet |
| 3. Temporal | durative actions, timestamped plans, timed initial literals | Out of scope for now |
| 4. PDDL+ | processes, events, continuous effects | Out of scope for now |

A plan that uses a construct outside the supported levels is reported as
`Unsupported`, never as valid. So are preferences, constraints, object fluents,
PDDL 1.2 `:extends` and `:timeless`, requirement flags the parser does not know,
and derived predicates that depend on their own negation.

## Diagnostics

Validation stops at the first violation, as VAL's does. The `Violation` names:

- the step, numbered from 1 as VAL counts, with its line in the plan file;
- what fails: an unknown action or object, a wrong number of arguments, an
  object of the wrong type, an unsatisfied precondition or an unmet goal;
- for a condition, the most specific part to blame (`Unmet`): the first failing
  conjunct or `forall` instance, or every alternative of an `or` or `exists`;
- a repair hint in the style of VAL's "Plan Repair Advice".

A domain or problem with an undeclared type, predicate, object or variable, or a
symbol applied to the wrong number of arguments, is rejected with a
`ModelError` before any plan runs. VAL is more lenient there.

## License

Licensed under either of

- the European Union Public Licence 1.2 ([LICENSE-EUPL-1.2.md](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-EUPL-1.2.md)), or
- the Mozilla Public License 2.0 ([LICENSE-MPL-2.0.txt](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-MPL-2.0.txt)),

at your option.
