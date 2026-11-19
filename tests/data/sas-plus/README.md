# Grounding fixtures

The PDDL inputs of the translator fixtures in
[sunsided/sas-plus](https://github.com/sunsided/sas-plus) (`tests/fixtures/translated/`),
used by `tests/sas_plus.rs` to check `pddl::sas_plus::ground`.

| Fixture | Fast Downward operators | Optimal cost |
|---|---|---|
| `sussman-anomaly` | 18 | 3 |
| `gripper-2` | 18 | 3 |
| `gripper-4` | 34 | 9 |
| `bw-large-a` | 648 | not checked |
| `logistics-e` | 756 | not checked |
| `gripper-costs` | 18 | 7 |
| `courier` | 18 | 16 (conditional effects) |
| `get-paid4` | 44 | (conditional effects) |
| `wiring` | 5, plus axioms | 4 (derived predicates; not supported yet) |

The operator counts and optima come from Fast Downward's translator at commit `a744e44`.

## Provenance and licences

- `gripper-costs`, `courier` and `wiring` were written for sas-plus and are licensed MIT OR Apache-2.0.
- The other domains, and all problems except `logistics-e/problem.pddl`, are VHPOP examples under the GNU General Public License, version 2 ([`COPYING.vhpop`](COPYING.vhpop)), as in `../vhpop`.
- `logistics-e/problem.pddl` is a Potoroo example under the Apache License 2.0 ([`LICENSE.potoroo`](LICENSE.potoroo), [`NOTICE.potoroo`](NOTICE.potoroo)), as in `../potoroo`.

These files are test input only. Like the rest of `tests/data/`, they are excluded from the published crate.
