//! The PDDL files the server knows, and the names across them.
//!
//! A problem sees the names of the domain its `(:domain ...)` names, in any
//! known file: the open documents, and the `.pddl` files found in the
//! workspace folders when the server started.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::ops::Range;

use lsp_types::Url;
use pddl::syntax;

use crate::analysis::{Analysis, Definition, Expect, Kind, Occurrence};
use crate::line_index::{Encoding, LineIndex};

/// A PDDL file and its names.
#[derive(Debug, Clone)]
pub struct Document {
    /// The file's text.
    pub text: String,
    /// The line starts of `text`.
    pub lines: LineIndex,
    /// The names in `text`.
    pub analysis: Analysis,
    /// Whether the editor has the file open, so `text` may differ from the disk.
    pub open: bool,
}

impl Document {
    /// Analyzes `text`.
    #[must_use]
    pub fn new(text: String, encoding: Encoding, open: bool) -> Self {
        let analysis = syntax::read(&text)
            .map(|tree| Analysis::new(&text, &tree))
            .unwrap_or_default();
        Self {
            lines: LineIndex::new(&text, encoding),
            text,
            analysis,
            open,
        }
    }
}

/// A definition in some document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    /// The document.
    pub uri: Url,
    /// The index of the definition in its document's [`Analysis::definitions`].
    pub index: usize,
}

/// How bad a [`Problem`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// The file is not valid PDDL.
    Error,
    /// The file may be fine, but something could not be checked.
    Warning,
}

/// A mistake found in a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// Where the mistake is.
    pub span: Range<usize>,
    /// How bad it is.
    pub severity: Severity,
    /// What is wrong.
    pub message: String,
    /// A word to replace the text at `span` with, to fix the mistake.
    pub fix: Option<String>,
}

/// The documents the server knows.
#[derive(Debug, Default)]
pub struct Workspace {
    /// Every known document by its URI.
    pub documents: BTreeMap<Url, Document>,
}

impl Workspace {
    /// The definition the name at `offset` in `uri` refers to, or defines.
    #[must_use]
    pub fn target_at(&self, uri: &Url, offset: usize) -> Option<Target> {
        let document = self.documents.get(uri)?;
        match document.analysis.occurrence_at(offset)? {
            Occurrence::Definition(index) => Some(Target {
                uri: uri.clone(),
                index,
            }),
            Occurrence::Reference(index) => self.resolve(uri, index),
            Occurrence::Variable(index) => {
                document.analysis.variables[index]
                    .binding
                    .map(|index| Target {
                        uri: uri.clone(),
                        index,
                    })
            }
        }
    }

    /// The definition of `target`.
    #[must_use]
    pub fn definition(&self, target: &Target) -> Option<&Definition> {
        self.documents
            .get(&target.uri)?
            .analysis
            .definitions
            .get(target.index)
    }

    /// The definition reference `index` in `uri` refers to.
    #[must_use]
    pub fn resolve(&self, uri: &Url, index: usize) -> Option<Target> {
        let document = self.documents.get(uri)?;
        let reference = &document.analysis.references[index];
        let context = &document.analysis.contexts[reference.context];
        let matches = |d: &Definition, kind: Kind| {
            d.kind == kind && d.name.eq_ignore_ascii_case(&reference.name)
        };
        if reference.expect == Expect::Domain {
            return self.in_domain(uri, &reference.name, |d| d.kind == Kind::Domain);
        }
        for &kind in reference.expect.kinds() {
            let found = if kind == Kind::Object {
                // A problem's objects are visible in that problem only.
                let definitions = &document.analysis.definitions;
                definitions
                    .iter()
                    .position(|d| d.context == reference.context && matches(d, kind))
                    .map(|index| Target {
                        uri: uri.clone(),
                        index,
                    })
            } else {
                let domain = context.domain.as_deref()?;
                self.in_domain(uri, domain, |d| matches(d, kind))
            };
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// The domains named `domain`, each as its document and context index,
    /// those in `first` before those in other documents.
    fn domains<'w>(
        &'w self,
        first: &'w Url,
        domain: &'w str,
    ) -> impl Iterator<Item = (&'w Url, &'w Document, usize)> + 'w {
        let preferred = self.documents.get_key_value(first);
        let others = self.documents.iter().filter(move |(uri, _)| *uri != first);
        preferred
            .into_iter()
            .chain(others)
            .flat_map(move |(uri, document)| {
                let contexts = document.analysis.contexts.iter().enumerate();
                contexts
                    .filter(move |(_, c)| {
                        c.kind == Kind::Domain
                            && c.domain
                                .as_deref()
                                .is_some_and(|n| n.eq_ignore_ascii_case(domain))
                    })
                    .map(move |(index, _)| (uri, document, index))
            })
    }

    /// The first definition in a domain named `domain` for which `accept` holds.
    fn in_domain(
        &self,
        first: &Url,
        domain: &str,
        accept: impl Fn(&Definition) -> bool,
    ) -> Option<Target> {
        self.domains(first, domain)
            .find_map(|(uri, document, context)| {
                let definitions = &document.analysis.definitions;
                definitions
                    .iter()
                    .position(|d| d.context == context && accept(d))
                    .map(|index| Target {
                        uri: uri.clone(),
                        index,
                    })
            })
    }

    /// The definitions usable in `context` of `uri`: its own objects, and the
    /// definitions of the domain it sees, except variables.
    #[must_use]
    pub fn visible<'w>(&'w self, uri: &'w Url, context: usize) -> Vec<&'w Definition> {
        let Some(document) = self.documents.get(uri) else {
            return Vec::new();
        };
        let own = &document.analysis.contexts[context];
        let mut found: Vec<&Definition> = document
            .analysis
            .definitions
            .iter()
            .filter(|d| d.context == context && d.kind == Kind::Object)
            .collect();
        let Some(domain) = own.domain.as_deref() else {
            return found;
        };
        for (_, document, context) in self.domains(uri, domain) {
            found.extend(document.analysis.definitions.iter().filter(|d| {
                d.context == context && !matches!(d.kind, Kind::Variable | Kind::Domain)
            }));
        }
        found
    }

    /// Whether the domain that `context` in `uri` sees is known.
    fn domain_known(&self, uri: &Url, context: usize) -> bool {
        let Some(document) = self.documents.get(uri) else {
            return false;
        };
        let Some(domain) = document.analysis.contexts[context].domain.as_deref() else {
            return false;
        };
        self.domains(uri, domain).next().is_some()
    }

    /// Every name defining or referring to `target`, the definition first.
    #[must_use]
    pub fn references(&self, target: &Target) -> Vec<(Url, Range<usize>)> {
        let Some(definition) = self.definition(target) else {
            return Vec::new();
        };
        let mut found = vec![(target.uri.clone(), definition.span.clone())];
        if definition.kind == Kind::Variable {
            let analysis = &self.documents[&target.uri].analysis;
            let uses = analysis
                .variables
                .iter()
                .filter(|v| v.binding == Some(target.index));
            found.extend(uses.map(|v| (target.uri.clone(), v.span.clone())));
            return found;
        }
        for (uri, document) in &self.documents {
            // With the name fixed, what a reference resolves to depends only on
            // its context and what it expects, so resolve each pair once.
            let mut resolved = HashMap::new();
            for (index, reference) in document.analysis.references.iter().enumerate() {
                if !reference.name.eq_ignore_ascii_case(&definition.name) {
                    continue;
                }
                let key = (reference.context, reference.expect);
                let hit = *resolved
                    .entry(key)
                    .or_insert_with(|| self.resolve(uri, index).as_ref() == Some(target));
                if hit {
                    found.push((uri.clone(), reference.span.clone()));
                }
            }
        }
        found
    }

    /// The mistakes in `uri`: parse errors, then names that don't resolve.
    #[must_use]
    pub fn problems(&self, uri: &Url) -> Vec<Problem> {
        let Some(document) = self.documents.get(uri) else {
            return Vec::new();
        };
        let mut problems: Vec<Problem> = pddl::parsers::check(&document.text)
            .iter()
            .map(parse_problem)
            .collect();
        let analysis = &document.analysis;
        for (index, reference) in analysis.references.iter().enumerate() {
            let error = |message: String| Problem {
                span: reference.span.clone(),
                severity: Severity::Error,
                message,
                fix: None,
            };
            match self.resolve(uri, index) {
                None if reference.expect == Expect::Domain => problems.push(Problem {
                    severity: Severity::Warning,
                    ..error(format!(
                        "domain `{}` is not in the workspace, so names from it are not checked",
                        reference.name
                    ))
                }),
                None if reference.expect == Expect::Type
                    && ["object", "number"]
                        .iter()
                        .any(|t| reference.name.eq_ignore_ascii_case(t)) => {}
                None if self.domain_known(uri, reference.context) => {
                    let what = match (reference.expect, analysis.contexts[reference.context].kind) {
                        (Expect::PredicateOrFunction, _) => "predicate or function",
                        (Expect::Term, Kind::Problem) => "object or constant",
                        (Expect::Term, _) => "constant",
                        (Expect::Type | Expect::Domain, _) => "type",
                    };
                    problems.push(error(format!("unknown {what} `{}`", reference.name)));
                }
                None => {}
                Some(target) => {
                    let Some(definition) = self.definition(&target) else {
                        continue;
                    };
                    let (Some(expected), Some(given)) = (definition.arity(), reference.args) else {
                        continue;
                    };
                    if expected != given {
                        problems.push(error(format!(
                            "{} `{}` takes {expected} argument{}, but {given} {} given",
                            definition.kind.label(),
                            definition.name,
                            if expected == 1 { "" } else { "s" },
                            if given == 1 { "is" } else { "are" },
                        )));
                    }
                }
            }
        }
        for variable in &analysis.variables {
            if variable.binding.is_none() && !variable.name.eq_ignore_ascii_case("?duration") {
                problems.push(Problem {
                    span: variable.span.clone(),
                    severity: Severity::Error,
                    message: format!("variable `{}` is not declared", variable.name),
                    fix: None,
                });
            }
        }
        problems.extend(duplicates(analysis));
        problems.sort_by_key(|p| p.span.start);
        problems
    }
}

/// Definitions repeated within the same domain, problem or parameter list.
fn duplicates(analysis: &Analysis) -> Vec<Problem> {
    let mut problems = Vec::new();
    let definitions = &analysis.definitions;
    for (i, later) in definitions.iter().enumerate() {
        let earlier = definitions[..i].iter().find(|d| {
            d.kind == later.kind
                && d.context == later.context
                && d.scope == later.scope
                && d.name.eq_ignore_ascii_case(&later.name)
        });
        if earlier.is_some() && !later.name.is_empty() {
            problems.push(Problem {
                span: later.span.clone(),
                severity: Severity::Error,
                message: format!("duplicate {} `{}`", later.kind.label(), later.name),
                fix: None,
            });
        }
    }
    problems
}

/// Describes a parse error, without the line and column the editor shows anyway.
fn parse_problem(error: &pddl::ParseError) -> Problem {
    let mut message = String::new();
    if let Some(keyword) = error.keyword() {
        let _ = write!(message, "in `{keyword}`: ");
    }
    if let Some(reason) = error.reason() {
        let _ = write!(message, "{reason}: ");
    }
    match error.found() {
        Some(found) => {
            let _ = write!(message, "unexpected `{found}`");
        }
        None => message.push_str("unexpected end of input"),
    }
    if let Some(help) = error.help() {
        let _ = write!(message, "; {help}");
    }
    // The help reads "did you mean `word`?"; the error span covers the
    // misspelled atom, after a list's `(`.
    let fix = error
        .help()
        .and_then(|h| h.split('`').nth(1))
        .map(str::to_string);
    let mut span = error.span();
    if fix.is_some() && error.found().is_some_and(|f| f.starts_with('(')) {
        span.start += 1;
    }
    Problem {
        span,
        severity: Severity::Error,
        message,
        fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    const DOMAIN: &str = "(define (domain d)
  (:types truck place)
  (:constants depot - place)
  (:predicates (at ?t - truck ?p - place) (ready))
  (:action drive :parameters (?t - truck ?to - place)
    :effect (and (at ?t ?to) (at ?t depot))))";

    fn workspace(files: &[(&str, &str)]) -> Workspace {
        let mut workspace = Workspace::default();
        for (name, text) in files {
            let uri = Url::parse(&format!("file:///{name}")).unwrap();
            let document = Document::new((*text).to_string(), Encoding::Utf8, true);
            workspace.documents.insert(uri, document);
        }
        workspace
    }

    fn uri(name: &str) -> Url {
        Url::parse(&format!("file:///{name}")).unwrap()
    }

    fn messages(workspace: &Workspace, name: &str) -> Vec<String> {
        workspace
            .problems(&uri(name))
            .into_iter()
            .map(|p| p.message)
            .collect()
    }

    #[test]
    fn resolves_problem_names_in_another_file() {
        let problem = "(define (problem p) (:domain D) (:objects t1 - truck) (:init (at t1 depot)) (:goal (ready)))";
        let ws = workspace(&[("d.pddl", DOMAIN), ("p.pddl", problem)]);
        let at = problem.find("(at").unwrap() + 1;
        let target = ws.target_at(&uri("p.pddl"), at).unwrap();
        check!(target.uri == uri("d.pddl"));
        check!(ws.definition(&target).unwrap().detail == "(at ?t - truck ?p - place)");
        let depot = ws
            .target_at(&uri("p.pddl"), problem.find("depot").unwrap())
            .unwrap();
        check!(ws.definition(&depot).unwrap().kind == Kind::Constant);
        let t1 = ws
            .target_at(&uri("p.pddl"), problem.rfind("t1").unwrap())
            .unwrap();
        check!(ws.definition(&t1).unwrap().kind == Kind::Object);
        check!(messages(&ws, "p.pddl").is_empty());
        check!(messages(&ws, "d.pddl").is_empty());
    }

    #[test]
    fn finds_references_across_files() {
        let problem = "(define (problem p) (:domain d) (:init (at t depot)) (:goal (at t depot)))";
        let ws = workspace(&[("d.pddl", DOMAIN), ("p.pddl", problem)]);
        let target = ws
            .target_at(&uri("d.pddl"), DOMAIN.find("(at ?t -").unwrap() + 1)
            .unwrap();
        let refs = ws.references(&target);
        let in_problem = refs.iter().filter(|(u, _)| *u == uri("p.pddl")).count();
        check!(refs.len() == 5);
        check!(in_problem == 2);
    }

    #[test]
    fn reports_unknown_names_arity_and_duplicates() {
        let domain = "(define (domain d)
  (:types truck)
  (:predicates (at ?t - truk) (at ?x) (ready))
  (:action a :parameters (?t - truck ?t)
    :precondition (and (at ?t ?u) (ready ?t) (gone))
    :effect (at c)))";
        let ws = workspace(&[("d.pddl", domain)]);
        check!(
            messages(&ws, "d.pddl")
                == [
                    "unknown type `truk`",
                    "duplicate predicate `at`",
                    "duplicate variable `?t`",
                    "predicate `at` takes 1 argument, but 2 are given",
                    "variable `?u` is not declared",
                    "predicate `ready` takes 0 arguments, but 1 is given",
                    "unknown predicate or function `gone`",
                    "unknown constant `c`",
                ]
        );
    }

    #[test]
    fn keeps_several_domains_and_problems_in_one_file_apart() {
        let file = "(define (domain trucks)
  (:predicates (at ?t ?p)))
(define (domain boats)
  (:predicates (docked ?b ?p)))
(define (problem p1) (:domain trucks) (:objects t home)
  (:init (at t home)) (:goal (at t home)))
(define (problem p2) (:domain boats) (:objects b home)
  (:init (docked b home)) (:goal (at b home)))";
        let ws = workspace(&[("all.pddl", file)]);
        let analysis = &ws.documents[&uri("all.pddl")].analysis;
        let contexts: Vec<_> = analysis
            .contexts
            .iter()
            .map(|c| (c.name.as_str(), c.domain.as_deref()))
            .collect();
        check!(
            contexts
                == [
                    ("trucks", Some("trucks")),
                    ("boats", Some("boats")),
                    ("p1", Some("trucks")),
                    ("p2", Some("boats")),
                ]
        );
        // `at` exists in `trucks` only, so p2, which uses `boats`, can't use it.
        check!(messages(&ws, "all.pddl") == ["unknown predicate or function `at`"]);
        let unknown = ws.problems(&uri("all.pddl"))[0].span.start;
        check!(unknown == file.rfind("(at b").unwrap() + 1);

        // Each problem's `home` is its own object.
        let home_p1 = ws.target_at(&uri("all.pddl"), file.find("t home)").unwrap() + 2);
        let home_p2 = ws.target_at(&uri("all.pddl"), file.find("b home)").unwrap() + 2);
        check!(home_p1.is_some() && home_p2.is_some() && home_p1 != home_p2);

        // `at` in p1 leads to the `trucks` domain in the same file.
        let at = ws
            .target_at(&uri("all.pddl"), file.find("(at t").unwrap() + 1)
            .unwrap();
        let definition = ws.definition(&at).unwrap();
        check!(analysis.contexts[definition.context].name == "trucks");
    }

    #[test]
    fn checks_problems_only_against_a_known_domain() {
        let problem = "(define (problem p) (:domain elsewhere) (:init (at a)) (:goal (at a)))";
        let ws = workspace(&[("p.pddl", problem)]);
        check!(
            messages(&ws, "p.pddl")
                == ["domain `elsewhere` is not in the workspace, so names from it are not checked"]
        );
    }

    #[test]
    fn parse_errors_carry_a_fix() {
        let ws = workspace(&[("d.pddl", "(define (domain d) (:predicats (p)))")]);
        let problems = ws.problems(&uri("d.pddl"));
        check!(
            problems[0].message
                == "unknown section: unexpected `(:predicats (p)))`; did you mean `:predicates`?"
        );
        check!(problems[0].fix.as_deref() == Some(":predicates"));
        let text = &ws.documents[&uri("d.pddl")].text;
        check!(&text[problems[0].span.clone()] == ":predicats");
    }
}
