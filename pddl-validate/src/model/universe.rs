//! The objects of a task and the types they belong to.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use pddl::{Domain, Problem, Type};

use crate::atom::lower;
use crate::error::{ModelError, ModelErrorKind};

/// The name of the root type every object belongs to.
const OBJECT: &str = "object";

/// Every object of a domain and problem, with the types it belongs to.
///
/// Types form a hierarchy: an object of type `truck` is also of every supertype of
/// `truck`, up to `object`. The hierarchy may contain cycles (`a - b`, `b - a`); they make
/// the types equivalent and do not stop construction.
#[derive(Debug, Clone)]
pub struct Universe {
    /// Every declared type, lowercased, with the objects of that type and its subtypes.
    /// Always contains `object`.
    members: BTreeMap<String, BTreeSet<String>>,
}

impl Universe {
    /// Collects the domain's constants and the problem's objects.
    ///
    /// A type named as a supertype counts as declared. An object declared twice, for example
    /// as a constant and again as an object, belongs to the types of both declarations.
    ///
    /// # Errors
    ///
    /// [`ModelErrorKind::UnknownType`] if an object is declared with an undeclared type.
    pub fn new(domain: &Domain, problem: &Problem) -> Result<Self, ModelError> {
        let parents = type_parents(domain);
        let mut members: BTreeMap<String, BTreeSet<String>> = parents
            .keys()
            .map(|name| (name.clone(), BTreeSet::new()))
            .collect();
        let declarations = domain.constants().iter().chain(problem.objects().iter());
        for typed in declarations {
            let object = lower(typed.value());
            let declared = primitive_names(typed.type_());
            let types = ancestors(&parents, declared)
                .map_err(|name| ModelError::new("objects", ModelErrorKind::UnknownType(name)))?;
            for ty in types {
                members.entry(ty).or_default().insert(object.clone());
            }
        }

        Ok(Self { members })
    }

    /// Returns `true` if `object` is declared.
    pub fn contains(&self, object: &str) -> bool {
        self.members
            .get(OBJECT)
            .is_some_and(|all| all.contains(object))
    }

    /// Returns the first primitive type of `ty` that is not declared, if any.
    pub fn unknown_type(&self, ty: &Type) -> Option<String> {
        primitive_names(ty)
            .into_iter()
            .find(|name| !self.members.contains_key(name))
    }

    /// Returns `true` if `object` belongs to `ty`, or to one of its alternatives for an
    /// `either` type.
    pub fn is_of(&self, object: &str, ty: &Type) -> bool {
        primitive_names(ty).iter().any(|name| {
            self.members
                .get(name)
                .is_some_and(|set| set.contains(object))
        })
    }

    /// Returns the objects of `ty`, without duplicates, sorted per alternative type.
    /// An undeclared type has no objects.
    pub fn objects_of(&self, ty: &Type) -> Vec<&str> {
        let mut objects: Vec<&str> = Vec::new();
        for name in primitive_names(ty) {
            for object in self.members.get(&name).into_iter().flatten() {
                if !objects.contains(&object.as_str()) {
                    objects.push(object);
                }
            }
        }

        objects
    }
}

/// Returns `ty` as a lowercased list of primitive type names: one for a plain type, the
/// alternatives for `(either ...)`.
pub fn primitive_names(ty: &Type) -> Vec<String> {
    match ty {
        Type::Exactly(primitive) => alloc::vec![lower(primitive)],
        Type::EitherOf(primitives) => primitives.iter().map(|p| lower(p)).collect(),
    }
}

/// Renders `ty` as PDDL, such as `truck` or `(either car truck)`.
pub fn type_text(ty: &Type) -> String {
    match ty {
        Type::Exactly(primitive) => lower(primitive),
        Type::EitherOf(primitives) => {
            let names: Vec<String> = primitives.iter().map(|p| lower(p)).collect();
            alloc::format!("(either {})", names.join(" "))
        }
    }
}

/// Maps every type of the domain, lowercased, to its direct supertypes. `object` is always
/// present, and so is every type named only as a supertype.
fn type_parents(domain: &Domain) -> BTreeMap<String, Vec<String>> {
    let mut parents: BTreeMap<String, Vec<String>> = BTreeMap::new();
    parents.insert(OBJECT.to_string(), Vec::new());
    for typed in domain.types().iter() {
        let name = lower(typed.value());
        for parent in primitive_names(typed.type_()) {
            parents.entry(parent.clone()).or_default();
            if parent != name {
                parents.entry(name.clone()).or_default().push(parent);
            }
        }

        parents.entry(name).or_default();
    }

    parents
}

/// Returns `start` with all their supertypes, plus `object`.
///
/// # Errors
///
/// Returns the first name that is not a declared type.
fn ancestors(
    parents: &BTreeMap<String, Vec<String>>,
    start: Vec<String>,
) -> Result<BTreeSet<String>, String> {
    let mut seen = BTreeSet::new();
    let mut pending = start;
    pending.push(OBJECT.to_string());
    while let Some(name) = pending.pop() {
        let direct = parents.get(&name).ok_or_else(|| name.clone())?;
        if seen.insert(name) {
            pending.extend(direct.iter().cloned());
        }
    }

    Ok(seen)
}
