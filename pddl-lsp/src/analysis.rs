//! Finds every name in a PDDL file: what it defines, what it refers to, and where.
//!
//! The analysis works on the [syntax tree](pddl::syntax), not the typed AST, so it
//! knows the position of every name and still works on a file with parse errors.
//! It only looks at one file; [`Workspace`](crate::workspace::Workspace) resolves
//! references across files.

use std::collections::HashSet;
use std::ops::Range;

use pddl::syntax::{self, Node};

/// What a definition defines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// A domain, `(define (domain <name>) ...)`.
    Domain,
    /// A problem, `(define (problem <name>) ...)`.
    Problem,
    /// A type in a domain's `:types`.
    Type,
    /// A constant in a domain's `:constants`.
    Constant,
    /// An object in a problem's `:objects`.
    Object,
    /// A predicate in a domain's `:predicates`.
    Predicate,
    /// A numeric or object fluent in a domain's `:functions`.
    Function,
    /// An action, durative action.
    Action,
    /// A parameter or quantified variable, such as `?x`.
    Variable,
}

impl Kind {
    /// The kind's name in messages, such as `predicate`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Problem => "problem",
            Self::Type => "type",
            Self::Constant => "constant",
            Self::Object => "object",
            Self::Predicate => "predicate",
            Self::Function => "function",
            Self::Action => "action",
            Self::Variable => "variable",
        }
    }
}

/// A domain or problem in the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    /// [`Kind::Domain`] or [`Kind::Problem`].
    pub kind: Kind,
    /// The domain's or problem's name, empty if it has none.
    pub name: String,
    /// The `(define ...)` form.
    pub range: Range<usize>,
    /// The domain whose names are visible: a domain's own name, or the name
    /// in a problem's `(:domain ...)`.
    pub domain: Option<String>,
}

/// A name being defined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    /// What it defines.
    pub kind: Kind,
    /// The name as written.
    pub name: String,
    /// The name's byte range.
    pub span: Range<usize>,
    /// The byte range of the whole form that defines it, such as the action.
    pub range: Range<usize>,
    /// The index of the domain or problem it belongs to, in [`Analysis::contexts`].
    pub context: usize,
    /// A one-line summary, such as `(at ?x - physob ?y - location)`.
    pub detail: String,
    /// The parameters of a predicate, function or action, as written, such as `?x - physob`.
    pub params: Vec<String>,
    /// Where a variable can be used, such as its action or `forall`.
    pub scope: Option<Range<usize>>,
}

impl Definition {
    /// The number of arguments a predicate, function or action takes.
    #[must_use]
    pub fn arity(&self) -> Option<usize> {
        matches!(self.kind, Kind::Predicate | Kind::Function | Kind::Action)
            .then_some(self.params.len())
    }
}

/// What a referenced name is expected to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Expect {
    /// The head of a formula or function term, such as `at` in `(at ?x ?y)`.
    PredicateOrFunction,
    /// A constant or object used as an argument, or a function without parameters.
    Term,
    /// A type after `-`, or in `(either ...)`.
    Type,
    /// The domain in a problem's `(:domain ...)`.
    Domain,
}

impl Expect {
    /// The kinds of definition a reference can resolve to, best first.
    #[must_use]
    pub const fn kinds(self) -> &'static [Kind] {
        match self {
            Self::PredicateOrFunction => &[Kind::Predicate, Kind::Function],
            Self::Term => &[Kind::Object, Kind::Constant, Kind::Function],
            Self::Type => &[Kind::Type],
            Self::Domain => &[Kind::Domain],
        }
    }
}

/// A name that refers to a definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// What the name is expected to be.
    pub expect: Expect,
    /// The name as written.
    pub name: String,
    /// The name's byte range.
    pub span: Range<usize>,
    /// The index of the domain or problem it is in, in [`Analysis::contexts`].
    pub context: usize,
    /// The number of arguments given, for the head of a formula.
    pub args: Option<usize>,
    /// The byte offset at which each argument starts, for the head of a
    /// predicate or function applied to terms. Empty otherwise.
    pub argument_starts: Vec<usize>,
}

/// A use of a variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableUse {
    /// The variable as written, with its `?`.
    pub name: String,
    /// The variable's byte range.
    pub span: Range<usize>,
    /// The index of the variable's definition in [`Analysis::definitions`],
    /// or `None` if no enclosing form declares it.
    pub binding: Option<usize>,
}

/// The names of one file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Analysis {
    /// The domains and problems, in order.
    pub contexts: Vec<Context>,
    /// Every name being defined, in order.
    pub definitions: Vec<Definition>,
    /// Every name referring to a domain-level or problem-level definition.
    pub references: Vec<Reference>,
    /// Every use of a variable.
    pub variables: Vec<VariableUse>,
    /// The names this file refers to, in lowercase, by what they're expected to be.
    /// Lets the workspace find unused definitions without walking every reference.
    pub referenced: HashSet<(Expect, String)>,
}

/// What is at a position: a definition, a reference or a variable use, by index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occurrence {
    /// An index into [`Analysis::definitions`].
    Definition(usize),
    /// An index into [`Analysis::references`].
    Reference(usize),
    /// An index into [`Analysis::variables`].
    Variable(usize),
}

impl Analysis {
    /// Finds the names in `tree`, read from `text`.
    #[must_use]
    pub fn new(text: &str, tree: &syntax::SyntaxTree) -> Self {
        let mut walker = Walker {
            text,
            out: Self::default(),
            scope: Vec::new(),
            context: 0,
        };
        for node in &tree.nodes {
            walker.top_level(node);
        }
        let mut out = walker.out;
        out.referenced = out
            .references
            .iter()
            .map(|r| (r.expect, r.name.to_ascii_lowercase()))
            .collect();
        out
    }

    /// The name at byte `offset`, including an offset just past its end.
    #[must_use]
    pub fn occurrence_at(&self, offset: usize) -> Option<Occurrence> {
        let hit = |span: &Range<usize>| span.start <= offset && offset <= span.end;
        self.definitions
            .iter()
            .position(|d| hit(&d.span))
            .map(Occurrence::Definition)
            .or_else(|| {
                self.references
                    .iter()
                    .position(|r| hit(&r.span))
                    .map(Occurrence::Reference)
            })
            .or_else(|| {
                self.variables
                    .iter()
                    .position(|v| hit(&v.span))
                    .map(Occurrence::Variable)
            })
    }

    /// The context, if any, whose `(define ...)` contains byte `offset`.
    #[must_use]
    pub fn context_at(&self, offset: usize) -> Option<usize> {
        self.contexts
            .iter()
            .position(|c| c.range.start <= offset && offset <= c.range.end)
    }
}

/// The byte range of a node, including a list's `)`.
#[must_use]
pub fn node_range(node: &Node) -> Range<usize> {
    match node.kind {
        syntax::Kind::List(_) if node.closed => node.start..node.end + 1,
        _ => node.start..node.end,
    }
}

/// Whether `atom` is a number, such as `1`, `-2.5` or `.5`.
#[must_use]
pub fn is_number(atom: &str) -> bool {
    atom.parse::<f64>().is_ok() && !atom.chars().any(char::is_alphabetic)
}

/// Collects the names of a file, one form at a time.
struct Walker<'t> {
    /// The text the tree was read from, for details.
    text: &'t str,
    /// What was found so far.
    out: Analysis,
    /// The variables in scope, innermost last, as indices into the definitions.
    scope: Vec<usize>,
    /// The index of the current context.
    context: usize,
}

/// The items of a typed list such as `a b - t c`: each name with the type after its `-`.
type TypedItems<'n, 'a> = Vec<(&'n Node<'a>, Option<&'n Node<'a>>)>;

/// Splits a typed list into its items, each with the type node after its `-`, if any.
fn typed_list<'n, 'a>(nodes: &'n [Node<'a>]) -> TypedItems<'n, 'a> {
    let mut items: TypedItems = Vec::new();
    let mut untyped = 0;
    let mut i = 0;
    while i < nodes.len() {
        if nodes[i].is("-") {
            let ty = nodes.get(i + 1);
            for item in &mut items[untyped..] {
                item.1 = ty;
            }
            untyped = items.len();
            i += 2;
        } else {
            items.push((&nodes[i], None));
            i += 1;
        }
    }
    items
}

impl Walker<'_> {
    /// Collapses the source of `node` into one line.
    fn source(&self, node: &Node) -> String {
        let range = node_range(node);
        self.text[range]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Records a definition of `name` and returns its index.
    fn define(&mut self, kind: Kind, name: &Node, range: Range<usize>, detail: String) -> usize {
        self.out.definitions.push(Definition {
            kind,
            name: name.atom().unwrap_or_default().to_string(),
            span: name.start..name.end,
            range,
            context: self.context,
            detail,
            params: Vec::new(),
            scope: None,
        });
        self.out.definitions.len() - 1
    }

    /// Records a reference to `name`, if it is an atom.
    fn refer(&mut self, expect: Expect, name: &Node, args: Option<usize>) {
        if let Some(atom) = name.atom() {
            self.out.references.push(Reference {
                expect,
                name: atom.to_string(),
                span: name.start..name.end,
                context: self.context,
                args,
                argument_starts: Vec::new(),
            });
        }
    }

    /// Records the types in a type node: an atom or `(either a b ...)`.
    fn type_ref(&mut self, ty: &Node) {
        match ty.children() {
            None => self.refer(Expect::Type, ty, None),
            Some(children) => {
                let skip = usize::from(ty.is("either") || ty.head() == Some("either"));
                for child in &children[skip..] {
                    self.type_ref(child);
                }
            }
        }
    }

    /// Records the names of a typed list as definitions of `kind`, with their
    /// types as references, and returns their indices.
    fn typed_defs(
        &mut self,
        kind: Kind,
        nodes: &[Node],
        range: &Range<usize>,
        scope: Option<&Range<usize>>,
    ) -> Vec<usize> {
        let mut defined = Vec::new();
        for (name, ty) in typed_list(nodes) {
            if name.atom().is_none() {
                self.formula(name);
                continue;
            }
            let ty_text = ty.map(|t| self.source(t));
            let detail = match (kind, &ty_text) {
                (Kind::Variable, Some(ty)) => format!("{} - {ty}", self.source(name)),
                (Kind::Variable, None) => self.source(name),
                (_, Some(ty)) => format!("{} {} - {ty}", kind.label(), self.source(name)),
                (_, None) => format!("{} {}", kind.label(), self.source(name)),
            };
            let index = self.define(kind, name, range.clone(), detail);
            self.out.definitions[index].scope = scope.cloned();
            defined.push(index);
        }
        self.type_refs(typed_list(nodes).into_iter().filter_map(|(_, ty)| ty));
        defined
    }

    /// Records the type nodes `types`, of which consecutive repeats are the same `-`.
    fn type_refs<'n, 'a: 'n>(&mut self, types: impl Iterator<Item = &'n Node<'a>>) {
        let mut previous = None;
        for ty in types {
            if previous != Some(ty.start) {
                self.type_ref(ty);
            }
            previous = Some(ty.start);
        }
    }

    /// Declares the variables of a typed list for use within `scope`, and returns
    /// their parameter texts.
    fn bind(&mut self, vars: &Node, scope: &Range<usize>) -> Vec<String> {
        let Some(children) = vars.children() else {
            return Vec::new();
        };
        let defined = self.typed_defs(Kind::Variable, children, scope, Some(scope));
        self.scope.extend(&defined);
        defined
            .iter()
            .map(|&i| self.out.definitions[i].detail.clone())
            .collect()
    }

    /// A top-level form: a domain or problem definition.
    fn top_level(&mut self, node: &Node) {
        let Some(children) = node.children() else {
            return;
        };
        if !node
            .head()
            .is_some_and(|h| h.eq_ignore_ascii_case("define"))
        {
            return;
        }
        let header = children.get(1);
        let kind = match header.and_then(Node::head).map(str::to_ascii_lowercase) {
            Some(head) if head == "domain" => Kind::Domain,
            Some(head) if head == "problem" => Kind::Problem,
            _ => return,
        };
        let name = header
            .and_then(Node::children)
            .and_then(|c| c.get(1))
            .filter(|n| n.atom().is_some());
        let range = node_range(node);
        self.context = self.out.contexts.len();
        self.out.contexts.push(Context {
            kind,
            name: name.and_then(Node::atom).unwrap_or_default().to_string(),
            range: range.clone(),
            domain: name
                .filter(|_| kind == Kind::Domain)
                .and_then(Node::atom)
                .map(str::to_string),
        });
        if let Some(name) = name {
            let detail = format!("{} {}", kind.label(), self.source(name));
            self.define(kind, name, range, detail);
        }
        for section in children.iter().skip(2) {
            match kind {
                Kind::Domain => self.domain_section(section),
                _ => self.problem_section(section),
            }
        }
    }

    /// A section of a domain, such as `(:predicates ...)`.
    fn domain_section(&mut self, section: &Node) {
        let Some([head, args @ ..]) = section.children() else {
            return;
        };
        let range = node_range(section);
        match head.atom().map(str::to_ascii_lowercase).as_deref() {
            Some(":types") => {
                self.typed_defs(Kind::Type, args, &range, None);
            }
            Some(":constants") => {
                self.typed_defs(Kind::Constant, args, &range, None);
            }
            Some(":predicates") => {
                for skeleton in args {
                    self.skeleton(Kind::Predicate, skeleton, None);
                }
            }
            Some(":functions") => {
                let items = typed_list(args);
                for &(skeleton, ty) in &items {
                    self.skeleton(Kind::Function, skeleton, ty);
                }
                let types = items.iter().filter_map(|(_, ty)| *ty);
                self.type_refs(types.filter(|t| !t.is("number")));
            }
            Some(":constraints" | ":timeless") => self.formulas(args),
            Some(":action" | ":durative-action") => self.action(section, args),
            Some(":derived") => {
                let scope_start = self.scope.len();
                if let Some(skeleton) = args.first() {
                    if let Some([name, vars @ ..]) = skeleton.children() {
                        let count = typed_list(vars).len();
                        self.refer(Expect::PredicateOrFunction, name, Some(count));
                        let defined = self.typed_defs(Kind::Variable, vars, &range, Some(&range));
                        self.scope.extend(defined);
                    }
                }
                self.formulas(args.get(1..).unwrap_or_default());
                self.scope.truncate(scope_start);
            }
            _ => {}
        }
    }

    /// A predicate or function skeleton such as `(at ?x - t)`, with a function's type.
    fn skeleton(&mut self, kind: Kind, skeleton: &Node, ty: Option<&Node>) {
        let Some([name, vars @ ..]) = skeleton.children() else {
            return;
        };
        if name.atom().is_none() {
            return;
        }
        let range = node_range(skeleton);
        let mut detail = self.source(skeleton);
        if let Some(ty) = ty {
            detail = format!("{detail} - {}", self.source(ty));
        }
        let index = self.define(kind, name, range.clone(), detail);
        let params = self.typed_defs(Kind::Variable, vars, &range, Some(&range));
        self.out.definitions[index].params = params
            .iter()
            .map(|&i| self.out.definitions[i].detail.clone())
            .collect();
    }

    /// An action or durative action, `(:action <name> :parameters (...) ...)`.
    fn action(&mut self, section: &Node, args: &[Node]) {
        let range = node_range(section);
        let scope_start = self.scope.len();
        let index = args
            .first()
            .filter(|n| n.atom().is_some())
            .map(|name| self.define(Kind::Action, name, range.clone(), String::new()));
        let parameters = args
            .windows(2)
            .find(|pair| pair[0].is(":parameters"))
            .map(|pair| &pair[1]);
        let params = parameters.map_or_else(Vec::new, |vars| self.bind(vars, &range));
        if let Some(index) = index {
            let head = section.head().unwrap_or(":action");
            let name = &self.out.definitions[index].name;
            self.out.definitions[index].detail =
                format!("({head} {name} :parameters ({}))", params.join(" "));
            self.out.definitions[index].params = params;
        }
        let mut i = 1;
        while i < args.len() {
            if args[i].atom().is_some_and(|a| a.starts_with(':')) {
                if let Some(value) = args.get(i + 1) {
                    if !args[i].is(":parameters") {
                        self.formula(value);
                    }
                }
                i += 2;
            } else {
                self.formula(&args[i]);
                i += 1;
            }
        }
        self.scope.truncate(scope_start);
    }

    /// A section of a problem, such as `(:init ...)`.
    fn problem_section(&mut self, section: &Node) {
        let Some([head, args @ ..]) = section.children() else {
            return;
        };
        let range = node_range(section);
        match head.atom().map(str::to_ascii_lowercase).as_deref() {
            Some(":domain") => {
                if let Some(name) = args.first() {
                    self.refer(Expect::Domain, name, None);
                    self.out.contexts[self.context].domain = name.atom().map(str::to_string);
                }
            }
            Some(":objects") => {
                self.typed_defs(Kind::Object, args, &range, None);
            }
            Some(":init" | ":goal" | ":constraints") => {
                self.formulas(args);
            }
            Some(":metric") => self.expressions(args.get(1..).unwrap_or_default()),
            _ => {}
        }
    }

    /// Each of `nodes` as a formula.
    fn formulas(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.formula(node);
        }
    }

    /// Each of `nodes` as an expression.
    fn expressions(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.expression(node);
        }
    }

    /// A goal, effect, constraint or initial fact.
    fn formula(&mut self, node: &Node) {
        let Some(children) = node.children() else {
            return self.term(node);
        };
        let Some(head) = children.first() else {
            return;
        };
        let Some(word) = head.atom() else {
            return self.formulas(children);
        };
        let args = &children[1..];
        let first_is = |words: &[&str]| {
            args.first()
                .is_some_and(|a| words.iter().any(|w| a.is(w)) || a.atom().is_some_and(is_number))
        };
        match word.to_ascii_lowercase().as_str() {
            "and" | "or" | "not" | "imply" | "when" | "always" | "sometime" | "at-most-once"
            | "sometime-after" | "sometime-before" | "within" | "always-within" | "hold-during"
            | "hold-after" => self.formulas(args),
            "at" if first_is(&["start", "end"]) => self.formulas(&args[1..]),
            "over" if first_is(&["all"]) => self.formulas(&args[1..]),
            "forall" | "exists" => {
                let scope_start = self.scope.len();
                if let Some(vars) = args.first() {
                    self.bind(vars, &node_range(node));
                }
                self.formulas(args.get(1..).unwrap_or_default());
                self.scope.truncate(scope_start);
            }
            // `(preference <name> <goal>)`, where the name is optional.
            "preference" => {
                self.formulas(args.last().map(std::slice::from_ref).unwrap_or_default());
            }
            "is-violated" => {}
            "=" | "<" | ">" | "<=" | ">=" | "assign" | "increase" | "decrease" | "scale-up"
            | "scale-down" | "+" | "-" | "*" | "/" => {
                self.expressions(args);
            }
            _ => self.application(head, args),
        }
    }

    /// A predicate or function applied to terms, such as `(at ?x home)`.
    fn application(&mut self, head: &Node, args: &[Node]) {
        self.refer(Expect::PredicateOrFunction, head, Some(args.len()));
        if head.atom().is_some() {
            if let Some(reference) = self.out.references.last_mut() {
                reference.argument_starts = args.iter().map(|a| a.start).collect();
            }
        }
        self.expressions(args);
    }

    /// A numeric expression or term.
    fn expression(&mut self, node: &Node) {
        let Some(children) = node.children() else {
            return self.term(node);
        };
        match children.split_first() {
            Some((head, args)) if matches!(head.atom(), Some("+" | "-" | "*" | "/")) => {
                self.expressions(args);
            }
            Some((head, _)) if head.is("total-time") || head.is("is-violated") => {}
            Some((head, args)) if head.atom().is_some() => self.application(head, args),
            _ => self.expressions(children),
        }
    }

    /// A variable, constant, object, number or parameterless function.
    fn term(&mut self, node: &Node) {
        let Some(atom) = node.atom() else {
            return;
        };
        if atom.starts_with('?') {
            let binding = self
                .scope
                .iter()
                .rev()
                .copied()
                .find(|&i| self.out.definitions[i].name.eq_ignore_ascii_case(atom));
            self.out.variables.push(VariableUse {
                name: atom.to_string(),
                span: node.start..node.end,
                binding,
            });
        } else if !is_number(atom) && !atom.starts_with(':') && !is_builtin_term(atom) {
            self.refer(Expect::Term, node, None);
        }
    }
}

/// Whether `atom` is a term PDDL defines itself, such as `total-time`.
fn is_builtin_term(atom: &str) -> bool {
    [
        "total-time",
        "#t",
        "minimize",
        "maximize",
        "start",
        "end",
        "all",
    ]
    .iter()
    .any(|w| atom.eq_ignore_ascii_case(w))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    const DOMAIN: &str = "(define (domain d)
  (:types truck - vehicle vehicle place)
  (:constants depot - place)
  (:predicates (at ?v - vehicle ?p - place) (ready))
  (:functions (fuel ?v - vehicle) - number)
  (:action drive
    :parameters (?t - truck ?from ?to - place)
    :precondition (and (at ?t ?from) (> (fuel ?t) 0))
    :effect (and (not (at ?t ?from)) (at ?t ?to) (decrease (fuel ?t) 1)
                 (forall (?x - place) (when (at ?t ?x) (ready))))))";

    fn analyze(text: &str) -> Analysis {
        Analysis::new(text, &syntax::read(text).unwrap())
    }

    fn defs(analysis: &Analysis, kind: Kind) -> Vec<&str> {
        analysis
            .definitions
            .iter()
            .filter(|d| d.kind == kind)
            .map(|d| d.name.as_str())
            .collect()
    }

    #[test]
    fn finds_domain_definitions() {
        let a = analyze(DOMAIN);
        check!(a.contexts.len() == 1);
        check!(a.contexts[0].domain.as_deref() == Some("d"));
        check!(defs(&a, Kind::Type) == ["truck", "vehicle", "place"]);
        check!(defs(&a, Kind::Constant) == ["depot"]);
        check!(defs(&a, Kind::Predicate) == ["at", "ready"]);
        check!(defs(&a, Kind::Function) == ["fuel"]);
        check!(defs(&a, Kind::Action) == ["drive"]);
        let at = a.definitions.iter().find(|d| d.name == "at").unwrap();
        check!(at.arity() == Some(2));
        check!(at.detail == "(at ?v - vehicle ?p - place)");
        let drive = a.definitions.iter().find(|d| d.name == "drive").unwrap();
        check!(
            drive.detail == "(:action drive :parameters (?t - truck ?from - place ?to - place))"
        );
        let fuel = a.definitions.iter().find(|d| d.name == "fuel").unwrap();
        check!(fuel.detail == "(fuel ?v - vehicle) - number");
    }

    #[test]
    fn finds_references_with_their_argument_count() {
        let a = analyze(DOMAIN);
        let heads: Vec<_> = a
            .references
            .iter()
            .filter(|r| r.expect == Expect::PredicateOrFunction)
            .map(|r| (r.name.as_str(), r.args.unwrap()))
            .collect();
        check!(
            heads
                == [
                    ("at", 2),
                    ("fuel", 1),
                    ("at", 2),
                    ("at", 2),
                    ("fuel", 1),
                    ("at", 2),
                    ("ready", 0)
                ]
        );
        let types: Vec<_> = a
            .references
            .iter()
            .filter(|r| r.expect == Expect::Type)
            .map(|r| r.name.as_str())
            .collect();
        check!(
            types == ["vehicle", "place", "vehicle", "place", "vehicle", "truck", "place", "place"]
        );
    }

    #[test]
    fn binds_variables_to_the_innermost_declaration() {
        let a = analyze(DOMAIN);
        check!(a.variables.iter().all(|v| v.binding.is_some()));
        let x = a.variables.iter().find(|v| v.name == "?x").unwrap();
        let binding = &a.definitions[x.binding.unwrap()];
        check!(binding.detail == "?x - place");
        check!(binding
            .scope
            .as_ref()
            .is_some_and(|s| DOMAIN[s.clone()].starts_with("(forall")));
        let unbound = analyze("(define (domain d) (:action a :parameters () :effect (p ?y)))");
        check!(unbound.variables[0].binding.is_none());
    }

    #[test]
    fn finds_problem_names() {
        let a = analyze(
            "(define (problem p) (:domain d) (:objects t1 - truck x)
              (:init (at t1 depot) (= (fuel t1) 5) (at 10 (ready)))
              (:goal (and (at t1 x) (preference p1 (ready))))
              (:metric minimize (+ (total-time) (is-violated p1))))",
        );
        check!(a.contexts[0].kind == Kind::Problem);
        check!(a.contexts[0].domain.as_deref() == Some("d"));
        check!(defs(&a, Kind::Object) == ["t1", "x"]);
        let refs: Vec<_> = a
            .references
            .iter()
            .map(|r| (r.expect, r.name.as_str()))
            .collect();
        check!(
            refs == [
                (Expect::Domain, "d"),
                (Expect::Type, "truck"),
                (Expect::PredicateOrFunction, "at"),
                (Expect::Term, "t1"),
                (Expect::Term, "depot"),
                (Expect::PredicateOrFunction, "fuel"),
                (Expect::Term, "t1"),
                (Expect::PredicateOrFunction, "ready"),
                (Expect::PredicateOrFunction, "at"),
                (Expect::Term, "t1"),
                (Expect::Term, "x"),
                (Expect::PredicateOrFunction, "ready"),
            ]
        );
    }

    #[test]
    fn handles_less_common_and_malformed_forms() {
        let a = analyze(
            "(define (domain odd)
  (:types car - (either vehicle thing) vehicle thing)
  (:constants (bad) c1)
  (:predicates (p ?x) (q) atom)
  (:derived (q ?y - car) (exists (?z) (and (p ?y) (p ?z))))
  (:action a :parameters (?x) :effect (and ((p ?x)) (increase total-cost (+ ((f)) 1))))
  stray)
(define (domain))
(define (problem nameless-section) (:domain odd) (:objects o) (:metric minimize (total-time)))
(define)
atom
(other)",
        );
        check!(defs(&a, Kind::Type) == ["car", "vehicle", "thing"]);
        let either: Vec<_> = a
            .references
            .iter()
            .filter(|r| r.expect == Expect::Type)
            .map(|r| r.name.as_str())
            .collect();
        check!(either == ["vehicle", "thing", "car"]);
        // `:derived` refers to its predicate and binds its parameters.
        check!(a
            .references
            .iter()
            .any(|r| r.name == "q" && r.args == Some(1)));
        check!(a.variables.iter().all(|v| v.binding.is_some()));
        // A parameterless function used as a term, and nested lists without a head.
        check!(a
            .references
            .iter()
            .any(|r| r.name == "total-cost" && r.expect == Expect::Term));
        check!(a.references.iter().any(|r| r.name == "f"));
        // A nameless domain still counts as a context; the other forms are skipped.
        check!(a.contexts.len() == 3);
        check!(a.contexts[1].name.is_empty());
        check!(Kind::Function.label() == "function");
        check!(Expect::Domain.kinds() == [Kind::Domain]);
    }

    #[test]
    fn finds_the_occurrence_at_an_offset() {
        let a = analyze(DOMAIN);
        let offset = DOMAIN.find("(ready))))))").unwrap() + 2;
        let Some(Occurrence::Reference(i)) = a.occurrence_at(offset) else {
            panic!("no reference at {offset}");
        };
        check!(a.references[i].name == "ready");
    }
}
