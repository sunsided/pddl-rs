# PDDL parser (work in progress)

A PDDL 3.1 parser implementation in Rust.

## Implemented BNF Elements

Parsers were implemented for the following elements
listed in the paper:

> "Complete BNF description of PDDL 3.1 (completely corrected)",
> _Daniel L. Kovacs_

### Domain Description

- [ ] `<domain>`
- [x] `<require-def>`
- [x] `<require-key>`
- [ ] `<types-def>`
- [ ] `<constants-def>`
- [ ] `<predicates-def>`
- [x] `<atomic formula skeleton>`
- [x] `<predicate>`
- [x] `<variable>`
- [ ] `<atomic function skeleton>`
- [x] `<function-symbol>`
- [x] `<functions-def>`
- [ ] `<function typed list (x)>`
- [ ] `<function type>`
- [ ] `<constraints>`
- [ ] `<structure-def>`
- [x] `<typed list (x)>`
- [x] `<primitive-type>`
- [x] `<type>`
- [x] `<emptyOr (x)>`
- [ ] `<action-def>`
- [x] `<action-symbol>`
- [ ] `<action-def body>`
- [ ] `<pre-GD>`
- [ ] `<pref-GD>`
- [ ] `<pref-name>`
- [ ] `<GD>`
- [ ] `<f-comp>`
- [x] `<literal(t)>`
- [x] `<atomic formula(t)>`
- [x] `<term>`
- [x] `<function-term>`
- [ ] `<f-exp>`
- [ ] `<f-head>`
- [ ] `<binary-op>`
- [ ] `<multi-op>`
- [ ] `<binary-comp>`
- [x] `<name>`
- [x] `<letter>`
- [x] `<any char>`
- [x] `<number>`
- [x] `<digit>`
- [x] `<decimal>`
- [ ] `<effect>`
- [ ] `<c-effect>`
- [ ] `<cond-effect>`
- [ ] `<assign-op>`
- [ ] `<durative-action-def>`
- [ ] `<da-symbol>`
- [ ] `<da-def body>`
- [ ] `<da-GD>`
- [ ] `<pref-timed-GD>`
- [ ] `<timed-GD>`
- [ ] `<time-specifier>`
- [ ] `<interval>`
- [ ] `<duration-constraint>`
- [ ] `<simple-duration-constraint>`
- [ ] `<d-op>`
- [ ] `<d-value>`
- [ ] `<da-effect>`
- [ ] `<timed-effect>`
- [ ] `<f-assign-da>`
- [ ] `<f-exp-da>`
- [ ] `<assign-op-t>`
- [ ] `<f-exp-t>`
- [ ] `<derived-def>`

### Problem Description

- [ ] `<problem>`
- [ ] `<object declaration>`
- [ ] `<init>`
- [ ] `<init-el>`
- [ ] `<basic-function-term>`
- [ ] `<goal>`
- [ ] `<constraints>`
- [ ] `<pref-con-GD>`
- [ ] `<con-GD>`
- [ ] `<metric-spec>`
- [ ] `<optimization>`
- [ ] `<metric-f-exp>`
- [ ] `<length-spec>`

### Lifting Restrictions

- [ ] `<con-GD>`
- [ ] `<con2-GD>`

### Requirements

The following requirements can be parsed. Note that all
requirement specific features are parsed unconditionally. 
A planner needs to ensure that it accepts or rejects a 
plan accordingly based on the stated domain requirements.

- [x] `:strips`
- [x] `:typing`
- [x] `:negative-preconditions`
- [x] `:disjunctive-preconditions`
- [x] `:equality`
- [x] `:existential-preconditions`
- [x] `:universal-preconditions`
- [x] `:quantified-preconditions`
- [x] `:conditional-effects`
- [x] `:fluents`
- [x] `:numeric-fluents`
- [x] `:adl`
- [x] `:durative-actions`
- [x] `:duration-inequalities`
- [x] `:continuous-effects`
- [x] `:derived-predicates`
- [x] `:timed-initial-literals`
- [x] `:preferences`
- [x] `:constraints`
- [x] `:action-costs`

