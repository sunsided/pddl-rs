#![cfg(feature = "pretty")]

use pddl::{Name, Pretty, Type, Variable};

#[test]
fn name_is_pretty() {
    assert_eq!(Name::new("x").pretty(10).to_string(), "x");
}

#[test]
fn variable_is_pretty() {
    assert_eq!(Variable::from("x").pretty(10).to_string(), "?x");
}

#[test]
fn either_wraps_according_to_width() {
    let t = Type::from_iter(["a", "b"]);
    assert_eq!(t.pretty(12).to_string(), "(either a b)");
    assert_eq!(t.pretty(10).to_string(), "(either a\n    b)");
    assert_eq!(t.pretty(8).to_string(), "(either\n    a b)");
}
