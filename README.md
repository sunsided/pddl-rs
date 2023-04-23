# PDDL parser (work in progress)

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sunsided/pddl-rs/rust.yml)
![docs.rs](https://img.shields.io/docsrs/pddl)
![Crates.io](https://img.shields.io/crates/v/pddl)
![Crates.io](https://img.shields.io/crates/l/pddl)

A PDDL 3.1 parser implementation in Rust based on [nom](https://crates.io/crates/nom). See [`src/briefcase_world.rs`](src/briefcase_world.rs) for an example.

Parser and domain/problem types can be used separately and will be completely 
decoupled through crate features in the future.

### ⚠️ Caveat Emptor

At this point the parser supports all domain and problem definition
elements required to fully describe a PDDL 3.1 (and earlier) environment.

However, some enum variants are currently inaccessible to external users of the crate.
This is where some work is still required.

## Supported Elements

Parsers were implemented based on the BNF elements listed in the paper:

> "Complete BNF description of PDDL 3.1 (completely corrected)",
> _Daniel L. Kovacs_

### Domain Description

- [x] [`<domain>`](src/parsers/domain.rs)
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
- [x] [`<constraints>`](src/parsers/domain_constraints_def.rs)
- [x] [`<structure-def>`](src/parsers/structure_def.rs)
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
- [x] [`<durative-action-def>`](src/parsers/da_def.rs)
- [x] [`<da-symbol>`](src/parsers/da_symbol.rs)
- [x] [`<da-def body>`](src/parsers/da_def.rs)
- [x] [`<da-GD>`](src/parsers/da_gd.rs)
- [x] [`<pref-timed-GD>`](src/parsers/pref_timed_gd.rs)
- [x] [`<timed-GD>`](src/parsers/timed_gd.rs)
- [x] [`<time-specifier>`](src/parsers/time_specifier.rs)
- [x] [`<interval>`](src/parsers/interval.rs)
- [x] [`<duration-constraint>`](src/parsers/duration_constraint.rs)
- [x] [`<simple-duration-constraint>`](src/parsers/simple_duration_constraint.rs)
- [x] [`<d-op>`](src/parsers/d_op.rs)
- [x] [`<d-value>`](src/parsers/d_value.rs)
- [x] [`<da-effect>`](src/parsers/da_effect.rs)
- [x] [`<timed-effect>`](src/parsers/timed_effect.rs)
- [x] [`<f-assign-da>`](src/parsers/f_assign_da.rs)
- [x] [`<f-exp-da>`](src/parsers/f_exp_da.rs)
- [x] [`<assign-op-t>`](src/parsers/assign_op_t.rs)
- [x] [`<f-exp-t>`](src/parsers/f_exp_t.rs)
- [x] [`<derived-def>`](src/parsers/derived_predicate.rs)

### Problem Description

- [x] [`<problem>`](src/parsers/problem.rs)
- [x] [`<object declaration>`](src/parsers/objects_def.rs)
- [x] [`<init>`](src/parsers/init_def.rs)
- [x] [`<init-el>`](src/parsers/init_el.rs)
- [x] [`<basic-function-term>`](src/parsers/basic_function_term.rs)
- [x] [`<goal>`](src/parsers/goal_def.rs)
- [x] [`<constraints>`](src/parsers/problem_constraints_def.rs)
- [x] [`<pref-con-GD>`](src/parsers/pref_con_gd.rs)
- [x] [~~`<con-GD>`~~](src/parsers/con_gd.rs) (uses embedded modal operators below)
- [x] [`<metric-spec>`](src/parsers/metric_spec.rs)
- [x] [`<optimization>`](src/parsers/optimization.rs)
- [x] [`<metric-f-exp>`](src/parsers/metric_f_exp.rs)
- [x] [`<length-spec>`](src/parsers/length_spec.rs)

### Lifting Restrictions

Using embedded modal operators:

- [x] [`<con-GD>`](src/parsers/con_gd.rs)
- [x] [`<con2-GD>`](src/parsers/con_gd.rs)

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

