//! Grounds the fixtures in `tests/data/sas-plus` (the PDDL inputs of the
//! `sas-plus` translator fixtures) and checks the results against known optima.

use pddl::sas_plus::{ground, ground_scaled, GroundError, MAX_DELETE_PIECES};
use pddl::{Domain, Parser, Problem};
use sas_plus::fdr::SasFile;

fn ground_fixture(name: &str) -> Result<SasFile, GroundError> {
    let dir = format!("{}/tests/data/sas-plus/{name}", env!("CARGO_MANIFEST_DIR"));
    let read = |file: &str| std::fs::read_to_string(format!("{dir}/{file}")).unwrap();
    let domain = Domain::from_str(&read("domain.pddl")).unwrap();
    let problem = Problem::from_str(&read("problem.pddl")).unwrap();
    ground(&domain, &problem)
}

/// Grounds a fixture and checks that the output parses back to the same file.
fn ground_round_trip(name: &str) -> SasFile {
    let file = ground_fixture(name).unwrap();
    let text = file.to_string();
    assert_eq!(
        text.parse::<SasFile>().unwrap(),
        file,
        "{name} does not round-trip"
    );
    file
}

/// Checks the optimal plan cost with exhaustive search.
fn assert_optimum(name: &str, optimum: u64) {
    let task = ground_round_trip(name).to_task();
    task.validate().unwrap();
    let h_star = task.perfect_heuristic();
    assert_eq!(h_star(&task.initial), Some(optimum), "{name}");
}

#[test]
fn sussman_anomaly() {
    assert_optimum("sussman-anomaly", 3);
}

#[test]
fn gripper_2() {
    assert_optimum("gripper-2", 3);
}

#[test]
fn gripper_4() {
    assert_optimum("gripper-4", 9);
}

#[test]
fn gripper_costs() {
    let file = ground_round_trip("gripper-costs");
    assert!(file.use_metric);
    assert_optimum("gripper-costs", 7);
}

#[test]
fn conditional_effects() {
    for name in ["courier", "get-paid4"] {
        let file = ground_round_trip(name);
        assert!(
            file.operators
                .iter()
                .flat_map(|op| &op.effects)
                .any(|effect| !effect.conditions.is_empty()),
            "{name} has no conditional effects"
        );
    }
}

#[test]
fn courier_costs_come_from_static_functions() {
    let file = ground_round_trip("courier");
    let costs: Vec<_> = file
        .operators
        .iter()
        .filter(|op| op.name.starts_with("drive "))
        .map(|op| op.cost)
        .collect();
    assert_ne!(costs, [] as [u32; 0]);
    assert!(costs.iter().all(|&cost| cost > 1), "{costs:?}");
}

#[test]
fn larger_problems() {
    for name in ["bw-large-a", "logistics-e"] {
        assert!(!ground_round_trip(name).operators.is_empty(), "{name}");
    }
}

/// Fast Downward's translator finds the same operators; only the variables
/// differ, because it merges mutex atoms into multi-valued variables.
#[test]
fn operator_counts_match_fast_downward() {
    for (name, operators) in [
        ("sussman-anomaly", 18),
        ("gripper-2", 18),
        ("gripper-4", 34),
        ("bw-large-a", 648),
        ("logistics-e", 756),
        ("gripper-costs", 18),
        ("courier", 18),
        ("get-paid4", 44),
    ] {
        assert_eq!(
            ground_fixture(name).unwrap().operators.len(),
            operators,
            "{name}"
        );
    }
}

#[test]
fn derived_predicates_are_unsupported() {
    assert_eq!(
        ground_fixture("wiring"),
        Err(GroundError::Unsupported("derived predicates"))
    );
}

/// Grounds a domain and problem given as text.
fn ground_str(domain: &str, problem: &str) -> Result<SasFile, GroundError> {
    let domain = Domain::from_str(domain).unwrap_or_else(|e| panic!("{domain}: {e:?}"));
    let problem = Problem::from_str(problem).unwrap_or_else(|e| panic!("{problem}: {e:?}"));
    ground(&domain, &problem)
}

/// A domain with a type `t`, the predicates `(p ?x)` and `(q)`, the function
/// `(f)` and the given extra sections.
fn domain_with(sections: &str) -> String {
    format!(
        "(define (domain d) (:types t) (:predicates (p ?x) (q)) (:functions (f) (total-cost))
           {sections})"
    )
}

/// A problem for [`domain_with`] with the objects `a` and `b` of type `t`.
fn problem_with(init: &str, goal: &str, extra: &str) -> String {
    format!(
        "(define (problem pr) (:domain d) (:objects a b - t)
           (:init {init}) (:goal {goal}) {extra})"
    )
}

/// An action `act` over `?x - t` with the given precondition and effect.
fn action(pre: &str, effect: &str) -> String {
    domain_with(&format!(
        "(:action act :parameters (?x - t) :precondition {pre} :effect {effect})"
    ))
}

/// Asserts that grounding fails on the given unsupported construct.
fn assert_unsupported(domain: &str, problem: &str, construct: &'static str) {
    assert_eq!(
        ground_str(domain, problem),
        Err(GroundError::Unsupported(construct)),
        "{domain}\n{problem}"
    );
}

#[test]
fn unsupported_sections_are_rejected() {
    let plain = problem_with("", "(p a)", "");
    let simple = action("(q)", "(p ?x)");
    for (domain, problem, construct) in [
        (
            domain_with(
                "(:durative-action dur :parameters () :duration (= ?duration 1)
                   :condition (at start (q)) :effect (at end (q)))",
            ),
            plain.clone(),
            "durative actions",
        ),
        (
            "(define (domain d) (:extends other) (:predicates (p ?x)))".to_string(),
            plain.clone(),
            "domain extension",
        ),
        (
            domain_with("(:constraints (always (q)))"),
            plain.clone(),
            "constraints",
        ),
        (
            "(define (domain d) (:types t) (:predicates (p ?x)) (:timeless (p a)))".to_string(),
            plain.clone(),
            "timeless literals",
        ),
        (
            simple.clone(),
            problem_with("", "(p a)", "(:metric maximize (total-cost))"),
            "metrics other than minimizing total-cost",
        ),
        (
            simple.clone(),
            problem_with("(at 5 (q))", "(p a)", ""),
            "timed initial literals",
        ),
        (
            simple.clone(),
            problem_with("(= (f) a)", "(p a)", ""),
            "object fluents",
        ),
    ] {
        assert_unsupported(&domain, &problem, construct);
    }
}

#[test]
fn unsupported_formulas_are_rejected() {
    let plain = problem_with("", "(p a)", "");
    let cost_problem = problem_with("", "(p a)", "(:metric minimize (total-cost))");
    for (pre, effect, problem, construct) in [
        ("(preference pref (q))", "(p ?x)", &plain, "preferences"),
        (
            "(not (and (q) (q)))",
            "(p ?x)",
            &plain,
            "not over a non-atomic formula",
        ),
        (
            "(forall (?y - t) (exists (?z - t) (p ?z)))",
            "(p ?x)",
            &plain,
            "nested exists",
        ),
        ("(or (q) (p ?x))", "(p ?x)", &plain, "or"),
        ("(imply (q) (p ?x))", "(p ?x)", &plain, "imply"),
        ("(> (f) 1)", "(p ?x)", &plain, "numeric fluents"),
        ("(p (f))", "(p ?x)", &plain, "object fluents"),
        (
            "(q)",
            "(when (q) (increase (total-cost) 1))",
            &cost_problem,
            "conditional action costs",
        ),
        (
            "(q)",
            "(increase (total-cost) (+ (f) 1))",
            &cost_problem,
            "numeric fluents",
        ),
        ("(q)", "(increase (f) 1)", &plain, "numeric fluents"),
        ("(q)", "(assign (f) a)", &plain, "object fluents"),
        ("(q)", "(= ?x ?x)", &plain, "equality in effects"),
    ] {
        assert_unsupported(&action(pre, effect), problem, construct);
    }
}

#[test]
fn undeclared_names_are_rejected() {
    let plain = problem_with("", "(p a)", "");
    assert_eq!(
        ground_str(
            &action("(q)", "(p ?x)"),
            "(define (problem pr) (:domain d) (:objects a - ghost) (:init) (:goal (p a)))"
        ),
        Err(GroundError::UnknownType("ghost".into()))
    );
    assert_eq!(
        ground_str(
            &action("(q)", "(p ?x)"),
            &problem_with("(p ghost)", "(p a)", "")
        ),
        Err(GroundError::UnknownObject("ghost".into()))
    );
    assert_eq!(
        ground_str(&action("(p ghost)", "(p ?x)"), &plain),
        Err(GroundError::UnknownObject("ghost".into()))
    );
    assert_eq!(
        ground_str(&action("(p ?y)", "(p ?x)"), &plain),
        Err(GroundError::UnknownVariable("y".into()))
    );
    assert_eq!(
        ground_str(
            &action("(q)", "(increase (total-cost) (f ?y))"),
            &problem_with("", "(p a)", "(:metric minimize (total-cost))")
        ),
        Err(GroundError::UnknownVariable("y".into()))
    );
}

#[test]
fn invalid_costs_are_rejected() {
    for (cost, init) in [("0.5", "(q)"), ("(f)", "(q)"), ("(f)", "(q) (= (f) 1.5)")] {
        assert_eq!(
            ground_str(
                &action(
                    "(q)",
                    &format!("(and (p ?x) (increase (total-cost) {cost}))")
                ),
                &problem_with(init, "(p a)", "(:metric minimize (total-cost))"),
            ),
            Err(GroundError::InvalidCost {
                action: "act a".into()
            }),
            "{cost} with {init}"
        );
    }
}

#[test]
fn costs_come_from_functions_without_arguments() {
    let file = ground_str(
        &action("(q)", "(and (p ?x) (increase (total-cost) (f)))"),
        &problem_with("(q) (= (f) 4)", "(p a)", "(:metric minimize (total-cost))"),
    )
    .unwrap();
    assert!(file.operators.iter().all(|op| op.cost == 4));
}

#[test]
fn quantified_preconditions_and_goals() {
    let domain = domain_with(
        "(:action fill :parameters (?x - t) :precondition (forall (?y - t) (not (p ?y)))
           :effect (and (p ?x) (when (forall (?y - t) (p ?y)) (q))))
         (:action pick :parameters ()
           :precondition (and (exists (?y - t) (and (p ?y) (q))))
           :effect (not (q)))",
    );
    let file = ground_str(
        &domain,
        &problem_with(
            "(q) (not (p b))",
            "(and (forall (?x - t) (p ?x)) (not (q)))",
            "",
        ),
    )
    .unwrap();
    let mut names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
    names.sort_unstable();
    // As in Fast Downward, the `exists` variable is not part of the name.
    assert_eq!(names, ["fill a", "fill b", "pick", "pick"]);
}

#[test]
fn contradictory_and_unreachable_preconditions_drop_actions() {
    let domain = "(define (domain d) (:types t) (:predicates (p ?x) (q) (r ?x))
         (:action set :parameters (?x - t) :precondition (q) :effect (p ?x))
         (:action never :parameters (?x - t) :precondition (and (p ?x) (not (p ?x)))
           :effect (not (q)))
         (:action stuck :parameters (?x - t) :precondition (r ?x) :effect (not (p ?x)))
         (:action free :parameters (?x - t) :precondition (not (r ?x)) :effect (r ?x)))";
    let file = ground_str(domain, &problem_with("(q)", "(p a)", "")).unwrap();
    assert!(file
        .operators
        .iter()
        .all(|op| !op.name.starts_with("never")));
}

#[test]
fn shared_supertypes_list_objects_once() {
    let file = ground_str(
        "(define (domain d) (:types u v - t t) (:constants a - u) (:predicates (p ?x))
           (:action set :parameters (?x - (either t u)) :precondition () :effect (p ?x)))",
        "(define (problem pr) (:domain d) (:objects a - u b - v) (:init)
           (:goal (p a)))",
    );
    let file = file.unwrap();
    assert_eq!(file.operators.len(), 2);
}

#[test]
fn redeclared_objects_keep_every_type() {
    let file = ground_str(
        "(define (domain d) (:types u v) (:constants a - u) (:predicates (p ?x))
           (:action set :parameters (?x - v) :precondition () :effect (p ?x)))",
        "(define (problem pr) (:domain d) (:objects a b - v) (:init) (:goal (p a)))",
    )
    .unwrap();
    let mut names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
    names.sort_unstable();
    assert_eq!(names, ["set a", "set b"]);
}

/// The atoms an operator's effects set, as `Atom r(a)` or `NegatedAtom r(a)`.
fn effect_values<'a>(file: &'a SasFile, op: &str) -> Vec<&'a str> {
    file.operators
        .iter()
        .filter(|o| o.name == op)
        .flat_map(|o| &o.effects)
        .map(|e| file.variables[e.var].values[usize::try_from(e.post).unwrap()].as_str())
        .collect()
}

#[test]
fn exists_binders_do_not_capture_parameters() {
    let file = ground_str(
        "(define (domain d) (:types t) (:predicates (p ?x) (r ?x))
           (:action act :parameters (?x - t) :precondition (exists (?x - t) (p ?x))
             :effect (r ?x)))",
        &problem_with("(p b)", "(r a)", ""),
    )
    .unwrap();
    assert_eq!(effect_values(&file, "act a"), ["Atom r(a)"]);
    assert_eq!(effect_values(&file, "act b"), ["Atom r(b)"]);
}

#[test]
fn sibling_exists_binders_are_independent() {
    let file = ground_str(
        "(define (domain d) (:types t) (:predicates (p ?x) (s ?x) (q))
           (:action act :parameters ()
             :precondition (and (exists (?y - t) (p ?y)) (exists (?y - t) (s ?y)))
             :effect (q)))",
        &problem_with("(p a) (s b)", "(q)", ""),
    )
    .unwrap();
    assert_eq!(effect_values(&file, "act"), ["Atom q()"]);
}

/// Each effect of an operator as its conditions and the value it sets.
fn effects_of<'a>(file: &'a SasFile, op: &str) -> Vec<(Vec<&'a str>, &'a str)> {
    let value = |var: usize, value: u32| {
        file.variables[var].values[usize::try_from(value).unwrap()].as_str()
    };
    file.operators
        .iter()
        .filter(|o| o.name == op)
        .flat_map(|o| &o.effects)
        .map(|e| {
            let conds = e.conditions.iter().map(|c| value(c.var, c.value)).collect();
            (conds, value(e.var, e.post))
        })
        .collect()
}

#[test]
fn adds_win_over_deletes_under_other_conditions() {
    let file = ground_str(
        "(define (domain d) (:predicates (p) (q) (r))
           (:action setq :parameters () :precondition () :effect (and (q) (r)))
           (:action act :parameters () :precondition ()
             :effect (and (not (p)) (when (q) (p))))
           (:action act2 :parameters () :precondition ()
             :effect (and (when (r) (not (p))) (when (q) (p)))))",
        "(define (problem pr) (:domain d) (:init (p)) (:goal (and (p) (q))))",
    )
    .unwrap();
    let mut act = effects_of(&file, "act");
    act.sort();
    assert_eq!(
        act,
        [
            (vec!["Atom q()"], "Atom p()"),
            (vec!["NegatedAtom q()"], "NegatedAtom p()"),
        ]
    );
    let mut act2 = effects_of(&file, "act2");
    act2.sort();
    assert_eq!(
        act2,
        [
            (vec!["Atom q()"], "Atom p()"),
            (vec!["NegatedAtom q()", "Atom r()"], "NegatedAtom p()"),
        ]
    );
}

#[test]
fn delete_pieces_covered_by_others_are_dropped() {
    // `(q) and (r)` splits the delete into `not q` and `not r`. `(q)` then
    // narrows `not r` to `not r and not q`, which `not q` already covers.
    let file = ground_str(
        "(define (domain d) (:predicates (p) (q) (r))
           (:action setq :parameters () :precondition () :effect (and (q) (r)))
           (:action act :parameters () :precondition ()
             :effect (and (not (p)) (when (and (q) (r)) (p)) (when (q) (p)))))",
        "(define (problem pr) (:domain d) (:init (p)) (:goal (and (p) (q))))",
    )
    .unwrap();
    let mut act = effects_of(&file, "act");
    act.sort();
    assert_eq!(
        act,
        [
            (vec!["Atom q()"], "Atom p()"),
            (vec!["Atom q()", "Atom r()"], "Atom p()"),
            (vec!["NegatedAtom q()"], "NegatedAtom p()"),
        ]
    );
}

/// A domain whose `act` deletes `(p)` unless some `(r ?y) and (s ?y)` holds,
/// with a problem of `objects` objects.
fn overlapping_adds(objects: usize) -> (String, String) {
    let domain = "(define (domain d) (:types t) (:predicates (p) (r ?y) (s ?y))
       (:action mark :parameters (?y - t) :precondition () :effect (and (r ?y) (s ?y)))
       (:action act :parameters () :precondition ()
         :effect (and (not (p)) (forall (?y - t) (when (and (r ?y) (s ?y)) (p))))))";
    let names: Vec<_> = (0..objects).map(|i| format!("o{i}")).collect();
    let problem = format!(
        "(define (problem pr) (:domain d) (:objects {} - t) (:init (p)) (:goal (p)))",
        names.join(" ")
    );
    (domain.to_string(), problem)
}

#[test]
fn delete_splits_are_bounded() {
    // Each of the n adds doubles the pieces: 2^10 fits, 2^11 doesn't.
    assert_eq!(MAX_DELETE_PIECES, 1 << 10);
    let (domain, problem) = overlapping_adds(10);
    let file = ground_str(&domain, &problem).unwrap();
    let deletes = effects_of(&file, "act")
        .into_iter()
        .filter(|(_, post)| *post == "NegatedAtom p()")
        .count();
    assert_eq!(deletes, 1 << 10);

    let (domain, problem) = overlapping_adds(11);
    assert_eq!(
        ground_str(&domain, &problem),
        Err(GroundError::ConflictingEffects {
            action: "act".to_string()
        })
    );
}

#[test]
fn unreachable_groundings_are_never_built() {
    // `(r b)` never becomes true, so `go b` is never grounded and its missing
    // cost `(c b)` is not an error.
    let file = ground_str(
        "(define (domain d) (:types t) (:predicates (r ?x) (q ?x)) (:functions (c ?x) (total-cost))
           (:action mk :parameters () :precondition () :effect (r a))
           (:action go :parameters (?x - t) :precondition (r ?x)
             :effect (and (q ?x) (increase (total-cost) (c ?x)))))",
        "(define (problem pr) (:domain d) (:objects a b - t)
           (:init (= (c a) 2) (= (total-cost) 0)) (:goal (q a)) (:metric minimize (total-cost)))",
    )
    .unwrap();
    let mut names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
    names.sort_unstable();
    assert_eq!(names, ["go a", "mk"]);
}

/// From `a`, reach `c` directly for `direct`, or through `b` for `step` twice.
fn decimal_costs(direct: &str, step: &str) -> (Domain, Problem) {
    let domain = Domain::from_str(
        "(define (domain d) (:requirements :action-costs) (:predicates (at ?x) (road ?x ?y))
           (:functions (len ?x ?y) (total-cost))
           (:action go :parameters (?x ?y) :precondition (and (at ?x) (road ?x ?y))
             :effect (and (not (at ?x)) (at ?y) (increase (total-cost) (len ?x ?y)))))",
    )
    .unwrap();
    let problem = Problem::from_str(&format!(
        "(define (problem p) (:domain d) (:objects a b c)
           (:init (at a) (road a c) (road a b) (road b c)
                  (= (len a c) {direct}) (= (len a b) {step}) (= (len b c) {step}) (= (total-cost) 0))
           (:goal (at c)) (:metric minimize (total-cost)))"
    ))
    .unwrap();
    (domain, problem)
}

#[test]
fn decimal_costs_scale_to_exact_integers() {
    // 0.1 + 0.1 beats 0.3: scaled by 10, 1 + 1 beats 3.
    let (domain, problem) = decimal_costs("0.3", "0.1");
    let scaled = ground_scaled(&domain, &problem).unwrap();
    assert_eq!(scaled.cost_scale, 10);
    let mut costs: Vec<_> = scaled
        .file
        .operators
        .iter()
        .map(|op| (op.name.as_str(), op.cost))
        .collect();
    costs.sort_unstable();
    assert_eq!(costs, [("go a b", 1), ("go a c", 3), ("go b c", 1)]);
    let task = scaled.file.to_task();
    assert_eq!(task.perfect_heuristic()(&task.initial), Some(2));

    // `ground` still rejects costs that aren't integers.
    assert!(matches!(
        ground(&domain, &problem),
        Err(GroundError::InvalidCost { .. })
    ));
}

#[test]
fn decimal_cost_sums_stay_exact() {
    // In f64, 0.1 + 0.2 is 0.30000000000000004; summed exactly it is 3 tenths.
    let domain = Domain::from_str(
        "(define (domain d) (:requirements :action-costs) (:predicates (done))
           (:functions (total-cost))
           (:action a :parameters () :precondition ()
             :effect (and (done) (increase (total-cost) 0.1) (increase (total-cost) 0.2))))",
    )
    .unwrap();
    let problem = Problem::from_str(
        "(define (problem p) (:domain d) (:init (= (total-cost) 0)) (:goal (done))
           (:metric minimize (total-cost)))",
    )
    .unwrap();
    let scaled = ground_scaled(&domain, &problem).unwrap();
    assert_eq!(scaled.cost_scale, 10);
    assert_eq!(scaled.file.operators[0].cost, 3);
}

#[test]
fn integer_costs_ground_as_before() {
    for name in ["gripper-costs", "courier"] {
        let (domain, problem) = (
            Domain::from_str(
                &std::fs::read_to_string(format!("tests/data/sas-plus/{name}/domain.pddl"))
                    .unwrap(),
            )
            .unwrap(),
            Problem::from_str(
                &std::fs::read_to_string(format!("tests/data/sas-plus/{name}/problem.pddl"))
                    .unwrap(),
            )
            .unwrap(),
        );
        let scaled = ground_scaled(&domain, &problem).unwrap();
        assert_eq!(scaled.cost_scale, 1, "{name}");
        assert_eq!(scaled.file, ground(&domain, &problem).unwrap(), "{name}");
    }
}

#[test]
fn scaled_costs_that_do_not_fit_are_errors() {
    // 4 294 967.296 at 3 places is u32::MAX + 1.
    let (domain, problem) = decimal_costs("4294967.296", "1");
    assert_eq!(
        ground_scaled(&domain, &problem),
        Err(GroundError::CostOutOfRange {
            action: "go a c".into(),
            places: 3
        })
    );
    // 10 places: the scale itself is more than a u32 holds.
    let (domain, problem) = decimal_costs("0.0000000001", "1");
    assert!(matches!(
        ground_scaled(&domain, &problem),
        Err(GroundError::CostOutOfRange { places: 10, .. })
    ));
}

/// A domain whose `act` adds `(done)` for the summed `costs`, and whose
/// `idle` only re-adds `(ready)`, which it needs, for `idle_cost`.
fn cost_sum(costs: &[&str], idle_cost: &str) -> (Domain, Problem) {
    let increases = costs.iter().fold(String::new(), |mut text, cost| {
        text.push_str(" (increase (total-cost) ");
        text.push_str(cost);
        text.push(')');
        text
    });
    let domain = Domain::from_str(&format!(
        "(define (domain d) (:requirements :action-costs) (:predicates (done) (ready))
           (:functions (total-cost))
           (:action act :parameters () :precondition (ready) :effect (and (done){increases}))
           (:action idle :parameters () :precondition (ready)
             :effect (and (ready) (increase (total-cost) {idle_cost}))))"
    ))
    .unwrap();
    let problem = Problem::from_str(
        "(define (problem p) (:domain d) (:init (ready) (= (total-cost) 0)) (:goal (done))
           (:metric minimize (total-cost)))",
    )
    .unwrap();
    (domain, problem)
}

#[test]
fn decimal_summands_of_whole_costs_are_whole() {
    // 0.5 + 0.5 is exactly 1: `ground` accepts it and the scale stays 1.
    let (domain, problem) = cost_sum(&["0.5", "0.5"], "1");
    assert_eq!(ground(&domain, &problem).unwrap().operators[0].cost, 1);
    assert_eq!(ground_scaled(&domain, &problem).unwrap().cost_scale, 1);
}

#[test]
fn dropped_operators_do_not_set_the_scale() {
    // `idle` changes nothing, so it isn't in the task, and its cost neither
    // makes `ground` fail nor sets the scale; not even when it can't fit.
    for idle in [
        "0.5",
        "4294967.296",
        "100000000000000000000000000000000000000000",
        "0.00000000000000000000000000000000000000001",
    ] {
        let (domain, problem) = cost_sum(&["2"], idle);
        let file = ground(&domain, &problem).unwrap();
        assert_eq!(file.operators.len(), 1, "{idle}");
        assert_eq!(file.operators[0].cost, 2, "{idle}");
        assert_eq!(
            ground_scaled(&domain, &problem).unwrap().cost_scale,
            1,
            "{idle}"
        );
    }
}

#[test]
fn ground_reports_costs_beyond_u32() {
    let (domain, problem) = cost_sum(&["5000000000"], "1");
    assert_eq!(
        ground(&domain, &problem),
        Err(GroundError::CostOutOfRange {
            action: "act".into(),
            places: 0
        })
    );
    // With `ground`, decimal places are invalid before they are out of range.
    let (domain, problem) = cost_sum(&["0.0000000001"], "1");
    assert!(matches!(
        ground(&domain, &problem),
        Err(GroundError::InvalidCost { .. })
    ));
}

#[test]
fn costs_with_too_many_digits_are_out_of_range() {
    // 39 places: the digits alone don't fit, let alone the scale.
    let (domain, problem) = cost_sum(&["0.000000000000000000000000000000000000001"], "1");
    assert_eq!(
        ground_scaled(&domain, &problem),
        Err(GroundError::CostOutOfRange {
            action: "act".into(),
            places: 39
        })
    );
    // With `ground`, decimal places are invalid before they are out of range.
    assert_eq!(
        ground(&domain, &problem),
        Err(GroundError::InvalidCost {
            action: "act".into()
        })
    );
    // A whole number with more digits than fit is out of range too.
    let (domain, problem) = cost_sum(&["100000000000000000000000000000000000000000"], "1");
    assert_eq!(
        ground(&domain, &problem),
        Err(GroundError::CostOutOfRange {
            action: "act".into(),
            places: 0
        })
    );
}

#[test]
fn cost_overflow_does_not_depend_on_the_order_of_the_terms() {
    // 2e38 + 2e38 overflows u128; whether 0.5 is added before or after the
    // overflow must not change the error.
    let big = "200000000000000000000000000000000000000";
    for costs in [[big, big, "0.5"], ["0.5", big, big]] {
        let (domain, problem) = cost_sum(&costs, "1");
        assert_eq!(
            ground(&domain, &problem),
            Err(GroundError::InvalidCost {
                action: "act".into()
            }),
            "{costs:?}"
        );
        assert_eq!(
            ground_scaled(&domain, &problem),
            Err(GroundError::CostOutOfRange {
                action: "act".into(),
                places: 1
            }),
            "{costs:?}"
        );
    }
}

/// Formats each number of `range` and joins the results with spaces.
fn spaced(range: std::ops::Range<usize>, format: impl Fn(usize) -> String) -> String {
    range.map(format).collect::<Vec<_>>().join(" ")
}

#[test]
fn exists_binders_do_not_duplicate_operators() {
    // Each of the 25 bindings of `?y ?z` grounds to the same operator.
    let objects = spaced(0..5, |i| format!("o{i}"));
    let init = spaced(0..5, |i| format!("(s o{i})"));
    let file = ground_str(
        "(define (domain d) (:predicates (s ?x) (q))
           (:action act :parameters () :precondition (exists (?y ?z) (and (s ?y) (s ?z)))
             :effect (q)))",
        &format!(
            "(define (problem p) (:domain d) (:objects {objects}) (:init {init}) (:goal (q)))"
        ),
    )
    .unwrap();
    let names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
    assert_eq!(names, ["act"]);
}

#[test]
fn actions_declared_twice_do_not_duplicate_operators() {
    // The two `a` are equal but not adjacent: `b` is declared between them.
    let file = ground_str(
        "(define (domain d) (:predicates (q) (r))
           (:action a :parameters () :effect (q))
           (:action b :parameters () :effect (r))
           (:action a :parameters () :effect (q)))",
        "(define (problem p) (:domain d) (:init) (:goal (and (q) (r))))",
    )
    .unwrap();
    let names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
    assert_eq!(names, ["a", "b"]);
}

#[test]
fn exists_binders_keep_distinct_operators() {
    // The binder picks which fluent atom the operator needs, so the two
    // groundings differ and both stay.
    let file = ground_str(
        "(define (domain d) (:predicates (s ?x) (q))
           (:action mk :parameters (?x) :effect (s ?x))
           (:action act :parameters () :precondition (exists (?y) (s ?y)) :effect (q)))",
        "(define (problem p) (:domain d) (:objects a b) (:init) (:goal (q)))",
    )
    .unwrap();
    let names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
    assert_eq!(names, ["mk a", "mk b", "act", "act"]);
}

#[test]
fn long_chains_ground_every_step() {
    // Each step makes the next one reachable, so reachability takes one round
    // per node. Recomputing every round in full took ~1.9 s (release) here;
    // trying only the bindings that use a newly reachable atom takes ~14 ms.
    let n = 300;
    let objects = spaced(0..n, |i| format!("n{i}"));
    let links = spaced(1..n, |i| format!("(conn n{} n{i})", i - 1));
    let file = ground_str(
        "(define (domain d) (:predicates (at ?x) (conn ?x ?y))
           (:action mv :parameters (?a ?b) :precondition (and (at ?a) (conn ?a ?b))
             :effect (and (at ?b) (not (at ?a)))))",
        &format!(
            "(define (problem p) (:domain d) (:objects {objects}) (:init (at n0) {links})
               (:goal (at n{})))",
            n - 1
        ),
    )
    .unwrap();
    let names: Vec<_> = file.operators.iter().map(|op| op.name.clone()).collect();
    let expected: Vec<_> = (1..n).map(|i| format!("mv n{} n{i}", i - 1)).collect();
    assert_eq!(names, expected);
    assert_eq!(file.variables.len(), n);
}

#[test]
fn processes_and_events_are_unsupported() {
    let plain = problem_with("", "(p a)", "");
    for section in [
        "(:process grow :parameters () :effect (increase (f) (* #t 1)))",
        "(:event flip :parameters () :precondition (q) :effect (p a))",
    ] {
        assert_unsupported(&domain_with(section), &plain, "processes and events");
    }
}
