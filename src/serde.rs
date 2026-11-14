//! Serde support: PDDL domains and problems as JSON, YAML, TOML or any other
//! self-describing format.
//!
//! Every PDDL type implements [`Serialize`], and every type with
//! a [`Parser`] also implements [`Deserialize`].
//!
//! # Schema
//!
//! The schema follows PDDL, so a PDDL reader can read it without a reference:
//!
//! - Names are strings. Variables keep their `?`, which tells them apart from
//!   constants: `"truck"`, `"?t"`. Numbers are numbers. PDDL has no negative
//!   literals, so in a formula a negative number reads back as a negation.
//! - Atoms and function terms are arrays, the symbol first: `["at", "?t", "depot"]`,
//!   `["total-cost"]`, and `["=", "?a", "?b"]` for equality.
//! - Every other form `(op a b ...)` is an object with the keyword as its only key
//!   and the arguments in PDDL order: `{"imply": [a, b]}`, `{"forall": [vars, gd]}`,
//!   `{"increase": [["total-cost"], 1]}`, `{">=": [a, b]}`. Forms with one argument
//!   take it directly: `{"not": ["at", "?t"]}`, `{"at start": gd}`, `{"over all": gd}`.
//!   `{"-": x}` is negation and `{"-": [a, b]}` is subtraction.
//! - A conjunction of one is written as that member alone, as in PDDL.
//! - A preference is `{"preference": gd}`, with an extra `"name"` key when it
//!   is named: `{"preference": gd, "name": "p"}`.
//! - Typed lists are objects from name to type, in order: `{"?t": "truck"}`.
//!   `either` types are arrays: `{"?x": ["truck", "plane"]}`.
//!
//! A domain:
//!
//! ```json
//! {
//!   "domain": "logistics",
//!   "requirements": ["strips", "typing", "action-costs"],
//!   "types": {"truck": "object", "location": "object"},
//!   "predicates": {"at": {"?t": "truck", "?l": "location"}},
//!   "functions": {
//!     "distance": {"?a": "location", "?b": "location"},
//!     "total-cost": {},
//!     "home": {"parameters": {"?t": "truck"}, "type": "location"}
//!   },
//!   "actions": {
//!     "drive": {
//!       "parameters": {"?t": "truck", "?from": "location", "?to": "location"},
//!       "precondition": {"and": [["at", "?t", "?from"], {"not": ["=", "?from", "?to"]}]},
//!       "effect": {"and": [
//!         {"not": ["at", "?t", "?from"]},
//!         ["at", "?t", "?to"],
//!         {"increase": [["total-cost"], ["distance", "?from", "?to"]]}
//!       ]}
//!     }
//!   },
//!   "durative-actions": {
//!     "load": {
//!       "parameters": {"?t": "truck"},
//!       "duration": {"=": ["?duration", 2]},
//!       "condition": {"at start": ["at", "?t", "depot"]},
//!       "effect": {"at end": ["loaded", "?t"]}
//!     }
//!   },
//!   "derived": [
//!     {"predicate": "parked", "parameters": {"?t": "truck"}, "condition": ["at", "?t", "depot"]}
//!   ]
//! }
//! ```
//!
//! Functions of type `number` map to their parameters alone. Other functions spell
//! out `parameters` and `type`. Sections that are empty in PDDL are left out.
//!
//! A problem:
//!
//! ```json
//! {
//!   "problem": "p1",
//!   "domain": "logistics",
//!   "objects": {"t1": "truck", "a": "location"},
//!   "init": [["at", "t1", "a"], {"=": [["distance", "a", "depot"], 5]}, {"at": [10, ["open", "depot"]]}],
//!   "goal": ["at", "t1", "depot"],
//!   "metric": {"minimize": ["total-cost"]}
//! }
//! ```
//!
//! # Deserialization
//!
//! Deserializing rebuilds the PDDL text and runs the [`Parser`] on it, so the input is
//! checked against the grammar and yields exactly what parsing the PDDL would. Where
//! the Rust types can represent one PDDL form in several ways, the result is the
//! parser's choice, and actions, durative actions and derived predicates come back in
//! that order. Error messages quote the rebuilt PDDL.
//!
//! The format must be self-describing; formats like `bincode` aren't supported.
//!
//! # Key order
//!
//! Typed lists are objects, so the order of an action's parameters is the order of
//! the object's keys. Parsing from text keeps it, but some in-memory maps sort their
//! keys: enable `serde_json`'s and `toml`'s `preserve_order` features when going
//! through `serde_json::Value` or `toml::Table`, or when writing TOML.

mod actions;
mod durative;
mod expressions;
mod goals;
mod structure;
mod terms;

use crate::parsers::Parser;
use crate::PreferenceName;
use ::serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use ::serde::ser::{SerializeMap, SerializeSeq, Serializer};
use ::serde::{Deserialize, Serialize};
use std::fmt;

/// Serializes as `{key: value}`.
pub(crate) struct Keyed<'a, V: ?Sized>(pub &'a str, pub &'a V);

impl<V: Serialize + ?Sized> Serialize for Keyed<'_, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(self.0, self.1)?;
        map.end()
    }
}

/// Serializes as `[head, items...]`, the shape of atoms and function terms.
pub(crate) struct Head<'a, H: ?Sized, T>(pub &'a H, pub &'a [T]);

impl<H: Serialize + ?Sized, T: Serialize> Serialize for Head<'_, H, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(1 + self.1.len()))?;
        seq.serialize_element(self.0)?;
        for item in self.1 {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

/// Serializes `items` as `{"and": items}`, or a single item alone.
pub(crate) fn conjunction<S: Serializer, T: Serialize>(
    serializer: S,
    items: &[T],
) -> Result<S::Ok, S::Error> {
    match items {
        [one] => one.serialize(serializer),
        _ => Keyed("and", items).serialize(serializer),
    }
}

/// Serializes `{"preference": goal}`, with a `"name"` key if the preference is named.
pub(crate) fn preference<S: Serializer, G: Serialize + ?Sized>(
    serializer: S,
    name: Option<&PreferenceName>,
    goal: &G,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(1 + usize::from(name.is_some())))?;
    map.serialize_entry("preference", goal)?;
    if let Some(name) = name {
        map.serialize_entry("name", name)?;
    }
    map.end()
}

/// Deserializes `T` by rebuilding its PDDL text with `render` and parsing it.
pub(crate) fn from_pddl<'de, D, T>(deserializer: D, render: Render) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Parser<Item = T>,
{
    parse_value(&Value::deserialize(deserializer)?, render)
}

/// Like [`from_pddl`], but an empty array or object is the default value, for
/// sections that PDDL can't write empty, such as `(:predicates)`.
pub(crate) fn from_pddl_or_default<'de, D, T>(
    deserializer: D,
    render: Render,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Parser<Item = T> + Default,
{
    let value = Value::deserialize(deserializer)?;
    match &value {
        Value::Seq(items) if items.is_empty() => Ok(T::default()),
        Value::Map(entries) if entries.is_empty() => Ok(T::default()),
        _ => parse_value(&value, render),
    }
}

/// Rebuilds the PDDL text of `value` with `render` and parses it as `T`.
fn parse_value<T: Parser<Item = T>, E: de::Error>(value: &Value, render: Render) -> Result<T, E> {
    let mut text = String::new();
    render(&mut text, value).map_err(E::custom)?;
    T::from_str(&text).map_err(|e| E::custom(format!("{e} in `{text}`")))
}

/// Implements `Deserialize` through [`from_pddl_or_default`] with the given renderer.
macro_rules! deserialize_from_pddl_or_default {
    ($($ty:ty => $render:expr),+ $(,)?) => {$(
        impl<'de> ::serde::Deserialize<'de> for $ty {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $crate::serde::from_pddl_or_default(deserializer, $render)
            }
        }
    )+};
}
pub(crate) use deserialize_from_pddl_or_default;

/// Implements `Deserialize` through [`from_pddl`] with the given renderer.
macro_rules! deserialize_from_pddl {
    ($($ty:ty => $render:expr),+ $(,)?) => {$(
        impl<'de> ::serde::Deserialize<'de> for $ty {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $crate::serde::from_pddl(deserializer, $render)
            }
        }
    )+};
}
pub(crate) use deserialize_from_pddl;

/// Implements `Serialize` as the string the type derefs to, for names.
macro_rules! serialize_deref {
    ($($ty:ty),+ $(,)?) => {$(
        impl ::serde::Serialize for $ty {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                (**self).serialize(serializer)
            }
        }
    )+};
}
pub(crate) use serialize_deref;

/// Implements `Serialize` as the type's `Display` output, for keywords.
macro_rules! serialize_display {
    ($($ty:ty),+ $(,)?) => {$(
        impl ::serde::Serialize for $ty {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }
    )+};
}
pub(crate) use serialize_display;

/// A deserialized value in any self-describing format, before it is turned into PDDL.
#[derive(Debug)]
pub(crate) enum Value {
    /// A string: a name, variable or keyword.
    Str(String),
    /// A number, as text, so that it reaches the parser unchanged.
    Num(String),
    /// An array.
    Seq(Vec<Value>),
    /// An object, its entries in order.
    Map(Vec<(String, Value)>),
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueVisitor)
    }
}

/// Builds a [`Value`] from whatever the format holds.
struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a string, number, array or object")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::Str(v.to_owned()))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(Value::Num(v.to_string()))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(Value::Num(v.to_string()))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
        if v.is_finite() {
            Ok(Value::Num(v.to_string()))
        } else {
            Err(E::custom(format!("{v} is not a finite number")))
        }
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Value, A::Error> {
        let mut items = Vec::new();
        while let Some(item) = seq.next_element()? {
            items.push(item);
        }
        Ok(Value::Seq(items))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut entries = Vec::new();
        while let Some(entry) = map.next_entry()? {
            entries.push(entry);
        }
        Ok(Value::Map(entries))
    }
}

/// Writes the PDDL text of a [`Value`], or explains why it has none.
pub(crate) type Render = fn(&mut String, &Value) -> Result<(), String>;

/// The keys written as two words in PDDL.
const TWO_WORD_KEYS: &[&str] = &["at start", "at end", "over all"];

/// The keywords that take a single argument, even when it is an array such as an atom.
const UNARY_KEYS: &[&str] = &[
    "not",
    "at start",
    "at end",
    "over all",
    "always",
    "sometime",
    "at-most-once",
    "preference",
    "is-violated",
    "minimize",
    "maximize",
];

/// Writes a name, keyword or variable, refusing anything that the reader
/// wouldn't take as exactly this one atom: a parenthesis or space would change
/// the structure of the text, and an inner `?` or `:` or a leading operator
/// would split it, e.g. `a?b` into the constant `a` and the variable `?b`.
pub(crate) fn atom(out: &mut String, s: &str) -> Result<(), String> {
    let single_atom = crate::parsers::sexpr::read(s).is_ok_and(|forest| {
        forest.unbalanced.is_none()
            && matches!(forest.nodes.as_slice(), [node] if node.atom() == Some(s))
    });
    if !single_atom {
        return Err(format!("`{s}` is not a valid PDDL name"));
    }
    out.push_str(s);
    Ok(())
}

/// Writes a formula or term: strings and numbers as atoms, arrays as `( ... )`,
/// objects as keyword forms or, if their keys are variables, as typed lists.
pub(crate) fn formula(out: &mut String, value: &Value) -> Result<(), String> {
    match value {
        Value::Str(s) => atom(out, s),
        // PDDL has no negative literals: `-5` is the negation `(- 5)`.
        Value::Num(n) => {
            match n.strip_prefix('-') {
                Some(magnitude) => {
                    out.push_str("(- ");
                    out.push_str(magnitude);
                    out.push(')');
                }
                None => out.push_str(n),
            }
            Ok(())
        }
        Value::Seq(items) => {
            out.push('(');
            spaced(out, items, formula)?;
            out.push(')');
            Ok(())
        }
        Value::Map(entries) if entries.iter().all(|(key, _)| key.starts_with('?')) => {
            out.push('(');
            typed_list(out, entries)?;
            out.push(')');
            Ok(())
        }
        Value::Map(entries) => keyed(out, entries),
    }
}

/// Writes `items` separated by spaces.
pub(crate) fn spaced(out: &mut String, items: &[Value], render: Render) -> Result<(), String> {
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        render(out, item)?;
    }
    Ok(())
}

/// Writes a keyword form `{"op": args}` as `(op args...)`.
fn keyed(out: &mut String, entries: &[(String, Value)]) -> Result<(), String> {
    let (key, value, name) = match entries {
        [(key, value)] => (key.as_str(), value, None),
        [(a, goal), (b, name)] | [(b, name), (a, goal)] if a == "preference" && b == "name" => {
            ("preference", goal, Some(name))
        }
        _ => {
            let keys: Vec<_> = entries.iter().map(|(key, _)| key.as_str()).collect();
            return Err(format!(
                "expected an object with a single keyword, found keys {keys:?}"
            ));
        }
    };
    out.push('(');
    if TWO_WORD_KEYS.contains(&key) {
        out.push_str(key);
    } else {
        atom(out, key)?;
    }
    if let Some(name) = name {
        out.push(' ');
        string(out, name)?;
    }
    match value {
        Value::Seq(items) if !takes_one(key, items) => {
            for item in items {
                out.push(' ');
                formula(out, item)?;
            }
        }
        _ => {
            out.push(' ');
            formula(out, value)?;
        }
    }
    out.push(')');
    Ok(())
}

/// Whether the keyword's array value is its single argument rather than its
/// argument list: always for unary keywords, and for `-` when the array is a
/// function term, i.e. `{"-": ["f", "?x"]}` negates `(f ?x)`.
fn takes_one(key: &str, items: &[Value]) -> bool {
    UNARY_KEYS.contains(&key)
        || (key == "-"
            && matches!(items.first(), Some(Value::Str(s)) if !s.starts_with(['?', '#'])))
}

/// Writes the entries of a typed list, `name - type`, separated by spaces.
pub(crate) fn typed_list(out: &mut String, entries: &[(String, Value)]) -> Result<(), String> {
    for (i, (name, r#type)) in entries.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        atom(out, name)?;
        out.push_str(" - ");
        type_(out, r#type)?;
    }
    Ok(())
}

/// Writes a type: a name, or `(either ...)` for an array of names.
pub(crate) fn type_(out: &mut String, value: &Value) -> Result<(), String> {
    match value {
        Value::Str(s) => atom(out, s),
        Value::Seq(items) => {
            out.push_str("(either");
            for item in items {
                out.push(' ');
                string(out, item)?;
            }
            out.push(')');
            Ok(())
        }
        _ => Err(format!("expected a type, found {}", describe(value))),
    }
}

/// Writes a value that must be a string.
pub(crate) fn string(out: &mut String, value: &Value) -> Result<(), String> {
    atom(out, as_str(value)?)
}

/// The string in `value`.
pub(crate) fn as_str(value: &Value) -> Result<&str, String> {
    match value {
        Value::Str(s) => Ok(s),
        _ => Err(format!("expected a string, found {}", describe(value))),
    }
}

/// The items of the array in `value`.
pub(crate) fn as_seq(value: &Value) -> Result<&[Value], String> {
    match value {
        Value::Seq(items) => Ok(items),
        _ => Err(format!("expected an array, found {}", describe(value))),
    }
}

/// The key and value of an object with exactly one entry.
pub(crate) fn single_entry(value: &Value) -> Result<(&str, &Value), String> {
    match as_map(value)? {
        [(key, value)] => Ok((key, value)),
        _ => Err("expected an object with a single entry".to_owned()),
    }
}

/// The entries of the object in `value`.
pub(crate) fn as_map(value: &Value) -> Result<&[(String, Value)], String> {
    match value {
        Value::Map(entries) => Ok(entries),
        _ => Err(format!("expected an object, found {}", describe(value))),
    }
}

/// A short description of `value` for error messages.
fn describe(value: &Value) -> String {
    match value {
        Value::Str(s) => format!("the string `{s}`"),
        Value::Num(n) => format!("the number {n}"),
        Value::Seq(_) => "an array".to_owned(),
        Value::Map(_) => "an object".to_owned(),
    }
}
