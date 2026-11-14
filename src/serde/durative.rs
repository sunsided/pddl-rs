//! Durative actions, timed goals and effects, and duration constraints.

use crate::parsers::Parser;
use crate::serde::expressions::{operands, operation};
use crate::serde::{
    as_map, atom, deserialize_from_pddl, formula, preference, serialize_deref, serialize_display,
    single_entry, typed_list, Keyed, Value,
};
use crate::{
    DurationConstraint, DurationOperator, DurationValue, DurativeActionDefinition,
    DurativeActionEffect, DurativeActionFluentExpression, DurativeActionFunctionAssignment,
    DurativeActionGoalDescription, DurativeActionSymbol, Interval, PreferenceTimedGoalDescription,
    SimpleDurationConstraint, TimeSpecifier, TimedAssignOperator, TimedEffect,
    TimedFluentExpression, TimedGoalDescription,
};
use ::serde::de::Error as _;
use ::serde::ser::SerializeMap;
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

serialize_deref!(DurativeActionSymbol);
serialize_display!(
    TimeSpecifier,
    Interval,
    DurationOperator,
    TimedAssignOperator
);

/// Serializes `{"at start": value}` or `{"at end": value}`.
fn at<S: Serializer, T: Serialize>(
    serializer: S,
    time: TimeSpecifier,
    value: &T,
) -> Result<S::Ok, S::Error> {
    Keyed(&format!("at {time}"), value).serialize(serializer)
}

impl Serialize for TimedGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TimedGoalDescription::At(time, gd) => at(serializer, *time, gd),
            TimedGoalDescription::Over(interval, gd) => {
                Keyed(&format!("over {interval}"), gd).serialize(serializer)
            }
        }
    }
}

impl Serialize for PreferenceTimedGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PreferenceTimedGoalDescription::Required(gd) => gd.serialize(serializer),
            PreferenceTimedGoalDescription::Preference(name, gd) => {
                preference(serializer, name.as_ref(), gd)
            }
        }
    }
}

impl Serialize for DurativeActionGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DurativeActionGoalDescription::Timed(gd) => gd.serialize(serializer),
            DurativeActionGoalDescription::And(gds) => Keyed("and", gds).serialize(serializer),
            DurativeActionGoalDescription::Forall(variables, gd) => {
                Keyed("forall", &(variables, gd)).serialize(serializer)
            }
        }
    }
}

impl Serialize for DurationValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DurationValue::Number(number) => number.serialize(serializer),
            DurationValue::FluentExpression(exp) => exp.serialize(serializer),
        }
    }
}

impl Serialize for SimpleDurationConstraint {
    /// `{"<=": ["?duration", value]}`, or the same inside `{"at end": ...}`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SimpleDurationConstraint::Op(op, value) => {
                operation(serializer, op, &("?duration", value))
            }
            SimpleDurationConstraint::At(time, constraint) => at(serializer, *time, constraint),
        }
    }
}

impl Serialize for DurationConstraint {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DurationConstraint::Single(constraint) => constraint.serialize(serializer),
            DurationConstraint::All(constraints) => Keyed("and", constraints).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DurationConstraint {
    /// Parses like the other types, but `()`, which means no constraint, is refused.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let mut text = String::new();
        formula(&mut text, &value).map_err(D::Error::custom)?;
        DurationConstraint::from_str(&text)
            .map_err(|e| D::Error::custom(format!("{e} in `{text}`")))?
            .ok_or_else(|| D::Error::custom("expected a duration constraint"))
    }
}

impl Serialize for TimedFluentExpression {
    /// `"#t"`, or `{"*": ["#t", exp]}` for a scaled rate.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TimedFluentExpression::Now => serializer.serialize_str("#t"),
            TimedFluentExpression::Scaled(exp) => Keyed("*", &("#t", exp)).serialize(serializer),
        }
    }
}

impl Serialize for DurativeActionFluentExpression {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        type E = DurativeActionFluentExpression;
        match self {
            E::Assign(op, head, exp) => operation(serializer, op, &(head, exp)),
            E::BinaryOp(op, a, b) => operation(serializer, op, &(a, b)),
            E::MultiOp(op, first, rest) => operation(serializer, op, &operands(&**first, rest)),
            E::Negative(value) => Keyed("-", value).serialize(serializer),
            E::Duration => serializer.serialize_str("?duration"),
            E::FluentExpression(exp) => exp.serialize(serializer),
        }
    }
}

impl Serialize for DurativeActionFunctionAssignment {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        operation(
            serializer,
            self.operation(),
            &(self.function(), self.function_expr()),
        )
    }
}

impl Serialize for TimedEffect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TimedEffect::Conditional(time, effect) => at(serializer, *time, effect),
            TimedEffect::NumericFluent(time, assignment) => at(serializer, *time, assignment),
            TimedEffect::ContinuousEffect(op, head, exp) => operation(serializer, op, &(head, exp)),
        }
    }
}

impl Serialize for DurativeActionEffect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DurativeActionEffect::Timed(effect) => effect.serialize(serializer),
            DurativeActionEffect::All(effects) => Keyed("and", effects).serialize(serializer),
            DurativeActionEffect::Forall(variables, effect) => {
                Keyed("forall", &(variables, effect)).serialize(serializer)
            }
            DurativeActionEffect::When(condition, effect) => {
                Keyed("when", &(condition, effect)).serialize(serializer)
            }
        }
    }
}

impl Serialize for DurativeActionDefinition {
    /// `{"name": {"parameters": ..., "duration": ..., "condition": ..., "effect": ...}}`,
    /// an entry of a domain's `durative-actions`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Keyed(self.symbol(), &DurativeActionBody(self)).serialize(serializer)
    }
}

/// A durative action without its name, as the value in a domain's `durative-actions`.
pub(crate) struct DurativeActionBody<'a>(pub &'a DurativeActionDefinition);

impl Serialize for DurativeActionBody<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let action = self.0;
        let mut map = serializer.serialize_map(None)?;
        if !action.parameters().is_empty() {
            map.serialize_entry("parameters", action.parameters())?;
        }
        if let Some(duration) = action.duration() {
            map.serialize_entry("duration", duration)?;
        }
        if let Some(condition) = action.condition() {
            map.serialize_entry("condition", condition)?;
        }
        if let Some(effect) = action.effect() {
            map.serialize_entry("effect", effect)?;
        }
        map.end()
    }
}

/// Writes `(:durative-action name ...)` from a durative action's name and body.
/// Missing parts are written as `()`, which PDDL uses for "none".
pub(crate) fn durative_action(out: &mut String, name: &str, body: &Value) -> Result<(), String> {
    let (mut parameters, mut duration, mut condition, mut effect) = (None, None, None, None);
    for (key, value) in as_map(body)? {
        let slot = match key.as_str() {
            "parameters" => &mut parameters,
            "duration" => &mut duration,
            "condition" => &mut condition,
            "effect" => &mut effect,
            _ => return Err(format!("unknown key `{key}` in durative action `{name}`")),
        };
        *slot = Some(value);
    }
    out.push_str("(:durative-action ");
    atom(out, name)?;
    out.push_str(" :parameters (");
    if let Some(parameters) = parameters {
        typed_list(out, as_map(parameters)?)?;
    }
    out.push(')');
    for (keyword, value) in [
        (" :duration ", duration),
        (" :condition ", condition),
        (" :effect ", effect),
    ] {
        out.push_str(keyword);
        match value {
            Some(value) => formula(out, value)?,
            None => out.push_str("()"),
        }
    }
    out.push(')');
    Ok(())
}

deserialize_from_pddl!(
    DurativeActionSymbol => formula,
    TimeSpecifier => formula,
    Interval => formula,
    DurationOperator => formula,
    TimedAssignOperator => formula,
    TimedGoalDescription => formula,
    PreferenceTimedGoalDescription => formula,
    DurativeActionGoalDescription => formula,
    DurationValue => formula,
    SimpleDurationConstraint => formula,
    TimedFluentExpression => formula,
    DurativeActionFluentExpression => formula,
    DurativeActionFunctionAssignment => formula,
    TimedEffect => formula,
    DurativeActionEffect => formula,
    DurativeActionDefinition => |out, value| {
        let (name, body) = single_entry(value)?;
        durative_action(out, name, body)
    },
);
