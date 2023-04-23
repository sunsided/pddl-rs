# PDDL parser (work in progress)

A PDDL 3.1 parser implementation in Rust.

## Implemented BNF Elements

Parsers were implemented for the following elements
listed in the paper:

> "Complete BNF description of PDDL 3.1 (completely corrected)",
> _Daniel L. Kovacs_

### Domain Description

- [ ] [`<domain>`](src/parsers/domain/domain.rs) — partial implementation missing `<constraints>`.
- [x] [`<require-def>`](src/parsers/domain/predicates_def.rs)
- [x] [`<require-key>`](src/parsers/domain/requirements.rs)
- [x] [`<types-def>`](src/parsers/domain/types_def.rs)
- [x] [`<constants-def>`](src/parsers/domain/constants_def.rs)
- [x] [`<predicates-def>`](src/parsers/domain/predicates_def.rs)
- [x] [`<atomic formula skeleton>`](src/parsers/domain/atomic_formula_skeleton.rs)
- [x] [`<predicate>`](src/parsers/domain/predicate.rs)
- [x] [`<variable>`](src/parsers/domain/variable.rs)
- [x] [`<atomic function skeleton>`](src/parsers/domain/atomic_formula_skeleton.rs)
- [x] [`<function-symbol>`](src/parsers/domain/function_symbol.rs)
- [x] [`<functions-def>`](src/parsers/domain/functions_def.rs)
- [x] [`<function typed list (x)>`](src/parsers/domain/function_typed_list.rs)
- [x] [`<function type>`](src/parsers/domain/function_type.rs)
- [ ] `<constraints>`
- [x] [`<structure-def>`](src/parsers/domain/structure_def.rs)
- [x] [`<typed list (x)>`](src/parsers/domain/typed_list.rs)
- [x] [`<primitive-type>`](src/parsers/domain/primitive_type.rs)
- [x] [`<type>`](src/parsers/domain/type.rs)
- [x] [`<emptyOr (x)>`](src/parsers/domain/empty_or.rs)
- [x] [`<action-def>`](src/parsers/domain/action_def.rs)
- [x] [`<action-symbol>`](src/parsers/domain/action_symbol.rs)
- [x] [`<action-def body>`](src/parsers/domain/action_def.rs)
- [x] [`<pre-GD>`](src/parsers/domain/pre_gd.rs)
- [x] [`<pref-GD>`](src/parsers/domain/pref_gd.rs)
- [x] [`<pref-name>`](src/parsers/domain/pref_name.rs)
- [x] [`<GD>`](src/parsers/domain/gd.rs)
- [x] [`<f-comp>`](src/parsers/domain/f_comp.rs)
- [x] [`<literal(t)>`](src/parsers/domain/literal.rs)
- [x] [`<atomic formula(t)>`](src/parsers/domain/atomic_formula.rs)
- [x] [`<term>`](src/parsers/domain/term.rs)
- [x] [`<function-term>`](src/parsers/domain/function_term.rs)
- [x] [`<f-exp>`](src/parsers/domain/f_exp.rs)
- [x] [`<f-head>`](src/parsers/domain/f_head.rs)
- [x] [`<binary-op>`](src/parsers/domain/binary_op.rs)
- [x] [`<multi-op>`](src/parsers/domain/multi_op.rs)
- [x] [`<binary-comp>`](src/parsers/domain/binary_comp.rs)
- [x] [`<name>`](src/parsers/domain/name.rs)
- [x] [`<letter>`](src/parsers/domain/name.rs)
- [x] [`<any char>`](src/parsers/domain/name.rs)
- [x] [`<number>`](src/parsers/domain/number.rs)
- [x] [`<digit>`](src/parsers/domain/number.rs)
- [x] [`<decimal>`](src/parsers/domain/number.rs)
- [x] [`<effect>`](src/parsers/domain/effect.rs)
- [x] [`<c-effect>`](src/parsers/domain/c_effect.rs)
- [x] [`<p-effect>`](src/parsers/domain/p_effect.rs)
- [x] [`<cond-effect>`](src/parsers/domain/cond_effect.rs)
- [x] [`<assign-op>`](src/parsers/domain/assign_op.rs)
- [x] [`<durative-action-def>`](src/parsers/domain/da_def.rs)
- [x] [`<da-symbol>`](src/parsers/domain/da_symbol.rs)
- [x] [`<da-def body>`](src/parsers/domain/da_def.rs)
- [x] [`<da-GD>`](src/parsers/domain/da_gd.rs)
- [x] [`<pref-timed-GD>`](src/parsers/domain/pref_timed_gd.rs)
- [x] [`<timed-GD>`](src/parsers/domain/timed_gd.rs)
- [x] [`<time-specifier>`](src/parsers/domain/time_specifier.rs)
- [x] [`<interval>`](src/parsers/domain/interval.rs)
- [x] [`<duration-constraint>`](src/parsers/domain/duration_constraint.rs)
- [x] [`<simple-duration-constraint>`](src/parsers/domain/simple_duration_constraint.rs)
- [x] [`<d-op>`](src/parsers/domain/d_op.rs)
- [x] [`<d-value>`](src/parsers/domain/d_value.rs)
- [x] [`<da-effect>`](src/parsers/domain/da_effect.rs)
- [x] [`<timed-effect>`](src/parsers/domain/timed_effect.rs)
- [x] [`<f-assign-da>`](src/parsers/domain/f_assign_da.rs)
- [x] [`<f-exp-da>`](src/parsers/domain/f_exp_da.rs)
- [x] [`<assign-op-t>`](src/parsers/domain/assign_op_t.rs)
- [x] [`<f-exp-t>`](src/parsers/domain/f_exp_t.rs)
- [x] [`<derived-def>`](src/parsers/domain/derived_predicate.rs)

### Problem Description

- [ ] `<problem>`
- [ ] `<object declaration>`
- [ ] `<init>`
- [ ] `<init-el>`
- [ ] `<basic-function-term>`
- [ ] `<goal>`
- [ ] `<constraints>`
- [ ] `<pref-con-GD>`
- [ ] ~~`<con-GD>`~~ see "Lifting Restrictions" below
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

- [x] [`:strips`](src/parsers/domain/requirements.rs)
- [x] [`:typing`](src/parsers/domain/requirements.rs)
- [x] [`:negative-preconditions`](src/parsers/domain/requirements.rs)
- [x] [`:disjunctive-preconditions`](src/parsers/domain/requirements.rs)
- [x] [`:equality`](src/parsers/domain/requirements.rs)
- [x] [`:existential-preconditions`](src/parsers/domain/requirements.rs)
- [x] [`:universal-preconditions`](src/parsers/domain/requirements.rs)
- [x] [`:quantified-preconditions`](src/parsers/domain/requirements.rs)
- [x] [`:conditional-effects`](src/parsers/domain/requirements.rs)
- [x] [`:fluents`](src/parsers/domain/requirements.rs)
- [x] [`:numeric-fluents`](src/parsers/domain/requirements.rs)
- [x] [`:adl`](src/parsers/domain/requirements.rs)
- [x] [`:durative-actions`](src/parsers/domain/requirements.rs)
- [x] [`:duration-inequalities`](src/parsers/domain/requirements.rs)
- [x] [`:continuous-effects`](src/parsers/domain/requirements.rs)
- [x] [`:derived-predicates`](src/parsers/domain/requirements.rs)
- [x] [`:timed-initial-literals`](src/parsers/domain/requirements.rs)
- [x] [`:preferences`](src/parsers/domain/requirements.rs)
- [x] [`:constraints`](src/parsers/domain/requirements.rs)
- [x] [`:action-costs`](src/parsers/domain/requirements.rs)

