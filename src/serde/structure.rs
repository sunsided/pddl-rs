//! Domains, problems and their sections.

use crate::serde::actions::{action, derived, ActionBody};
use crate::serde::durative::{durative_action, DurativeActionBody};
use crate::serde::{
    as_map, as_seq, as_str, atom, deserialize_from_pddl, deserialize_from_pddl_or_default, formula,
    single_entry, spaced, string, type_, typed_list, Keyed, Value,
};
use crate::{
    AtomicFormulaSkeleton, AtomicFunctionSkeleton, Constants, Definition, Domain,
    DomainConstraintsDef, FunctionType, FunctionTyped, FunctionTypedList, Functions, InitElement,
    InitElements, LengthSpec, MetricSpec, Objects, PddlFile, PredicateDefinitions, Problem,
    ProblemConstraintsDef, ProblemGoalDescription, Requirement, Requirements, StructureDef,
    StructureDefs, Timeless, Types,
};
use ::serde::ser::{SerializeMap, SerializeSeq};
use ::serde::{Serialize, Serializer};

impl Serialize for Requirement {
    /// The requirement's name without its leading colon, e.g. `"strips"`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str().trim_start_matches(':'))
    }
}

impl Serialize for Requirements {
    /// The known requirements, then the unknown ones, without colons.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len() + self.unknown().len()))?;
        for requirement in self.iter() {
            seq.serialize_element(requirement)?;
        }
        for name in self.unknown() {
            seq.serialize_element(name.trim_start_matches(':'))?;
        }
        seq.end()
    }
}

/// Writes a requirement name with its colon.
fn requirement(out: &mut String, value: &Value) -> Result<(), String> {
    out.push(':');
    string(out, value)
}

/// Writes `(:requirements :a :b ...)`.
fn requirements(out: &mut String, value: &Value) -> Result<(), String> {
    out.push_str("(:requirements ");
    spaced(out, as_seq(value)?, requirement)?;
    out.push(')');
    Ok(())
}

impl Serialize for Types {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl Serialize for Constants {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl Serialize for Objects {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

/// Writes `(:keyword typed list)` from an object of names to types.
fn typed_section(out: &mut String, keyword: &str, value: &Value) -> Result<(), String> {
    out.push_str("(:");
    out.push_str(keyword);
    out.push(' ');
    typed_list(out, as_map(value)?)?;
    out.push(')');
    Ok(())
}

impl Serialize for AtomicFormulaSkeleton {
    /// `{"predicate": {parameters}}`, an entry of a domain's `predicates`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Keyed(self.predicate(), self.variables()).serialize(serializer)
    }
}

impl Serialize for PredicateDefinitions {
    /// An object from each predicate to its parameters.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for skeleton in self.iter() {
            map.serialize_entry(skeleton.predicate(), skeleton.variables())?;
        }
        map.end()
    }
}

/// Writes `(name ?x - type ...)` from a name and its parameters.
fn skeleton(out: &mut String, name: &str, parameters: &Value) -> Result<(), String> {
    out.push('(');
    atom(out, name)?;
    let parameters = as_map(parameters)?;
    if !parameters.is_empty() {
        out.push(' ');
        typed_list(out, parameters)?;
    }
    out.push(')');
    Ok(())
}

/// Writes `(:predicates (p ...) ...)`.
fn predicates(out: &mut String, value: &Value) -> Result<(), String> {
    out.push_str("(:predicates");
    for (name, parameters) in as_map(value)? {
        out.push(' ');
        skeleton(out, name, parameters)?;
    }
    out.push(')');
    Ok(())
}

impl Serialize for AtomicFunctionSkeleton {
    /// `{"function": {parameters}}`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Keyed(self.symbol(), self.variables()).serialize(serializer)
    }
}

/// A function's value in a domain's `functions`: its parameters if it is a
/// `number`, otherwise `{"parameters": ..., "type": ...}`.
struct FunctionEntry<'a>(&'a FunctionTyped<AtomicFunctionSkeleton>);

impl Serialize for FunctionEntry<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let variables = self.0.value_ref().variables();
        let r#type = self.0.type_ref();
        if *r#type == FunctionType::NUMBER {
            return variables.serialize(serializer);
        }
        let mut map = serializer.serialize_map(None)?;
        if !variables.is_empty() {
            map.serialize_entry("parameters", variables)?;
        }
        map.serialize_entry("type", r#type)?;
        map.end()
    }
}

impl Serialize for FunctionTypedList<AtomicFunctionSkeleton> {
    /// An object from each function to its parameters, or to `{"parameters": ..., "type": ...}`
    /// when its type isn't `number`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for function in self.iter() {
            map.serialize_entry(function.value_ref().symbol(), &FunctionEntry(function))?;
        }
        map.end()
    }
}

impl Serialize for Functions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

/// Writes `(:functions (f ...) - type ...)`, typing every function explicitly so
/// that no function picks up the type of the one after it.
fn functions(out: &mut String, value: &Value) -> Result<(), String> {
    let no_parameters = Value::Map(Vec::new());
    out.push_str("(:functions");
    for (name, entry) in as_map(value)? {
        let entries = as_map(entry)?;
        let (parameters, r#type) = if entries.iter().all(|(key, _)| key.starts_with('?')) {
            (entry, None)
        } else {
            let (mut parameters, mut r#type) = (None, None);
            for (key, value) in entries {
                match key.as_str() {
                    "parameters" => parameters = Some(value),
                    "type" => r#type = Some(value),
                    _ => return Err(format!("unknown key `{key}` in function `{name}`")),
                }
            }
            (parameters.unwrap_or(&no_parameters), r#type)
        };
        out.push(' ');
        skeleton(out, name, parameters)?;
        out.push_str(" - ");
        match r#type {
            Some(r#type) => type_(out, r#type)?,
            None => out.push_str("number"),
        }
    }
    out.push(')');
    Ok(())
}

impl Serialize for DomainConstraintsDef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl Serialize for Timeless {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

/// Writes `(:keyword items...)` from an array.
fn list_section(out: &mut String, keyword: &str, value: &Value) -> Result<(), String> {
    out.push_str("(:");
    out.push_str(keyword);
    for item in as_seq(value)? {
        out.push(' ');
        formula(out, item)?;
    }
    out.push(')');
    Ok(())
}

/// Writes `(:keyword formula)`.
fn formula_section(out: &mut String, keyword: &str, value: &Value) -> Result<(), String> {
    out.push_str("(:");
    out.push_str(keyword);
    out.push(' ');
    formula(out, value)?;
    out.push(')');
    Ok(())
}

impl Serialize for StructureDef {
    /// `{"action": {...}}`, `{"durative-action": {...}}` or `{"derived": {...}}`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StructureDef::Action(def) => Keyed("action", def).serialize(serializer),
            StructureDef::DurativeAction(def) => {
                Keyed("durative-action", def).serialize(serializer)
            }
            StructureDef::Derived(def) => Keyed("derived", def).serialize(serializer),
        }
    }
}

/// Writes a structure definition from `{"action": {"name": body}}` and the like.
fn structure_def(out: &mut String, value: &Value) -> Result<(), String> {
    let (kind, value) = single_entry(value)?;
    match kind {
        "action" => {
            let (name, body) = single_entry(value)?;
            action(out, name, body)
        }
        "durative-action" => {
            let (name, body) = single_entry(value)?;
            durative_action(out, name, body)
        }
        "derived" => derived(out, value),
        _ => Err(format!("unknown structure definition `{kind}`")),
    }
}

impl Serialize for StructureDefs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

/// The actions among a domain's structure definitions, as an object from name to body.
struct Actions<'a>(&'a [StructureDef]);

impl Serialize for Actions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        for def in self.0 {
            if let StructureDef::Action(action) = def {
                map.serialize_entry(action.symbol(), &ActionBody(action))?;
            }
        }
        map.end()
    }
}

/// The durative actions among a domain's structure definitions, by name.
struct DurativeActions<'a>(&'a [StructureDef]);

impl Serialize for DurativeActions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        for def in self.0 {
            if let StructureDef::DurativeAction(action) = def {
                map.serialize_entry(action.symbol(), &DurativeActionBody(action))?;
            }
        }
        map.end()
    }
}

/// The derived predicates among a domain's structure definitions.
struct Derived<'a>(&'a [StructureDef]);

impl Serialize for Derived<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;
        for def in self.0 {
            if let StructureDef::Derived(derived) = def {
                seq.serialize_element(derived)?;
            }
        }
        seq.end()
    }
}

impl Serialize for Domain {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let structure = self.structure().values();
        let has = |f: fn(&StructureDef) -> bool| structure.iter().any(f);
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("domain", self.name())?;
        if !self.extends().is_empty() {
            map.serialize_entry("extends", self.extends())?;
        }
        if !self.requirements().is_empty_including_unknown() {
            map.serialize_entry("requirements", self.requirements())?;
        }
        if !self.types().is_empty() {
            map.serialize_entry("types", self.types())?;
        }
        if !self.constants().is_empty() {
            map.serialize_entry("constants", self.constants())?;
        }
        if !self.predicates().is_empty() {
            map.serialize_entry("predicates", self.predicates())?;
        }
        if !self.functions().is_empty() {
            map.serialize_entry("functions", self.functions())?;
        }
        if !self.constraints().is_empty() {
            map.serialize_entry("constraints", self.constraints())?;
        }
        if !self.timeless().is_empty() {
            map.serialize_entry("timeless", self.timeless())?;
        }
        if has(|def| matches!(def, StructureDef::Action(_))) {
            map.serialize_entry("actions", &Actions(structure))?;
        }
        if has(|def| matches!(def, StructureDef::DurativeAction(_))) {
            map.serialize_entry("durative-actions", &DurativeActions(structure))?;
        }
        if has(|def| matches!(def, StructureDef::Derived(_))) {
            map.serialize_entry("derived", &Derived(structure))?;
        }
        map.end()
    }
}

/// Whether a section's value is an empty array or object, which PDDL leaves out.
fn is_empty(value: &Value) -> bool {
    match value {
        Value::Seq(items) => items.is_empty(),
        Value::Map(entries) => entries.is_empty(),
        _ => false,
    }
}

/// The string under `key` in an object's entries.
fn required_name<'a>(entries: &'a [(String, Value)], key: &str) -> Result<&'a str, String> {
    let (_, value) = entries
        .iter()
        .find(|(k, _)| k == key)
        .ok_or_else(|| format!("missing `{key}`"))?;
    as_str(value)
}

/// Writes `(define (domain ...) ...)`.
fn domain(out: &mut String, value: &Value) -> Result<(), String> {
    let entries = as_map(value)?;
    out.push_str("(define (domain ");
    atom(out, required_name(entries, "domain")?)?;
    out.push(')');
    for (key, value) in entries {
        let known = matches!(
            key.as_str(),
            "domain"
                | "extends"
                | "requirements"
                | "types"
                | "constants"
                | "predicates"
                | "functions"
                | "constraints"
                | "timeless"
                | "actions"
                | "durative-actions"
                | "derived"
        );
        if !known {
            return Err(format!("unknown key `{key}` in a domain"));
        }
        if key == "domain" || is_empty(value) {
            continue;
        }
        out.push(' ');
        match key.as_str() {
            "extends" => list_section(out, "extends", value)?,
            "requirements" => requirements(out, value)?,
            "types" | "constants" => typed_section(out, key, value)?,
            "predicates" => predicates(out, value)?,
            "functions" => functions(out, value)?,
            "constraints" => formula_section(out, "constraints", value)?,
            "timeless" => list_section(out, "timeless", value)?,
            "actions" => {
                for (i, (name, body)) in as_map(value)?.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    action(out, name, body)?;
                }
            }
            "durative-actions" => {
                for (i, (name, body)) in as_map(value)?.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    durative_action(out, name, body)?;
                }
            }
            "derived" => spaced(out, as_seq(value)?, derived)?,
            _ => unreachable!("checked above"),
        }
    }
    out.push(')');
    Ok(())
}

impl Serialize for InitElement {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            InitElement::Literal(literal) => literal.serialize(serializer),
            InitElement::At(time, literal) => Keyed("at", &(time, literal)).serialize(serializer),
            InitElement::IsValue(function, value) => {
                Keyed("=", &(function, value)).serialize(serializer)
            }
            InitElement::IsObject(function, name) => {
                Keyed("=", &(function, name)).serialize(serializer)
            }
        }
    }
}

impl Serialize for InitElements {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl Serialize for ProblemGoalDescription {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl Serialize for ProblemConstraintsDef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl Serialize for MetricSpec {
    /// `{"minimize": exp}` or `{"maximize": exp}`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Keyed(&self.optimization().to_string(), self.expression()).serialize(serializer)
    }
}

/// Writes `(:metric minimize exp)`.
fn metric(out: &mut String, value: &Value) -> Result<(), String> {
    let (optimization, exp) = single_entry(value)?;
    out.push_str("(:metric ");
    atom(out, optimization)?;
    out.push(' ');
    formula(out, exp)?;
    out.push(')');
    Ok(())
}

impl Serialize for LengthSpec {
    /// `{"serial": n, "parallel": n}`, either one optional.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if let Some(serial) = self.serial() {
            map.serialize_entry("serial", &serial)?;
        }
        if let Some(parallel) = self.parallel() {
            map.serialize_entry("parallel", &parallel)?;
        }
        map.end()
    }
}

/// Writes `(:length (:serial n) (:parallel n))`, in the order PDDL requires.
fn length(out: &mut String, value: &Value) -> Result<(), String> {
    let entries = as_map(value)?;
    if let Some((key, _)) = entries
        .iter()
        .find(|(k, _)| k != "serial" && k != "parallel")
    {
        return Err(format!("unknown key `{key}` in `length`"));
    }
    out.push_str("(:length");
    for keyword in ["serial", "parallel"] {
        if let Some((_, value)) = entries.iter().find(|(k, _)| k == keyword) {
            out.push_str(" (:");
            out.push_str(keyword);
            out.push(' ');
            formula(out, value)?;
            out.push(')');
        }
    }
    out.push(')');
    Ok(())
}

impl Serialize for Problem {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("problem", self.name())?;
        map.serialize_entry("domain", self.domain())?;
        if !self.requirements().is_empty_including_unknown() {
            map.serialize_entry("requirements", self.requirements())?;
        }
        if !self.objects().is_empty() {
            map.serialize_entry("objects", self.objects())?;
        }
        map.serialize_entry("init", self.init())?;
        map.serialize_entry("goal", self.goals())?;
        if !self.constraints().is_empty() {
            map.serialize_entry("constraints", self.constraints())?;
        }
        if let Some(metric) = self.metric_spec() {
            map.serialize_entry("metric", metric)?;
        }
        if let Some(length) = self.length_spec() {
            map.serialize_entry("length", length)?;
        }
        map.end()
    }
}

/// Writes `(define (problem ...) ...)`.
fn problem(out: &mut String, value: &Value) -> Result<(), String> {
    let entries = as_map(value)?;
    out.push_str("(define (problem ");
    atom(out, required_name(entries, "problem")?)?;
    out.push_str(") (:domain ");
    atom(out, required_name(entries, "domain")?)?;
    out.push(')');
    for (key, value) in entries {
        let known = matches!(
            key.as_str(),
            "problem"
                | "domain"
                | "requirements"
                | "objects"
                | "init"
                | "goal"
                | "constraints"
                | "metric"
                | "length"
        );
        if !known {
            return Err(format!("unknown key `{key}` in a problem"));
        }
        if matches!(key.as_str(), "problem" | "domain")
            || (key != "init" && key != "goal" && is_empty(value))
        {
            continue;
        }
        out.push(' ');
        match key.as_str() {
            "requirements" => requirements(out, value)?,
            "objects" => typed_section(out, "objects", value)?,
            "init" => list_section(out, "init", value)?,
            "goal" => formula_section(out, "goal", value)?,
            "constraints" => formula_section(out, "constraints", value)?,
            "metric" => metric(out, value)?,
            "length" => length(out, value)?,
            _ => unreachable!("checked above"),
        }
    }
    out.push(')');
    Ok(())
}

impl Serialize for Definition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Definition::Domain(domain) => domain.serialize(serializer),
            Definition::Problem(problem) => problem.serialize(serializer),
        }
    }
}

/// Writes a domain or, if the object has a `problem` key, a problem.
fn definition(out: &mut String, value: &Value) -> Result<(), String> {
    if as_map(value)?.iter().any(|(key, _)| key == "problem") {
        problem(out, value)
    } else {
        domain(out, value)
    }
}

impl Serialize for PddlFile {
    /// `{"domains": [...], "problems": [...]}`, leaving out an empty list.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        if !self.domains.is_empty() {
            map.serialize_entry("domains", &self.domains)?;
        }
        if !self.problems.is_empty() {
            map.serialize_entry("problems", &self.problems)?;
        }
        map.end()
    }
}

/// Writes every domain, then every problem.
fn pddl_file(out: &mut String, value: &Value) -> Result<(), String> {
    for (key, value) in as_map(value)? {
        let render = match key.as_str() {
            "domains" => domain,
            "problems" => problem,
            _ => return Err(format!("unknown key `{key}` in a PDDL file")),
        };
        for item in as_seq(value)? {
            render(out, item)?;
            out.push('\n');
        }
    }
    Ok(())
}

deserialize_from_pddl_or_default!(
    Requirements => requirements,
    PredicateDefinitions => predicates,
    Timeless => |out, value| list_section(out, "timeless", value),
);

deserialize_from_pddl!(
    Requirement => requirement,
    Types => |out, value| typed_section(out, "types", value),
    Constants => |out, value| typed_section(out, "constants", value),
    Objects => |out, value| typed_section(out, "objects", value),
    AtomicFormulaSkeleton => |out, value| {
        let (name, parameters) = single_entry(value)?;
        skeleton(out, name, parameters)
    },
    AtomicFunctionSkeleton => |out, value| {
        let (name, parameters) = single_entry(value)?;
        skeleton(out, name, parameters)
    },
    Functions => functions,
    DomainConstraintsDef => |out, value| formula_section(out, "constraints", value),
    StructureDef => structure_def,
    Domain => domain,
    InitElement => formula,
    InitElements => |out, value| list_section(out, "init", value),
    ProblemGoalDescription => |out, value| formula_section(out, "goal", value),
    ProblemConstraintsDef => |out, value| formula_section(out, "constraints", value),
    MetricSpec => metric,
    LengthSpec => length,
    Problem => problem,
    Definition => definition,
    PddlFile => pddl_file,
);
