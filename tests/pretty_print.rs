//! Pretty-printing of individual PDDL elements.

#![cfg(feature = "pretty")]

use assert2::check;

use pddl::{Name, Pretty, Type, Variable};

#[test]
fn name_is_pretty() {
    check!(Name::new("x").pretty(10).to_string() == "x");
}

#[test]
fn variable_is_pretty() {
    check!(Variable::from("x").pretty(10).to_string() == "?x");
}

#[test]
fn either_wraps_according_to_width() {
    let t = Type::from_iter(["a", "b"]);
    check!(t.pretty(12).to_string() == "(either a b)");
    check!(t.pretty(10).to_string() == "(either a\n    b)");
    check!(t.pretty(8).to_string() == "(either\n    a b)");
}
