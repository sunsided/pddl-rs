//! PDDL survives a round trip through JSON, YAML and TOML with the `serde` feature.

use assert2::check;

use pddl::{Domain, FluentExpression, Name, Number, Parser, PddlFile, Problem};
use serde_json::json;
use std::fs;
use std::path::Path;

/// Every corpus file the parser accepts serializes to JSON, YAML and TOML and
/// deserializes back to a value that serializes to the same JSON. Problems also
/// compare equal; domains may come back with their actions, durative actions and
/// derived predicates regrouped, which the schema doesn't distinguish.
#[test]
fn corpus_round_trips_through_json_yaml_and_toml() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let mut checked = 0;
    for folder in ["vhpop", "potoroo"] {
        for entry in fs::read_dir(root.join(folder)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension() != Some("pddl".as_ref()) {
                continue;
            }
            let Ok(file) = PddlFile::from_str(&fs::read_to_string(&path).unwrap()) else {
                continue;
            };
            let json = serde_json::to_string(&file).unwrap();
            let back: PddlFile = serde_json::from_str(&json)
                .unwrap_or_else(|e| panic!("{}: {e}\n{json}", path.display()));
            check!(
                serde_json::to_string(&back).unwrap() == json,
                "{}",
                path.display()
            );
            check!(back.problems == file.problems, "{}", path.display());

            let yaml = serde_yaml_ng::to_string(&file).unwrap();
            let back: PddlFile = serde_yaml_ng::from_str(&yaml)
                .unwrap_or_else(|e| panic!("{}: {e}\n{yaml}", path.display()));
            check!(
                serde_json::to_string(&back).unwrap() == json,
                "{}: YAML",
                path.display()
            );

            let toml = toml::to_string(&file).unwrap();
            let back: PddlFile =
                toml::from_str(&toml).unwrap_or_else(|e| panic!("{}: {e}\n{toml}", path.display()));
            check!(
                serde_json::to_string(&back).unwrap() == json,
                "{}: TOML",
                path.display()
            );
            checked += 1;
        }
    }
    check!(checked > 80);
}

#[test]
fn a_domain_reads_like_pddl() {
    let domain = Domain::from_str(
        "(define (domain logistics)
           (:requirements :strips :typing :action-costs)
           (:types truck location)
           (:predicates (at ?t - truck ?l - location))
           (:functions (distance ?a ?b - location) (total-cost) - number
                       (home ?t - truck) - location)
           (:action drive
             :parameters (?t - truck ?from ?to - location)
             :precondition (and (at ?t ?from) (not (= ?from ?to)))
             :effect (and (not (at ?t ?from)) (at ?t ?to)
                          (increase (total-cost) (distance ?from ?to)))))",
    )
    .unwrap();
    let expected = json!({
        "domain": "logistics",
        "requirements": ["strips", "typing", "action-costs"],
        "types": {"truck": "object", "location": "object"},
        "predicates": {"at": {"?t": "truck", "?l": "location"}},
        "functions": {
            "distance": {"?a": "location", "?b": "location"},
            "total-cost": {},
            "home": {"parameters": {"?t": "truck"}, "type": "location"}
        },
        "actions": {
            "drive": {
                "parameters": {"?t": "truck", "?from": "location", "?to": "location"},
                "precondition": {"and": [["at", "?t", "?from"], {"not": ["=", "?from", "?to"]}]},
                "effect": {"and": [
                    {"not": ["at", "?t", "?from"]},
                    ["at", "?t", "?to"],
                    {"increase": [["total-cost"], ["distance", "?from", "?to"]]}
                ]}
            }
        }
    });
    check!(serde_json::to_value(&domain).unwrap() == expected);
    check!(serde_json::from_value::<Domain>(expected).unwrap() == domain);
}

#[test]
fn a_problem_reads_like_pddl() {
    let problem = Problem::from_str(
        "(define (problem p1) (:domain logistics)
           (:objects t1 - truck a - location)
           (:init (at t1 a) (= (distance a depot) 5) (at 10 (open depot)))
           (:goal (at t1 depot))
           (:metric minimize (total-cost)))",
    )
    .unwrap();
    let expected = json!({
        "problem": "p1",
        "domain": "logistics",
        "objects": {"t1": "truck", "a": "location"},
        "init": [["at", "t1", "a"], {"=": [["distance", "a", "depot"], 5]}, {"at": [10, ["open", "depot"]]}],
        "goal": ["at", "t1", "depot"],
        "metric": {"minimize": ["total-cost"]}
    });
    check!(serde_json::to_value(&problem).unwrap() == expected);
    check!(serde_json::from_value::<Problem>(expected).unwrap() == problem);
}

#[test]
fn durative_actions_and_derived_predicates() {
    let domain = Domain::from_str(
        "(define (domain d)
           (:durative-action load
             :parameters (?t - truck)
             :duration (and (>= ?duration 1) (at end (<= ?duration (- (cap ?t) 2))))
             :condition (and (at start (at ?t depot)) (preference p (over all (free ?t))))
             :effect (and (at end (loaded ?t)) (increase (fuel ?t) (* #t 2))))
           (:derived (parked ?t - truck) (at ?t depot)))",
    )
    .unwrap();
    let expected = json!({
        "domain": "d",
        "durative-actions": {
            "load": {
                "parameters": {"?t": "truck"},
                "duration": {"and": [
                    {">=": ["?duration", 1]},
                    {"at end": {"<=": ["?duration", {"-": [["cap", "?t"], 2]}]}}
                ]},
                "condition": {"and": [
                    {"at start": ["at", "?t", "depot"]},
                    {"preference": {"over all": ["free", "?t"]}, "name": "p"}
                ]},
                "effect": {"and": [
                    {"at end": ["loaded", "?t"]},
                    {"increase": [["fuel", "?t"], {"*": ["#t", 2]}]}
                ]}
            }
        },
        "derived": [
            {"predicate": "parked", "parameters": {"?t": "truck"}, "condition": ["at", "?t", "depot"]}
        ]
    });
    check!(serde_json::to_value(&domain).unwrap() == expected);
    check!(serde_json::from_value::<Domain>(expected).unwrap() == domain);
}

#[test]
fn negation_and_subtraction_are_told_apart() {
    for pddl in [
        "(- (f ?x))",
        "(- (f ?x) 1)",
        "(- 1 (f ?x))",
        "(- 3)",
        "(+ 1 2 3)",
    ] {
        let exp = FluentExpression::from_str(pddl).unwrap();
        let json = serde_json::to_value(&exp).unwrap();
        check!(
            serde_json::from_value::<FluentExpression>(json).unwrap() == exp,
            "{pddl}"
        );
    }
}

#[test]
fn names_are_plain_strings() {
    check!(serde_json::to_string(&Name::new("At")).unwrap() == r#""At""#);
    check!(serde_json::from_str::<Name>(r#""At""#).unwrap() == Name::new("at"));
}

#[test]
fn numbers_are_numbers() {
    check!(serde_json::to_string(&Number::from(5)).unwrap() == "5");
    check!(serde_json::to_string(&Number::from(1.5)).unwrap() == "1.5");
    check!(serde_json::from_str::<Number>("1e999").is_err());
}

/// PDDL has no negative literals, so in a formula a negative number reads back
/// as a negation; on its own, a `Number` keeps its sign.
#[test]
fn negative_numbers_read_back() {
    check!(serde_json::from_str::<Number>("-5").unwrap() == Number::from(-5));

    let exp = FluentExpression::Number(Number::from(-5));
    let json = serde_json::to_value(&exp).unwrap();
    check!(json == json!(-5));
    let back: FluentExpression = serde_json::from_value(json).unwrap();
    check!(back == FluentExpression::negative(FluentExpression::Number(Number::from(5))));
}

#[test]
fn input_cannot_change_the_pddl_structure() {
    let injected = json!({"problem": "p) (:goal (x", "domain": "d", "init": [], "goal": ["g"]});
    let error = serde_json::from_value::<Problem>(injected).unwrap_err();
    check!(error.to_string().contains("is not a valid PDDL name"));

    // Names the reader would split into several atoms: `a?b` would become the
    // constant `a` and the variable `?b`.
    for name in ["a?b", "a:b", "-x", "a b"] {
        let goal = json!({"problem": "p", "domain": "d", "init": [], "goal": ["at", name]});
        let error = serde_json::from_value::<Problem>(goal).unwrap_err();
        check!(
            error.to_string().contains("is not a valid PDDL name"),
            "{name}"
        );
    }
}

/// Sections that PDDL can't write empty still deserialize from an empty value.
#[test]
fn empty_sections_deserialize() {
    use pddl::{PredicateDefinitions, Requirements, Timeless};
    check!(serde_json::from_str::<Requirements>("[]").unwrap() == Requirements::default());
    check!(
        serde_json::from_str::<PredicateDefinitions>("{}").unwrap()
            == PredicateDefinitions::default()
    );
    check!(serde_json::from_str::<Timeless>("[]").unwrap() == Timeless::default());
}

#[test]
fn mistakes_are_reported() {
    let unknown = json!({"domain": "d", "action": {}});
    let error = serde_json::from_value::<Domain>(unknown).unwrap_err();
    check!(error.to_string() == "unknown key `action` in a domain");

    // The grammar still applies: `not` takes one goal, not two.
    let invalid =
        json!({"problem": "p", "domain": "d", "init": [], "goal": {"not": [["a"], ["b"]]}});
    check!(serde_json::from_value::<Problem>(invalid).is_err());
}

/// Parses `pddl` as `T`, round-trips it through JSON and expects the same value.
fn round_trip<T>(pddl: &str)
where
    T: Parser<Item = T>
        + serde::Serialize
        + serde::de::DeserializeOwned
        + PartialEq
        + std::fmt::Debug,
{
    let value = T::from_str(pddl).unwrap_or_else(|e| panic!("{pddl}: {e}"));
    let json = serde_json::to_value(&value).unwrap();
    let back: T =
        serde_json::from_value(json.clone()).unwrap_or_else(|e| panic!("{pddl}: {e}\n{json}"));
    check!(back == value, "{pddl}\n{json}");
}

/// Names, terms, types and expressions.
#[test]
fn terms_and_expressions_round_trip() {
    use pddl::*;
    round_trip::<Name>("truck");
    round_trip::<Variable>("?t");
    round_trip::<Predicate>("at");
    round_trip::<FunctionSymbol>("fuel");
    round_trip::<PrimitiveType>("truck");
    round_trip::<PreferenceName>("p1");
    round_trip::<ActionSymbol>("drive");
    round_trip::<DurativeActionSymbol>("load");
    round_trip::<Term>("(f a ?x)");
    round_trip::<FunctionTerm>("(f a (g ?x))");
    round_trip::<BasicFunctionTerm>("(f a b)");
    round_trip::<Type>("(either truck plane)");
    round_trip::<FunctionType>("number");
    round_trip::<FunctionHead>("(f ?x)");
    round_trip::<FluentComparison>("(>= (f) 1)");
    round_trip::<FluentExpression>("(* (f ?x) 2 3)");
    round_trip::<MetricFluentExpression>(
        "(+ (* 2 total-time) (- (is-violated p)) (/ (f a) 3) (- 1 2) (g))",
    );
    round_trip::<AssignOp>("scale-up");
    round_trip::<BinaryComparison>("<=");
    round_trip::<BinaryOp>("/");
    round_trip::<MultiOp>("*");
    round_trip::<Optimization>("maximize");
}

/// Goals, preferences and constraints.
#[test]
fn goals_round_trip() {
    use pddl::*;
    round_trip::<GoalDescription>(
        "(and (p (f ?x)) (or (q) (not (r))) (imply (a) (b)) (exists (?y - t) (p ?y))
              (forall (?z - (either a b)) (p ?z)) (> (f ?x) (- (g) 1)) (= ?x ?y))",
    );
    round_trip::<PreferenceGoalDescription>("(preference p (q))");
    round_trip::<PreferenceGoalDescription>("(preference (q))");
    round_trip::<PreconditionGoalDescriptions>("(and (forall (?x) (preference p (q ?x))) (r))");
    round_trip::<ConstraintGoalDescription>(
        "(and (forall (?x) (always (p ?x))) (at end (q)) (sometime (q)) (within 5 (q))
              (at-most-once (q)) (sometime-after (p) (q)) (sometime-before (p) (q))
              (always-within 3 (p) (q)) (hold-during 1 2 (q)) (hold-after 4 (q))
              (always (sometime (q))))",
    );
    round_trip::<ConstraintGoalDescriptionInner>("(sometime (q))");
    round_trip::<PreferenceConstraintGoalDescriptions>(
        "(and (preference p1 (always (q))) (preference (sometime (q)))
              (forall (?x) (preference p2 (always (q ?x)))) (always (r)))",
    );
}

/// Effects, actions and durative actions.
#[test]
fn actions_and_durative_actions_round_trip() {
    use pddl::*;
    round_trip::<PrimitiveEffect>("(assign (f a) b)");
    round_trip::<PrimitiveEffect>("(assign (f a) undefined)");
    round_trip::<ConditionalEffect>("(when (p) (and (q) (not (r))))");
    round_trip::<ForallConditionalEffect>("(forall (?x) (p ?x))");
    round_trip::<WhenConditionalEffect>("(when (p) (q))");
    round_trip::<EffectCondition>("(and (q) (increase (f) 2))");
    round_trip::<Effects>("(and (forall (?x) (when (p ?x) (q ?x))) (p a))");
    round_trip::<TimeSpecifier>("start");
    round_trip::<Interval>("all");
    round_trip::<DurationOperator>(">=");
    round_trip::<TimedAssignOperator>("decrease");
    round_trip::<TimedGoalDescription>("(over all (p))");
    round_trip::<PreferenceTimedGoalDescription>("(preference p (at start (p)))");
    round_trip::<DurativeActionGoalDescription>("(forall (?y) (at start (p ?y)))");
    round_trip::<DurationValue>("(f a)");
    round_trip::<SimpleDurationConstraint>("(at end (<= ?duration 5))");
    round_trip::<TimedFluentExpression>("#t");
    round_trip::<TimedFluentExpression>("(* #t 2)");
    round_trip::<DurativeActionFluentExpression>("(+ ?duration 1 (- ?duration))");
    round_trip::<DurativeActionFunctionAssignment>("(increase (f a) (* ?duration 2))");
    round_trip::<TimedEffect>("(at end (increase (f a) ?duration))");
    round_trip::<TimedEffect>("(increase (f a) #t)");
    round_trip::<DurativeActionEffect>(
        "(and (forall (?y) (at end (q ?y))) (when (at start (p a)) (at end (q a))))",
    );
    round_trip::<DurativeActionDefinition>(
        "(:durative-action d :parameters (?x) :duration (= ?duration (- (f ?x) 1))
           :condition (at start (p ?x)) :effect (at end (q ?x)))",
    );
    round_trip::<DurativeActionDefinition>(
        "(:durative-action d :parameters () :duration () :condition () :effect ())",
    );
    round_trip::<ActionDefinition>(
        "(:action a :parameters (?x) :precondition (p ?x) :effect (q ?x))",
    );
    round_trip::<DerivedPredicate>("(:derived (p ?x - t) (q ?x))");
    round_trip::<DerivedPredicate>("(:derived (p) (q))");
    round_trip::<StructureDef>("(:action a :parameters ())");
    round_trip::<StructureDef>(
        "(:durative-action d :parameters () :duration () :condition () :effect ())",
    );
    round_trip::<StructureDef>("(:derived (p) (q))");
}

/// Sections, domains and problems.
#[test]
fn sections_round_trip() {
    use pddl::*;
    round_trip::<AtomicFormulaSkeleton>("(p ?x - t)");
    round_trip::<AtomicFunctionSkeleton>("(f ?x)");
    round_trip::<Requirement>(":typing");
    round_trip::<Requirements>("(:requirements :strips :foo-bar)");
    round_trip::<Types>("(:types a b - c)");
    round_trip::<Constants>("(:constants a - t)");
    round_trip::<Objects>("(:objects a)");
    round_trip::<PredicateDefinitions>("(:predicates (p ?x) (q))");
    round_trip::<Functions>("(:functions (f ?x) - number (g) - object)");
    round_trip::<DomainConstraintsDef>("(:constraints (always (p)))");
    round_trip::<Timeless>("(:timeless (p a) (not (q)))");
    round_trip::<InitElement>("(= (g a) b)");
    round_trip::<InitElements>("(:init (p a) (not (q a)) (at 3 (r)) (= (f a) 2) (= (g a) b))");
    round_trip::<ProblemGoalDescription>("(:goal (p))");
    round_trip::<ProblemConstraintsDef>("(:constraints (preference p (always (q))))");
    round_trip::<MetricSpec>("(:metric maximize (f))");
    round_trip::<LengthSpec>("(:length (:serial 3) (:parallel 2))");
    round_trip::<Domain>(
        "(define (domain d) (:extends base) (:requirements :strips)
           (:constraints (always (p))) (:timeless (p)))",
    );
    round_trip::<Problem>(
        "(define (problem p) (:domain d) (:init) (:goal (p))
           (:constraints (always (p))) (:length (:serial 2)))",
    );
    round_trip::<Definition>("(define (domain d))");
    round_trip::<Definition>("(define (problem p) (:domain d) (:init) (:goal (p)))");
}

#[test]
fn types_without_a_parser_still_serialize() {
    use pddl::{GoalDescription, Literal, Type, Typed, TypedList};
    let typed = Typed::new(Name::new("a"), Type::OBJECT);
    check!(serde_json::to_value(&typed).unwrap() == json!({"a": "object"}));
    check!(serde_json::to_value(TypedList::new(vec![typed])).unwrap() == json!({"a": "object"}));

    let atom = pddl::AtomicFormula::new_predicate("p".into(), [pddl::Term::Name("a".into())]);
    let literal = GoalDescription::Literal(Literal::new_not(atom));
    check!(serde_json::to_value(&literal).unwrap() == json!({"not": ["p", "a"]}));
}

#[test]
fn duration_constraints_round_trip() {
    use pddl::DurationConstraint;
    let constraint = DurationConstraint::from_str("(and (>= ?duration 1) (<= ?duration 5))")
        .unwrap()
        .unwrap();
    let json = serde_json::to_value(&constraint).unwrap();
    check!(serde_json::from_value::<DurationConstraint>(json).unwrap() == constraint);

    let error = serde_json::from_value::<DurationConstraint>(json!({"and": []})).unwrap_err();
    check!(error.to_string().contains("in `(and)`"));
}

/// Mistakes in the input are explained in terms of the schema.
#[test]
fn schema_mistakes_are_explained() {
    use pddl::{
        ActionDefinition, DerivedPredicate, DurativeActionDefinition, FluentExpression, Functions,
        GoalDescription, LengthSpec, StructureDef, Types,
    };
    fn message<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        json: serde_json::Value,
    ) -> String {
        serde_json::from_value::<T>(json).unwrap_err().to_string()
    }
    check!(
        message::<ActionDefinition>(json!({"a": {"pre": []}})) == "unknown key `pre` in action `a`"
    );
    check!(
        message::<DurativeActionDefinition>(json!({"d": {"cond": []}}))
            == "unknown key `cond` in durative action `d`"
    );
    check!(
        message::<DerivedPredicate>(json!({"pred": "p"}))
            == "unknown key `pred` in a derived predicate"
    );
    check!(
        message::<DerivedPredicate>(json!({"condition": ["q"]}))
            == "a derived predicate needs a `predicate`"
    );
    check!(
        message::<DerivedPredicate>(json!({"predicate": "p"}))
            == "a derived predicate needs a `condition`"
    );
    check!(
        message::<Functions>(json!({"f": {"kind": "x"}})) == "unknown key `kind` in function `f`"
    );
    check!(message::<LengthSpec>(json!({"steps": 3})) == "unknown key `steps` in `length`");
    check!(
        message::<StructureDef>(json!({"method": {}})) == "unknown structure definition `method`"
    );
    check!(message::<PddlFile>(json!({"files": []})) == "unknown key `files` in a PDDL file");
    check!(
        message::<Problem>(json!({"problem": "p", "domain": "d", "plan": []}))
            == "unknown key `plan` in a problem"
    );
    check!(message::<Domain>(json!({"name": "d"})) == "missing `domain`");
    check!(
        message::<GoalDescription>(json!({"and": [], "or": []}))
            == r#"expected an object with a single keyword, found keys ["and", "or"]"#
    );
    check!(message::<Types>(json!({"a": 5})) == "expected a type, found the number 5");
    check!(
        message::<DerivedPredicate>(json!({"predicate": ["p"], "condition": ["q"]}))
            == "expected a string, found an array"
    );
    check!(
        message::<Problem>(json!({"problem": "p", "domain": "d", "init": {"a": 1}, "goal": ["g"]}))
            == "expected an array, found an object"
    );
    check!(message::<ActionDefinition>(json!({})) == "expected an object with a single entry");
    check!(message::<ActionDefinition>(json!({"a": []})) == "expected an object, found an array");
    check!(message::<Domain>(json!({"domain": {"x": 1}})) == "expected a string, found an object");
    check!(message::<Domain>(json!({"domain": "a b"})) == "`a b` is not a valid PDDL name");
    check!(message::<Domain>(json!({"domain": 5})) == "expected a string, found the number 5");
    check!(message::<GoalDescription>(json!(true)).contains("a string, number, array or object"));

    // YAML can spell a non-finite number; PDDL can't.
    let error = serde_yaml_ng::from_str::<FluentExpression>(".nan").unwrap_err();
    check!(error.to_string().contains("NaN is not a finite number"));
}
