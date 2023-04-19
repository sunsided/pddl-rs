# PDDL parser (work in progress)

A PDDL 3.1 parser implementation in Rust.

## Implemented BNF Elements

Parsers were implemented for the following elements
listed in the paper:

> "Complete BNF description of PDDL 3.1 (completely corrected)",
> _Daniel L. Kovacs_

### Domain Description

- [ ] [`<domain>`](src/parsers/domain.rs) — partial implementation missing `<constraints>`.
- [x] [`<require-def>`](src/parsers/predicates_def.rs)
- [x] [`<require-key>`](src/parsers/requirements.rs)
- [x] [`<types-def>`](src/parsers/types_def.rs)
- [x] [`<constants-def>`](src/parsers/constants_def.rs)
- [x] [`<predicates-def>`](src/parsers/predicates_def.rs)
- [x] [`<atomic formula skeleton>`](src/parsers/atomic_formula_skeleton.rs)
- [x] [`<predicate>`](src/parsers/predicate.rs)
- [x] [`<variable>`](src/parsers/variable.rs)
- [x] [`<atomic function skeleton>`](src/parsers/atomic_formula_skeleton.rs)
- [x] [`<function-symbol>`](src/parsers/function_symbol.rs)
- [x] [`<functions-def>`](src/parsers/functions_def.rs)
- [x] [`<function typed list (x)>`](src/parsers/function_typed_list.rs)
- [x] [`<function type>`](src/parsers/function_type.rs)
- [ ] `<constraints>`
- [ ] [`<structure-def>`](src/parsers/structure_def.rs) — partial implementation missing `:durative-actions` and `:derived-actions` variants.
- [x] [`<typed list (x)>`](src/parsers/typed_list.rs)
- [x] [`<primitive-type>`](src/parsers/primitive_type.rs)
- [x] [`<type>`](src/parsers/type.rs)
- [x] [`<emptyOr (x)>`](src/parsers/empty_or.rs)
- [x] [`<action-def>`](src/parsers/action_def.rs)
- [x] [`<action-symbol>`](src/parsers/action_symbol.rs)
- [x] [`<action-def body>`](src/parsers/action_def.rs)
- [x] [`<pre-GD>`](src/parsers/pre_gd.rs)
- [x] [`<pref-GD>`](src/parsers/pref_gd.rs)
- [x] [`<pref-name>`](src/parsers/pref_name.rs)
- [x] [`<GD>`](src/parsers/gd.rs)
- [x] [`<f-comp>`](src/parsers/f_comp.rs)
- [x] [`<literal(t)>`](src/parsers/literal.rs)
- [x] [`<atomic formula(t)>`](src/parsers/atomic_formula.rs)
- [x] [`<term>`](src/parsers/term.rs)
- [x] [`<function-term>`](src/parsers/function_term.rs)
- [x] [`<f-exp>`](src/parsers/f_exp.rs)
- [x] [`<f-head>`](src/parsers/f_head.rs)
- [x] [`<binary-op>`](src/parsers/binary_op.rs)
- [x] [`<multi-op>`](src/parsers/multi_op.rs)
- [x] [`<binary-comp>`](src/parsers/binary_comp.rs)
- [x] [`<name>`](src/parsers/name.rs)
- [x] [`<letter>`](src/parsers/name.rs)
- [x] [`<any char>`](src/parsers/name.rs)
- [x] [`<number>`](src/parsers/number.rs)
- [x] [`<digit>`](src/parsers/number.rs)
- [x] [`<decimal>`](src/parsers/number.rs)
- [x] [`<effect>`](src/parsers/effect.rs)
- [x] [`<c-effect>`](src/parsers/c_effect.rs)
- [x] [`<p-effect>`](src/parsers/p_effect.rs)
- [x] [`<cond-effect>`](src/parsers/cond_effect.rs)
- [x] [`<assign-op>`](src/parsers/assign_op.rs)
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
- [x] [`<f-exp-da>`](src/parsers/f_exp_da.rs)
- [x] [`<assign-op-t>`](src/parsers/assign_op_t.rs)
- [x] [`<f-exp-t>`](src/parsers/f_exp_t.rs)
- [x] [`<derived-def>`](src/parsers/derived_predicate.rs)

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

- [x] [`:strips`](src/parsers/requirements.rs)
- [x] [`:typing`](src/parsers/requirements.rs)
- [x] [`:negative-preconditions`](src/parsers/requirements.rs)
- [x] [`:disjunctive-preconditions`](src/parsers/requirements.rs)
- [x] [`:equality`](src/parsers/requirements.rs)
- [x] [`:existential-preconditions`](src/parsers/requirements.rs)
- [x] [`:universal-preconditions`](src/parsers/requirements.rs)
- [x] [`:quantified-preconditions`](src/parsers/requirements.rs)
- [x] [`:conditional-effects`](src/parsers/requirements.rs)
- [x] [`:fluents`](src/parsers/requirements.rs)
- [x] [`:numeric-fluents`](src/parsers/requirements.rs)
- [x] [`:adl`](src/parsers/requirements.rs)
- [x] [`:durative-actions`](src/parsers/requirements.rs)
- [x] [`:duration-inequalities`](src/parsers/requirements.rs)
- [x] [`:continuous-effects`](src/parsers/requirements.rs)
- [x] [`:derived-predicates`](src/parsers/requirements.rs)
- [x] [`:timed-initial-literals`](src/parsers/requirements.rs)
- [x] [`:preferences`](src/parsers/requirements.rs)
- [x] [`:constraints`](src/parsers/requirements.rs)
- [x] [`:action-costs`](src/parsers/requirements.rs)

