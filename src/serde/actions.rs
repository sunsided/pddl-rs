//! Actions, derived predicates and effects.

use crate::serde::expressions::operation;
use crate::serde::{
    as_map, atom, conjunction, deserialize_from_pddl, formula, serialize_deref, single_entry,
    string, typed_list, Keyed, Value,
};
use crate::{
    ActionDefinition, ActionSymbol, ConditionalEffect, DerivedPredicate, EffectCondition, Effects,
    ForallConditionalEffect, PrimitiveEffect, WhenConditionalEffect,
};
use ::serde::ser::SerializeMap;
use ::serde::{Serialize, Serializer};

serialize_deref!(ActionSymbol);

impl Serialize for PrimitiveEffect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PrimitiveEffect::AtomicFormula(formula) => formula.serialize(serializer),
            PrimitiveEffect::NotAtomicFormula(formula) => {
                Keyed("not", formula).serialize(serializer)
            }
            PrimitiveEffect::AssignNumericFluent(op, head, exp) => {
                operation(serializer, op, &(head, exp))
            }
            PrimitiveEffect::AssignObjectFluent(function, Some(term)) => {
                Keyed("assign", &(function, term)).serialize(serializer)
            }
            PrimitiveEffect::AssignObjectFluent(function, None) => {
                Keyed("assign", &(function, "undefined")).serialize(serializer)
            }
        }
    }
}

impl Serialize for ConditionalEffect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ConditionalEffect::Effect(effect) => effect.serialize(serializer),
            ConditionalEffect::Forall(effect) => effect.serialize(serializer),
            ConditionalEffect::When(effect) => effect.serialize(serializer),
        }
    }
}

impl Serialize for ForallConditionalEffect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Keyed("forall", &(&self.variables, &self.effects)).serialize(serializer)
    }
}

impl Serialize for WhenConditionalEffect {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Keyed("when", &(&self.condition, &self.effect)).serialize(serializer)
    }
}

impl Serialize for EffectCondition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            EffectCondition::Single(effect) => effect.serialize(serializer),
            EffectCondition::All(effects) => Keyed("and", effects).serialize(serializer),
        }
    }
}

impl Serialize for Effects {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        conjunction(serializer, self)
    }
}

impl Serialize for ActionDefinition {
    /// `{"name": {"parameters": ..., "precondition": ..., "effect": ...}}`, an entry
    /// of a domain's `actions`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Keyed(self.symbol(), &ActionBody(self)).serialize(serializer)
    }
}

/// An action without its name, as the value in a domain's `actions`.
pub(crate) struct ActionBody<'a>(pub &'a ActionDefinition);

impl Serialize for ActionBody<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let action = self.0;
        let mut map = serializer.serialize_map(None)?;
        if !action.parameters().is_empty() {
            map.serialize_entry("parameters", action.parameters())?;
        }
        if !action.precondition().is_empty() {
            map.serialize_entry("precondition", action.precondition())?;
        }
        if let Some(effect) = action.effect() {
            map.serialize_entry("effect", effect)?;
        }
        map.end()
    }
}

/// Writes `(:action name ...)` from an action's name and body.
pub(crate) fn action(out: &mut String, name: &str, body: &Value) -> Result<(), String> {
    let (mut parameters, mut precondition, mut effect) = (None, None, None);
    for (key, value) in as_map(body)? {
        let slot = match key.as_str() {
            "parameters" => &mut parameters,
            "precondition" => &mut precondition,
            "effect" => &mut effect,
            _ => return Err(format!("unknown key `{key}` in action `{name}`")),
        };
        *slot = Some(value);
    }
    out.push_str("(:action ");
    atom(out, name)?;
    out.push_str(" :parameters (");
    if let Some(parameters) = parameters {
        typed_list(out, as_map(parameters)?)?;
    }
    out.push(')');
    for (keyword, value) in [(" :precondition ", precondition), (" :effect ", effect)] {
        if let Some(value) = value {
            out.push_str(keyword);
            formula(out, value)?;
        }
    }
    out.push(')');
    Ok(())
}

impl Serialize for DerivedPredicate {
    /// `{"predicate": "name", "parameters": ..., "condition": gd}`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let skeleton = self.predicate();
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("predicate", skeleton.predicate())?;
        if !skeleton.variables().is_empty() {
            map.serialize_entry("parameters", skeleton.variables())?;
        }
        map.serialize_entry("condition", self.expression())?;
        map.end()
    }
}

/// Writes `(:derived (predicate parameters...) condition)`.
pub(crate) fn derived(out: &mut String, value: &Value) -> Result<(), String> {
    let (mut predicate, mut parameters, mut condition) = (None, None, None);
    for (key, value) in as_map(value)? {
        match key.as_str() {
            "predicate" => predicate = Some(value),
            "parameters" => parameters = Some(value),
            "condition" => condition = Some(value),
            _ => return Err(format!("unknown key `{key}` in a derived predicate")),
        }
    }
    out.push_str("(:derived (");
    string(
        out,
        predicate.ok_or("a derived predicate needs a `predicate`")?,
    )?;
    if let Some(parameters) = parameters {
        let parameters = as_map(parameters)?;
        if !parameters.is_empty() {
            out.push(' ');
            typed_list(out, parameters)?;
        }
    }
    out.push_str(") ");
    formula(
        out,
        condition.ok_or("a derived predicate needs a `condition`")?,
    )?;
    out.push(')');
    Ok(())
}

deserialize_from_pddl!(
    ActionSymbol => formula,
    PrimitiveEffect => formula,
    ConditionalEffect => formula,
    ForallConditionalEffect => formula,
    WhenConditionalEffect => formula,
    EffectCondition => formula,
    Effects => formula,
    ActionDefinition => |out, value| {
        let (name, body) = single_entry(value)?;
        action(out, name, body)
    },
    DerivedPredicate => derived,
);
