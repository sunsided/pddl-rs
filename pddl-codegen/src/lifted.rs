//! Level B1: typed planning code for a lifted domain.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::model::{ActionInfo, Arg, Comparison, Cond, Effect, Model, NumExpr, NumericOp};
use crate::{Bindings, Error};

/// Rust keywords, which can't be identifiers.
const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
    "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "try", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield",
];

/// A PDDL name as a `snake_case` Rust identifier: `on-table` is `on_table`,
/// `type` is `type_`.
fn snake(name: &str) -> String {
    let mut s: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if s.is_empty() || s.starts_with(|c: char| c.is_ascii_digit()) {
        s.insert(0, '_');
    }
    if KEYWORDS.contains(&s.as_str()) {
        s.push('_');
    }
    s
}

/// A PDDL name as a `CamelCase` Rust identifier: `pick-up` is `PickUp`.
fn camel(name: &str) -> String {
    let mut s: String = name
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let first = chars.next().expect("not empty").to_ascii_uppercase();
            std::iter::once(first)
                .chain(chars.map(|c| c.to_ascii_lowercase()))
                .collect::<String>()
        })
        .collect();
    if s.is_empty() || s.starts_with(|c: char| c.is_ascii_digit()) {
        s.insert(0, 'T');
    }
    if s == "Self" {
        s.push('_');
    }
    s
}

/// Makes identifiers for `names` with `case`, failing if two PDDL names map
/// to the same identifier.
fn idents(kind: &str, names: &[&str], case: fn(&str) -> String) -> Result<Vec<Ident>, Error> {
    let mut seen = std::collections::HashMap::new();
    names
        .iter()
        .map(|name| {
            let ident = case(name);
            if let Some(other) = seen.insert(ident.clone(), *name) {
                return Err(Error::new(format!(
                    "the {kind} names `{other}` and `{name}` both map to the Rust name `{ident}`; rename one"
                )));
            }
            Ok(Ident::new(&ident, Span::call_site()))
        })
        .collect()
}

/// The identifiers of the generated items.
struct Names {
    /// One id type per PDDL type, in `types::`.
    types: Vec<Ident>,
    /// One `State` or `Statics` field per predicate.
    predicates: Vec<Ident>,
    /// One `State` or `Statics` field per function.
    functions: Vec<Ident>,
    /// One `Action` variant per action.
    actions: Vec<Ident>,
    /// The fields of each action variant, one per parameter.
    params: Vec<Vec<Ident>>,
}

impl Names {
    /// Names every item, failing on collisions.
    fn new(model: &Model) -> Result<Self, Error> {
        // Predicates and functions share the fields of `State` and `Statics`.
        let mut fields = idents(
            "predicate and function",
            &model
                .predicates
                .iter()
                .map(|p| p.name.as_str())
                .chain(model.functions.iter().map(|f| f.name.as_str()))
                .collect::<Vec<_>>(),
            snake,
        )?;
        let functions = fields.split_off(model.predicates.len());
        Ok(Self {
            functions,
            types: idents(
                "type",
                &model
                    .types
                    .iter()
                    .map(|t| t.name.as_str())
                    .collect::<Vec<_>>(),
                camel,
            )?,
            predicates: fields,
            actions: idents(
                "action",
                &model
                    .actions
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>(),
                camel,
            )?,
            params: model
                .actions
                .iter()
                .map(|a| {
                    let params: Vec<&str> =
                        a.slots[..a.arity].iter().map(|s| s.name.as_str()).collect();
                    idents("parameter", &params, snake).map_err(|e| e.context(&a.name))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

/// The generator for one domain.
struct Gen<'a> {
    /// The domain.
    model: &'a Model,
    /// The identifiers.
    names: Names,
    /// The Rust type each PDDL type is bound to, if any.
    bound: Vec<Option<TokenStream>>,
    /// The Rust function each predicate is attached to, if any.
    attached_predicates: Vec<Option<TokenStream>>,
    /// The Rust function each numeric function is attached to, if any.
    attached_functions: Vec<Option<TokenStream>>,
}

/// The variable an `Action` match binds parameter `i`'s typed id to; the `__`
/// keeps it apart from the PDDL names and the generated locals.
fn param(i: usize) -> Ident {
    format_ident!("__p{}", i)
}

/// The pattern of an `Action` variant that binds each field `f` to [`param`].
fn pattern(variant: &Ident, fields: &[Ident]) -> TokenStream {
    let params = (0..fields.len()).map(param);
    quote!(Action::#variant { #(#fields: #params),* })
}

/// The variable holding slot `i`'s object number.
fn var(i: usize) -> Ident {
    format_ident!("v{}", i)
}

/// The `Task::new` parameter with the values of the bound type `name`; the
/// `__` keeps it apart from `problem` and the locals of `Task::new`.
fn bound_param(name: &str) -> Ident {
    format_ident!("__{}", snake(name))
}

/// Whether a binding's `name` names the PDDL name `pddl`: exactly, or with
/// `_` standing for `-`, which Rust identifiers can't hold.
fn names_match(pddl: &str, name: &str) -> bool {
    let name = crate::model::lc(name);
    pddl == name || pddl == name.replace('_', "-")
}

/// Generates the items of level B1 for `model`, with `bindings`.
pub fn generate(model: &Model, bindings: &Bindings) -> Result<TokenStream, Error> {
    let mut bound = vec![None; model.types.len()];
    for (name, path) in &bindings.types {
        let t = model
            .types
            .iter()
            .position(|t| names_match(&t.name, name))
            .ok_or_else(|| Error::new(format!("can't bind `{name}`: no such type")))?;
        bound[t] = Some(path.clone());
    }
    let mut attached_predicates = vec![None; model.predicates.len()];
    let mut attached_functions = vec![None; model.functions.len()];
    for (name, path) in &bindings.attachments {
        let name = crate::model::lc(name);
        if let Some(p) = model
            .predicates
            .iter()
            .position(|p| names_match(&p.name, &name))
        {
            if !model.predicates[p].is_static {
                return Err(Error::new(format!(
                    "can't attach `{name}`: actions or rules change it, and attachments must be static"
                )));
            }
            attached_predicates[p] = Some(path.clone());
        } else if let Some(f) = model
            .functions
            .iter()
            .position(|f| names_match(&f.name, &name))
        {
            if !model.functions[f].is_static {
                return Err(Error::new(format!(
                    "can't attach `{name}`: actions change it, and attachments must be static"
                )));
            }
            attached_functions[f] = Some(path.clone());
        } else {
            return Err(Error::new(format!(
                "can't attach `{name}`: no such predicate or function"
            )));
        }
    }
    let generator = Gen {
        model,
        names: Names::new(model)?,
        bound,
        attached_predicates,
        attached_functions,
    };
    Ok(generator.items())
}

impl Gen<'_> {
    /// The id type of type `t`, as a path.
    fn ty(&self, t: usize) -> TokenStream {
        let ident = &self.names.types[t];
        quote!(types::#ident)
    }

    /// The storage type of predicate `p`.
    fn storage_type(&self, p: usize) -> TokenStream {
        match self.model.predicates[p].args.as_slice() {
            [] => quote!(bool),
            [a] => {
                let a = self.ty(*a);
                quote!(::pddl::runtime::BitSet<#a>)
            }
            [a, b] => {
                let (a, b) = (self.ty(*a), self.ty(*b));
                quote!(::pddl::runtime::BitRelation<#a, #b>)
            }
            _ => quote!(::pddl::runtime::BitTable),
        }
    }

    /// Empty storage for predicate `p`; `count(t)` must be in scope.
    fn storage_new(&self, p: usize) -> TokenStream {
        let args = &self.model.predicates[p].args;
        match args.as_slice() {
            [] => quote!(false),
            [a] => quote!(::pddl::runtime::BitSet::new(count(#a))),
            [a, b] => quote!(::pddl::runtime::BitRelation::new(count(#a), count(#b))),
            _ => quote!(::pddl::runtime::BitTable::new(&[#(count(#args)),*])),
        }
    }

    /// The local id of the object numbered `global` among type `t`'s objects.
    fn local(&self, t: usize, global: &TokenStream, starts: &TokenStream) -> TokenStream {
        let ty = self.ty(t);
        quote!(#ty(#global - #starts[#t]))
    }

    /// Reads (`set` is `None`) or writes the fact `p(args)` in `store`.
    fn fact(
        &self,
        store: &TokenStream,
        p: usize,
        args: &[TokenStream],
        starts: &TokenStream,
        set: Option<bool>,
    ) -> TokenStream {
        let field = &self.names.predicates[p];
        let types = &self.model.predicates[p].args;
        let locals: Vec<TokenStream> = types
            .iter()
            .zip(args)
            .map(|(&t, g)| self.local(t, g, starts))
            .collect();
        let indices: Vec<TokenStream> = types
            .iter()
            .zip(args)
            .map(|(t, g)| quote!((#g - #starts[#t]) as usize))
            .collect();
        match (types.len(), set) {
            (0, None) => quote!(#store.#field),
            (0, Some(value)) => quote!(#store.#field = #value),
            (1 | 2, None) => quote!(#store.#field.contains(#(#locals),*)),
            (1 | 2, Some(value)) => quote!(#store.#field.set(#(#locals,)* #value)),
            (_, None) => quote!(#store.#field.contains(&[#(#indices),*])),
            (_, Some(value)) => quote!(#store.#field.set(&[#(#indices),*], #value)),
        }
    }

    /// The object number of an argument, and its type.
    fn arg(&self, action: &ActionInfo, arg: Arg) -> (TokenStream, usize) {
        match arg {
            Arg::Var(slot) => {
                let v = var(slot);
                (quote!(#v), action.slots[slot].ty)
            }
            Arg::Const(c) => (quote!(self.constants[#c]), self.model.constants[c].1),
        }
    }

    /// Checks that the arguments fit predicate `p`'s argument types where their
    /// own types don't guarantee it; `true` if nothing needs checking.
    fn type_guard(&self, p: usize, args: &[(TokenStream, usize)]) -> TokenStream {
        self.guard(&self.model.predicates[p].args, args)
    }

    /// Checks that the arguments fit the types `want` where their own types
    /// don't guarantee it; `true` if nothing needs checking.
    fn guard(&self, want: &[usize], args: &[(TokenStream, usize)]) -> TokenStream {
        let checks: Vec<TokenStream> = want
            .iter()
            .zip(args)
            .filter(|(&want, (_, have))| !self.model.is_subtype(*have, want))
            .map(|(want, (g, _))| quote!((self.starts[#want]..self.ends[#want]).contains(&#g)))
            .collect();
        if checks.is_empty() {
            quote!(true)
        } else {
            quote!((#(#checks)&&*))
        }
    }

    /// The value of function `func` at `args` in `state` (or the static
    /// facts), as an `Option<f64>`: `None` where undefined or where an argument
    /// has the wrong type.
    fn fluent(
        &self,
        action: &ActionInfo,
        func: usize,
        args: &[Arg],
        state: &TokenStream,
    ) -> TokenStream {
        let info = &self.model.functions[func];
        if let Some(path) = &self.attached_functions[func] {
            let args: Vec<_> = args.iter().map(|&a| self.arg(action, a)).collect();
            let guard = self.guard(&info.args, &args);
            let call = self.call(path, &info.args, &args, &quote!(self.starts));
            return quote!((if #guard { ::core::option::Option::Some(#call).filter(|v: &f64| v.is_finite()) } else { ::core::option::Option::None }));
        }
        let field = &self.names.functions[func];
        let store = if info.is_static {
            quote!(self.statics)
        } else {
            state.clone()
        };
        let args: Vec<_> = args.iter().map(|&a| self.arg(action, a)).collect();
        let guard = self.guard(&info.args, &args);
        let index = Self::index(&info.args, &args);
        quote!((if #guard { #store.#field.get(&#index) } else { ::core::option::Option::None }))
    }

    /// A call of an attachment with the bound values and the arguments as
    /// typed ids of the types `types`.
    fn call(
        &self,
        path: &TokenStream,
        types: &[usize],
        args: &[(TokenStream, usize)],
        starts: &TokenStream,
    ) -> TokenStream {
        let ids = types
            .iter()
            .zip(args)
            .map(|(&t, (g, _))| self.local(t, g, starts));
        quote!(#path(&self.bound, #(#ids),*))
    }

    /// The index array of a `Values` entry for arguments of the types `types`.
    fn index(types: &[usize], args: &[(TokenStream, usize)]) -> TokenStream {
        let parts = types
            .iter()
            .zip(args)
            .map(|(t, (g, _))| quote!((#g - self.starts[#t]) as usize));
        quote!([#(#parts),*])
    }

    /// A numeric expression over `state`, as an `Option<f64>`: `None` where
    /// undefined, as is every non-finite value.
    fn num(&self, action: &ActionInfo, expr: &NumExpr, state: &TokenStream) -> TokenStream {
        let pair =
            |a: &NumExpr, b: &NumExpr| (self.num(action, a, state), self.num(action, b, state));
        match expr {
            NumExpr::Number(n) if !n.is_finite() => quote!(::core::option::Option::<f64>::None),
            NumExpr::Number(n) => {
                let n = proc_macro2::Literal::f64_suffixed(*n);
                quote!(::core::option::Option::Some(#n))
            }
            NumExpr::Fluent { func, args } => self.fluent(action, *func, args, state),
            NumExpr::Add(a, b) => {
                let (a, b) = pair(a, b);
                quote!(#a.zip(#b).map(|(a, b)| a + b).filter(|v| v.is_finite()))
            }
            NumExpr::Sub(a, b) => {
                let (a, b) = pair(a, b);
                quote!(#a.zip(#b).map(|(a, b)| a - b).filter(|v| v.is_finite()))
            }
            NumExpr::Mul(a, b) => {
                let (a, b) = pair(a, b);
                quote!(#a.zip(#b).map(|(a, b)| a * b).filter(|v| v.is_finite()))
            }
            NumExpr::Div(a, b) => {
                let (a, b) = pair(a, b);
                quote!(#a.zip(#b).and_then(|(a, b)| (b != 0.0).then(|| a / b)).filter(|v| v.is_finite()))
            }
            NumExpr::Neg(a) => {
                let a = self.num(action, a, state);
                quote!(#a.map(|a| -a))
            }
        }
    }

    /// A condition that fails if a comparison in it reads an undefined value,
    /// also under a negation. `undefined` must be in scope.
    fn checked(&self, action: &ActionInfo, cond: &Cond, state: &TokenStream) -> TokenStream {
        let cond = self.cond(action, cond, state);
        quote!({
            undefined.set(false);
            let holds = #cond;
            holds && !undefined.get()
        })
    }

    /// Where predicate `p`'s facts are: the state, or the static facts.
    fn store(&self, p: usize, state: &TokenStream) -> TokenStream {
        if self.model.predicates[p].is_static {
            quote!(self.statics)
        } else {
            state.clone()
        }
    }

    /// A condition as a boolean expression over `state`.
    fn cond(&self, action: &ActionInfo, cond: &Cond, state: &TokenStream) -> TokenStream {
        match cond {
            Cond::Atom { pred, args } => {
                let args: Vec<_> = args.iter().map(|&a| self.arg(action, a)).collect();
                let guard = self.type_guard(*pred, &args);
                if let Some(path) = &self.attached_predicates[*pred] {
                    let call = self.call(
                        path,
                        &self.model.predicates[*pred].args,
                        &args,
                        &quote!(self.starts),
                    );
                    return quote!((#guard && #call));
                }
                let globals: Vec<TokenStream> = args.into_iter().map(|(g, _)| g).collect();
                let fact = self.fact(
                    &self.store(*pred, state),
                    *pred,
                    &globals,
                    &quote!(self.starts),
                    None,
                );
                quote!((#guard && #fact))
            }
            Cond::Eq(a, b) => {
                let (a, _) = self.arg(action, *a);
                let (b, _) = self.arg(action, *b);
                quote!((#a == #b))
            }
            Cond::Not(inner) => {
                let inner = self.cond(action, inner, state);
                quote!((!#inner))
            }
            // A part with a comparison must run even when the others decide
            // the result: it may read an undefined value, which fails it all.
            Cond::And(parts) => {
                let parts: Vec<_> = parts.iter().map(|p| self.cond(action, p, state)).collect();
                match (parts.is_empty(), Self::compares(cond)) {
                    (true, _) => quote!(true),
                    (false, false) => quote!((#(#parts)&&*)),
                    (false, true) => quote!((#(#parts)&*)),
                }
            }
            Cond::Or(parts) => {
                let parts: Vec<_> = parts.iter().map(|p| self.cond(action, p, state)).collect();
                match (parts.is_empty(), Self::compares(cond)) {
                    (true, _) => quote!(false),
                    (false, false) => quote!((#(#parts)||*)),
                    (false, true) => quote!((#(#parts)|*)),
                }
            }
            Cond::Exists(slots, body) | Cond::Forall(slots, body) => {
                self.quantified(action, matches!(cond, Cond::Exists(..)), slots, body, state)
            }
            Cond::Compare(op, a, b) => {
                let (a, b) = (self.num(action, a, state), self.num(action, b, state));
                let op = match op {
                    Comparison::Less => quote!(<),
                    Comparison::LessOrEqual => quote!(<=),
                    Comparison::Equal => quote!(==),
                    Comparison::GreaterOrEqual => quote!(>=),
                    Comparison::Greater => quote!(>),
                };
                quote!(match (#a, #b) {
                    (::core::option::Option::Some(a), ::core::option::Option::Some(b)) => a #op b,
                    _ => {
                        undefined.set(true);
                        false
                    }
                })
            }
        }
    }

    /// An `exists` (`exists` is `true`) or `forall` over `slots`: one loop
    /// per slot, which visits every object if `body` has a comparison.
    fn quantified(
        &self,
        action: &ActionInfo,
        exists: bool,
        slots: &[usize],
        body: &Cond,
        state: &TokenStream,
    ) -> TokenStream {
        let every = Self::compares(body);
        let mut expr = self.cond(action, body, state);
        for &slot in slots.iter().rev() {
            let v = var(slot);
            let ty = action.slots[slot].ty;
            let member = Self::member(action, slot);
            let objects = quote!((self.starts[#ty]..self.ends[#ty]));
            expr = match (exists, every) {
                (true, false) => quote!(#objects.any(|#v| #member && #expr)),
                (false, false) => quote!(#objects.all(|#v| !#member || #expr)),
                (true, true) => quote!(#objects.fold(false, |any, #v| any | (#member && #expr))),
                (false, true) => quote!(#objects.fold(true, |all, #v| all & (!#member || #expr))),
            };
        }
        expr
    }

    /// Whether a condition has a numeric comparison.
    fn compares(cond: &Cond) -> bool {
        match cond {
            Cond::Atom { .. } | Cond::Eq(..) => false,
            Cond::Compare(..) => true,
            Cond::Not(inner) | Cond::Exists(_, inner) | Cond::Forall(_, inner) => {
                Self::compares(inner)
            }
            Cond::And(parts) | Cond::Or(parts) => parts.iter().any(Self::compares),
        }
    }

    /// Whether slot `slot`'s object belongs to its `either` type; `true` for
    /// a plain type, whose range the loop already keeps to.
    fn member(action: &ActionInfo, slot: usize) -> TokenStream {
        let v = var(slot);
        action.slots[slot].either.as_ref().map_or_else(
            || quote!(true),
            |members| quote!((#((self.starts[#members]..self.ends[#members]).contains(&#v))||*)),
        )
    }

    /// The highest parameter slot below `arity` that a condition reads, if any;
    /// quantified variables are bound inside the condition.
    fn last_slot(cond: &Cond, arity: usize) -> Option<usize> {
        let of = |args: &[Arg]| {
            args.iter()
                .filter_map(|a| match a {
                    Arg::Var(slot) if *slot < arity => Some(*slot),
                    _ => None,
                })
                .max()
        };
        match cond {
            Cond::Atom { args, .. } => of(args),
            Cond::Eq(a, b) => of(&[*a, *b]),
            Cond::Not(inner) | Cond::Exists(_, inner) | Cond::Forall(_, inner) => {
                Self::last_slot(inner, arity)
            }
            Cond::And(parts) | Cond::Or(parts) => {
                parts.iter().filter_map(|p| Self::last_slot(p, arity)).max()
            }
            Cond::Compare(_, a, b) => Self::num_slot(a, arity).max(Self::num_slot(b, arity)),
        }
    }

    /// The highest parameter slot below `arity` that an expression reads.
    fn num_slot(expr: &NumExpr, arity: usize) -> Option<usize> {
        match expr {
            NumExpr::Number(_) => None,
            NumExpr::Fluent { args, .. } => args
                .iter()
                .filter_map(|a| match a {
                    Arg::Var(slot) if *slot < arity => Some(*slot),
                    _ => None,
                })
                .max(),
            NumExpr::Add(a, b) | NumExpr::Sub(a, b) | NumExpr::Mul(a, b) | NumExpr::Div(a, b) => {
                Self::num_slot(a, arity).max(Self::num_slot(b, arity))
            }
            NumExpr::Neg(a) => Self::num_slot(a, arity),
        }
    }

    /// The action's cost under the bound parameters, as an `Option<f64>`;
    /// `Some(0.0)` without cost effects. Costs read only static functions.
    fn cost_expr(&self, action: &ActionInfo) -> TokenStream {
        let terms = action
            .cost
            .iter()
            .map(|t| self.num(action, t, &quote!(self.statics)));
        quote!(::core::option::Option::Some(0.0f64) #(.zip(#terms).map(|(a, b)| a + b))*)
    }

    /// The name of action `a`'s apply method.
    fn apply_fn(a: usize) -> Ident {
        format_ident!("apply_{}", a)
    }

    /// Successor generation for action `a`: nested loops over its parameters,
    /// each precondition checked as soon as its last variable is bound.
    fn successors(&self, a: usize) -> TokenStream {
        let action = &self.model.actions[a];
        let state = quote!(state);
        let mut checks: Vec<Vec<TokenStream>> = vec![Vec::new(); action.arity + 1];
        for cond in &action.pre {
            let depth = Self::last_slot(cond, action.arity).map_or(0, |s| s + 1);
            checks[depth].push(self.checked(action, cond, &state));
        }
        let all = |checks: &[TokenStream]| {
            if checks.is_empty() {
                quote!(true)
            } else {
                quote!(#(#checks)&&*)
            }
        };

        let variant = &self.names.actions[a];
        let fields = &self.names.params[a];
        let values: Vec<TokenStream> = action.slots[..action.arity]
            .iter()
            .enumerate()
            .map(|(i, slot)| {
                let v = var(i);
                self.local(slot.ty, &quote!(#v), &quote!(self.starts))
            })
            .collect();
        let vars: Vec<Ident> = (0..action.arity).map(var).collect();
        let apply = Self::apply_fn(a);
        let cost = if action.cost.is_empty() {
            quote!(true)
        } else {
            let cost = self.cost_expr(action);
            quote!((!self.use_costs || #cost.is_some()))
        };
        // An action whose effects or cost read an undefined value is not applicable.
        let mut body = quote! {
            if let ::core::option::Option::Some(next) = self.#apply(state, #(#vars),*) {
                if #cost {
                    out.push((Action::#variant { #(#fields: #values),* }, next));
                }
            }
        };
        for depth in (0..action.arity).rev() {
            let v = var(depth);
            let slot = &action.slots[depth];
            let ty = slot.ty;
            let member = Self::member(action, depth);
            let check = all(&checks[depth + 1]);
            body = quote! {
                for #v in self.starts[#ty]..self.ends[#ty] {
                    if !(#member && #check) {
                        continue;
                    }
                    #body
                }
            };
        }
        let check = all(&checks[0]);
        quote! {
            if #check {
                #body
            }
        }
    }

    /// The code for the fact effects of one kind (`add` or delete) among
    /// `effects`, with conditions read from `state` and changes made to `next`.
    fn effects(&self, action: &ActionInfo, effects: &[Effect], add: bool) -> TokenStream {
        let next = quote!(next);
        let state = quote!(state);
        let parts = effects.iter().filter(|e| {
            let mut found = false;
            e.facts(&mut |_, a| found |= a == add);
            found
        });
        let code = parts.map(|effect| match effect {
            Effect::Fact {
                pred,
                args,
                add: kind,
            } => {
                debug_assert_eq!(*kind, add);
                let args: Vec<_> = args.iter().map(|&arg| self.arg(action, arg)).collect();
                let guard = self.type_guard(*pred, &args);
                let globals: Vec<TokenStream> = args.into_iter().map(|(g, _)| g).collect();
                let set = self.fact(&next, *pred, &globals, &quote!(self.starts), Some(add));
                quote!(if #guard { #set; })
            }
            Effect::Forall(slots, body) => {
                let mut code = self.effects(action, body, add);
                for &slot in slots.iter().rev() {
                    let v = var(slot);
                    let ty = action.slots[slot].ty;
                    let member = Self::member(action, slot);
                    code = quote! {
                        for #v in self.starts[#ty]..self.ends[#ty] {
                            if #member {
                                #code
                            }
                        }
                    };
                }
                code
            }
            Effect::When(cond, body) => {
                let cond = self.checked(action, cond, &state);
                let body = self.effects(action, body, add);
                quote!(if #cond { #body })
            }
            Effect::Assign { .. } => unreachable!("filtered out: no fact effects"),
        });
        quote!(#(#code)*)
    }

    /// The code for the numeric effects among `effects`: values read `state`,
    /// changes go to `next`, so `increase` and `decrease` add up. Returns
    /// `None` from the apply method where a value is undefined or a result is
    /// not finite.
    fn assigns(&self, action: &ActionInfo, effects: &[Effect]) -> TokenStream {
        let state = quote!(state);
        let code = effects
            .iter()
            .filter(|e| Effect::has_assign(std::slice::from_ref(e)))
            .map(|effect| match effect {
                Effect::Assign {
                    op,
                    func,
                    args,
                    value,
                } => {
                    let info = &self.model.functions[*func];
                    let field = &self.names.functions[*func];
                    let args: Vec<_> = args.iter().map(|&a| self.arg(action, a)).collect();
                    let guard = self.guard(&info.args, &args);
                    let index = Self::index(&info.args, &args);
                    let value = self.num(action, value, &state);
                    let new = match op {
                        NumericOp::Assign => quote!(value),
                        NumericOp::Increase => quote!(next.#field.get(&index)? + value),
                        NumericOp::Decrease => quote!(next.#field.get(&index)? - value),
                        NumericOp::ScaleUp => quote!(next.#field.get(&index)? * value),
                        NumericOp::ScaleDown => quote!({
                            if value == 0.0 {
                                return ::core::option::Option::None;
                            }
                            next.#field.get(&index)? / value
                        }),
                    };
                    quote!(if #guard {
                        let index = #index;
                        let value = #value?;
                        let new = ::core::option::Option::Some(#new).filter(|v: &f64| v.is_finite())?;
                        next.#field.set(&index, ::core::option::Option::Some(new));
                    })
                }
                Effect::Forall(slots, body) => {
                    let mut code = self.assigns(action, body);
                    for &slot in slots.iter().rev() {
                        let v = var(slot);
                        let ty = action.slots[slot].ty;
                        let member = Self::member(action, slot);
                        code = quote! {
                            for #v in self.starts[#ty]..self.ends[#ty] {
                                if #member {
                                    #code
                                }
                            }
                        };
                    }
                    code
                }
                Effect::When(cond, body) => {
                    let cond = self.checked(action, cond, &state);
                    let body = self.assigns(action, body);
                    quote!(if #cond { #body })
                }
                Effect::Fact { .. } => unreachable!("filtered out: no numeric effects"),
            });
        quote!(#(#code)*)
    }

    /// The apply method of action `a`: deletes, then adds, so an add wins.
    /// Conditions read the state before the action.
    fn apply(&self, a: usize) -> TokenStream {
        let action = &self.model.actions[a];
        let name = Self::apply_fn(a);
        let vars: Vec<Ident> = (0..action.arity).map(var).collect();
        let assigns = self.assigns(action, &action.effects);
        let deletes = self.effects(action, &action.effects, false);
        let adds = self.effects(action, &action.effects, true);
        let derive = if self.model.strata.is_empty() {
            quote!()
        } else {
            quote!(self.derive(&mut next);)
        };
        let doc = format!(
            "Applies `{}` to `state`; `None` if an effect reads an undefined value.",
            action.name
        );
        quote! {
            #[doc = #doc]
            fn #name(&self, state: &State, #(#vars: u32),*) -> ::core::option::Option<State> {
                let undefined = ::core::cell::Cell::new(false);
                let mut next = state.clone();
                #assigns
                #deletes
                #adds
                #derive
                ::core::option::Option::Some(next)
            }
        }
    }

    /// The method that recomputes the derived predicates of `state`: clear
    /// them, then fire each stratum's rules until nothing changes.
    fn derive(&self) -> TokenStream {
        let state = quote!(state);
        let clear = self
            .model
            .predicates
            .iter()
            .enumerate()
            .filter(|(_, p)| p.derived)
            .map(|(i, p)| {
                let field = &self.names.predicates[i];
                if p.args.is_empty() {
                    quote!(state.#field = false;)
                } else {
                    quote!(state.#field.clear();)
                }
            });
        let strata = self.model.strata.iter().map(|rules| {
            let rules = rules.iter().map(|rule| {
                let pred = self
                    .model
                    .predicates
                    .iter()
                    .position(|p| p.name == rule.name)
                    .expect("a rule is named after its predicate");
                let args: Vec<_> = (0..rule.arity)
                    .map(|i| self.arg(rule, Arg::Var(i)))
                    .collect();
                let guard = self.type_guard(pred, &args);
                let globals: Vec<TokenStream> = args.into_iter().map(|(g, _)| g).collect();
                let head = self.fact(&state, pred, &globals, &quote!(self.starts), None);
                let set = self.fact(&state, pred, &globals, &quote!(self.starts), Some(true));
                let body = self.checked(rule, &rule.pre[0], &state);
                let mut code = quote! {
                    if #guard && !#head && #body {
                        #set;
                        changed = true;
                    }
                };
                for slot in (0..rule.arity).rev() {
                    let v = var(slot);
                    let ty = rule.slots[slot].ty;
                    let member = Self::member(rule, slot);
                    code = quote! {
                        for #v in self.starts[#ty]..self.ends[#ty] {
                            if #member {
                                #code
                            }
                        }
                    };
                }
                code
            });
            quote! {
                loop {
                    let mut changed = false;
                    #(#rules)*
                    if !changed {
                        break;
                    }
                }
            }
        });
        quote! {
            /// Recomputes the derived predicates of `state` from its other facts.
            fn derive(&self, state: &mut State) {
                let undefined = ::core::cell::Cell::new(false);
                #(#clear)*
                #(#strata)*
            }
        }
    }

    /// The domain's [`Signature`](pddl_core::codegen) for the loader.
    fn signature(&self) -> TokenStream {
        let name = &self.model.name;
        let types = self.model.types.iter().map(|t| {
            let name = &t.name;
            let parent = t.parent.map_or_else(
                || quote!(::core::option::Option::None),
                |p| quote!(::core::option::Option::Some(#p)),
            );
            quote!(::pddl::codegen::TypeSignature { name: #name, parent: #parent })
        });
        let predicates = self.model.predicates.iter().map(|p| {
            let name = &p.name;
            let args = &p.args;
            quote!(::pddl::codegen::Symbol { name: #name, args: &[#(#args),*] })
        });
        let functions = self.model.functions.iter().map(|f| {
            let name = &f.name;
            let args = &f.args;
            quote!(::pddl::codegen::Symbol { name: #name, args: &[#(#args),*] })
        });
        let constants = self
            .model
            .constants
            .iter()
            .map(|(name, t)| quote!((#name, #t)));
        quote! {
            /// The domain, as the problem loader sees it.
            pub static SIGNATURE: ::pddl::codegen::Signature = ::pddl::codegen::Signature {
                domain: #name,
                types: &[#(#types),*],
                predicates: &[#(#predicates),*],
                functions: &[#(#functions),*],
                constants: &[#(#constants),*],
            };
        }
    }

    /// All generated items.
    #[allow(clippy::too_many_lines)] // the generated module's layout, in one place
    fn items(&self) -> TokenStream {
        let model = self.model;
        let allow = quote!(#[allow(unused_variables, unused_mut, unused_parens, clippy::all, clippy::pedantic)]);

        let type_items = model.types.iter().zip(&self.names.types).map(|(t, ident)| {
            let doc = format!(
                "An object of type `{}`, numbered among the objects of that type and its subtypes.",
                t.name
            );
            quote! {
                #[doc = #doc]
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
                pub struct #ident(pub u32);

                impl ::pddl::runtime::Id for #ident {
                    fn index(self) -> usize {
                        self.0 as usize
                    }

                    fn from_index(index: usize) -> Self {
                        Self(u32::try_from(index).expect("an id fits in a u32"))
                    }
                }
            }
        });

        let field = |static_: bool| {
            model
                .predicates
                .iter()
                .enumerate()
                .filter(move |(i, p)| {
                    p.is_static == static_ && self.attached_predicates[*i].is_none()
                })
                .map(|(i, p)| {
                    let ident = &self.names.predicates[i];
                    let ty = self.storage_type(i);
                    let doc = format!("The facts of `{}`.", p.name);
                    quote!(#[doc = #doc] pub #ident: #ty)
                })
                .collect::<Vec<_>>()
        };
        let function_field = |static_: bool| {
            model
                .functions
                .iter()
                .enumerate()
                .filter(move |(i, f)| {
                    f.is_static == static_ && self.attached_functions[*i].is_none()
                })
                .map(|(i, f)| {
                    let ident = &self.names.functions[i];
                    let doc = format!("The values of `{}`.", f.name);
                    quote!(#[doc = #doc] pub #ident: ::pddl::runtime::Values)
                })
                .collect::<Vec<_>>()
        };
        let mut state_fields = field(false);
        state_fields.extend(function_field(false));
        let mut static_fields = field(true);
        static_fields.extend(function_field(true));
        let new_fields = |static_: bool| {
            model
                .predicates
                .iter()
                .enumerate()
                .filter(move |(i, p)| {
                    p.is_static == static_ && self.attached_predicates[*i].is_none()
                })
                .map(|(i, _)| {
                    let ident = &self.names.predicates[i];
                    let new = self.storage_new(i);
                    quote!(#ident: #new)
                })
                .collect::<Vec<_>>()
        };
        let new_function = |static_: bool| {
            model
                .functions
                .iter()
                .enumerate()
                .filter(move |(i, f)| {
                    f.is_static == static_ && self.attached_functions[*i].is_none()
                })
                .map(|(i, f)| {
                    let ident = &self.names.functions[i];
                    let args = &f.args;
                    quote!(#ident: ::pddl::runtime::Values::new(&[#(count(#args)),*]))
                })
                .collect::<Vec<_>>()
        };
        let mut new_state = new_fields(false);
        new_state.extend(new_function(false));
        let mut new_statics = new_fields(true);
        new_statics.extend(new_function(true));

        let globals_of = |types: &[usize]| -> Vec<(TokenStream, usize)> {
            types
                .iter()
                .enumerate()
                .map(|(k, &t)| (quote!(args[#k]), t))
                .collect()
        };
        let set_values = model.functions.iter().enumerate().map(|(i, f)| {
            if self.attached_functions[i].is_some() {
                return quote!(#i => {});
            }
            let store = if f.is_static {
                quote!(statics)
            } else {
                quote!(state)
            };
            let field = &self.names.functions[i];
            let index = f
                .args
                .iter()
                .enumerate()
                .map(|(k, t)| quote!((args[#k] - loaded.starts[#t]) as usize));
            quote!(#i => #store.#field.set(&[#(#index),*], ::core::option::Option::Some(*value)),)
        });
        let read_value = model.functions.iter().enumerate().map(|(i, f)| {
            if let Some(path) = &self.attached_functions[i] {
                let call = self.call(path, &f.args, &globals_of(&f.args), &quote!(self.starts));
                return quote!(#i => ::core::option::Option::Some(#call),);
            }
            let store = if f.is_static {
                quote!(self.statics)
            } else {
                quote!(state)
            };
            let field = &self.names.functions[i];
            let index = f
                .args
                .iter()
                .enumerate()
                .map(|(k, t)| quote!((args[#k] - self.starts[#t]) as usize));
            quote!(#i => #store.#field.get(&[#(#index),*]),)
        });
        let mut cost_functions = Vec::new();
        for action in &model.actions {
            for term in &action.cost {
                term.functions(&mut |func| {
                    if !cost_functions.contains(&func) {
                        cost_functions.push(func);
                    }
                });
            }
        }
        cost_functions.sort_unstable();
        let cost_arms = (0..model.actions.len()).map(|a| {
            let action = &model.actions[a];
            let variant = &self.names.actions[a];
            let pattern = pattern(variant, &self.names.params[a]);
            let binds = action.slots[..action.arity]
                .iter()
                .enumerate()
                .map(|(i, slot)| {
                    let (v, p) = (var(i), param(i));
                    let t = slot.ty;
                    quote!(let #v = #p.0 + self.starts[#t];)
                });
            let cost = self.cost_expr(action);
            quote!(#pattern => {
                #(#binds)*
                // The loader checked that cost functions are non-negative integers.
                #cost.map_or(u32::MAX, |cost| cost.min(f64::from(u32::MAX)) as u32)
            })
        });

        let variants = model.actions.iter().enumerate().map(|(a, action)| {
            let variant = &self.names.actions[a];
            let fields = &self.names.params[a];
            let types = action.slots[..action.arity].iter().map(|s| self.ty(s.ty));
            let doc = format!("The action `{}`.", action.name);
            let docs = action.slots[..action.arity]
                .iter()
                .map(|s| format!("The parameter `?{}`.", s.name));
            quote!(#[doc = #doc] #variant { #(#[doc = #docs] #fields: #types),* })
        });

        let set_initial = model.predicates.iter().enumerate().map(|(i, p)| {
            // Facts of an attached predicate come from its function, not the problem.
            if self.attached_predicates[i].is_some() {
                return quote!(#i => {});
            }
            let store = if p.is_static {
                quote!(statics)
            } else {
                quote!(state)
            };
            let args: Vec<TokenStream> = (0..p.args.len()).map(|k| quote!(args[#k])).collect();
            let set = self.fact(&store, i, &args, &quote!(loaded.starts), Some(true));
            quote!(#i => { #set; })
        });
        let read_fact = model.predicates.iter().enumerate().map(|(i, p)| {
            if let Some(path) = &self.attached_predicates[i] {
                let call = self.call(path, &p.args, &globals_of(&p.args), &quote!(self.starts));
                return quote!(#i => #call,);
            }
            let store = if p.is_static {
                quote!(self.statics)
            } else {
                quote!(state)
            };
            let args: Vec<TokenStream> = (0..p.args.len()).map(|k| quote!(args[#k])).collect();
            let get = self.fact(&store, i, &args, &quote!(self.starts), None);
            quote!(#i => #get,)
        });

        let successors = (0..model.actions.len()).map(|a| self.successors(a));
        let applies = (0..model.actions.len()).map(|a| self.apply(a));
        let globals = |a: usize| {
            let action = &model.actions[a];
            action.slots[..action.arity]
                .iter()
                .enumerate()
                .map(|(i, slot)| {
                    let (p, t) = (param(i), slot.ty);
                    quote!(#p.0 + self.starts[#t])
                })
                .collect::<Vec<_>>()
        };
        let apply_arms = (0..model.actions.len()).map(|a| {
            let pattern = pattern(&self.names.actions[a], &self.names.params[a]);
            let apply = Self::apply_fn(a);
            let globals = globals(a);
            quote!(#pattern => self.#apply(state, #(#globals),*),)
        });
        let describe_arms = (0..model.actions.len()).map(|a| {
            let pattern = pattern(&self.names.actions[a], &self.names.params[a]);
            let name = &model.actions[a].name;
            let globals = globals(a);
            quote! {
                #pattern => {
                    let mut text = ::std::string::String::from(concat!("(", #name));
                    #(
                        text.push(' ');
                        text.push_str(&self.names[(#globals) as usize]);
                    )*
                    text.push(')');
                    text
                }
            }
        });
        let signature = self.signature();
        let bound_fields: Vec<TokenStream> = self
            .bound
            .iter()
            .enumerate()
            .filter_map(|(t, path)| {
                let path = path.as_ref()?;
                let field = Ident::new(&snake(&model.types[t].name), Span::call_site());
                let doc = format!(
                    "The value of each object of type `{}`, by its id.",
                    model.types[t].name
                );
                Some(quote!(#[doc = #doc] pub #field: ::std::vec::Vec<#path>))
            })
            .collect();
        let bound_params: Vec<TokenStream> = self
            .bound
            .iter()
            .enumerate()
            .filter_map(|(t, path)| {
                let path = path.as_ref()?;
                let param = bound_param(&model.types[t].name);
                Some(quote!(mut #param: impl FnMut(&str) -> #path))
            })
            .collect();
        let bound_values: Vec<TokenStream> = self.bound.iter().enumerate().filter_map(|(t, path)| {
            path.as_ref()?;
            let field = Ident::new(&snake(&model.types[t].name), Span::call_site());
            let param = bound_param(&model.types[t].name);
            Some(quote!(#field: (loaded.starts[#t]..loaded.ends[#t]).map(|g| #param(&loaded.names[g as usize])).collect()))
        }).collect();
        let bound_docs = if bound_params.is_empty() {
            quote!()
        } else {
            let names: Vec<String> = self
                .bound
                .iter()
                .enumerate()
                .filter(|(_, p)| p.is_some())
                .map(|(t, _)| format!("`{}`", model.types[t].name))
                .collect();
            let doc = format!(
                "The closures give the bound value of each object of the types {}, by name.",
                names.join(", ")
            );
            quote!(#[doc = ""] #[doc = #doc])
        };
        let derive = self.derive();
        let domain_doc = format!(
            "The planning task of domain `{}`, generated by `pddl-codegen`.",
            model.name
        );

        quote! {
            /// The id types, one per PDDL type.
            pub mod types {
                #(#type_items)*
            }

            #signature

            /// A state: the facts of every predicate that actions change.
            #[derive(Clone, PartialEq, Eq, Hash, Debug)]
            pub struct State {
                #(#state_fields,)*
            }

            /// The facts of the static predicates, which no action changes.
            #[derive(Clone, Debug)]
            pub struct Statics {
                #(#static_fields,)*
            }

            /// The values of the objects of the PDDL types bound to Rust types,
            /// indexed by id: `bound.robot[id.0 as usize]`. Attachments read them.
            #[derive(Clone, Debug)]
            pub struct Bound {
                #(#bound_fields,)*
            }

            /// A ground action, with its arguments as typed ids.
            #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
            pub enum Action {
                #(#variants,)*
            }

            #[doc = #domain_doc]
            #[derive(Clone, Debug)]
            #[allow(dead_code)]
            pub struct Task {
                /// The name of each object, by number.
                names: ::std::vec::Vec<::std::string::String>,
                /// The first object of each type.
                starts: ::std::vec::Vec<u32>,
                /// One past the last object of each type.
                ends: ::std::vec::Vec<u32>,
                /// The number of each constant.
                constants: ::std::vec::Vec<u32>,
                /// The static facts.
                statics: Statics,
                /// The values of the objects of bound types.
                bound: Bound,
                /// The initial state.
                initial: State,
                /// The goal.
                goal: ::pddl::runtime::Formula,
                /// Whether the problem minimizes `total-cost`, so actions cost what they say.
                use_costs: bool,
            }

            #allow
            impl Task {
                /// Loads a problem for this domain.
                ///
                /// # Errors
                /// A [`LoadError`](::pddl::codegen::LoadError) if the problem is for another
                /// domain, uses undeclared names, or uses an unsupported construct.
                #bound_docs
                pub fn new(problem: &::pddl::Problem, #(#bound_params),*) -> ::core::result::Result<Self, ::pddl::codegen::LoadError> {
                    let loaded = ::pddl::codegen::load(&SIGNATURE, problem)?;
                    let count = |t: usize| (loaded.ends[t] - loaded.starts[t]) as usize;
                    let mut state = State { #(#new_state,)* };
                    let mut statics = Statics { #(#new_statics,)* };
                    for (pred, args) in &loaded.facts {
                        match *pred {
                            #(#set_initial)*
                            _ => unreachable!("the loader checks predicate indices"),
                        }
                    }
                    for (func, args, value) in &loaded.values {
                        // Costs count only under the metric; otherwise any value will do.
                        if loaded.minimize_total_cost
                            && [#(#cost_functions),*].contains(func)
                            && !(*value >= 0.0 && value.fract() == 0.0 && *value <= f64::from(u32::MAX))
                        {
                            return ::core::result::Result::Err(::pddl::codegen::LoadError::InvalidCost {
                                function: SIGNATURE.functions[*func].name.to_string(),
                                value: *value,
                            });
                        }
                        match *func {
                            #(#set_values)*
                            _ => unreachable!("the loader checks function indices"),
                        }
                    }
                    let bound = Bound { #(#bound_values,)* };
                    let mut task = Self {
                        names: loaded.names,
                        starts: loaded.starts,
                        ends: loaded.ends,
                        constants: loaded.constants,
                        statics,
                        bound,
                        initial: state,
                        goal: loaded.goal,
                        use_costs: loaded.minimize_total_cost,
                    };
                    let mut initial = task.initial.clone();
                    task.derive(&mut initial);
                    task.initial = initial;
                    Ok(task)
                }

                /// The name of each object, by number.
                pub fn names(&self) -> &[::std::string::String] {
                    &self.names
                }

                /// The values of the objects of bound types.
                pub fn bound(&self) -> &Bound {
                    &self.bound
                }

                /// The static facts.
                pub fn statics(&self) -> &Statics {
                    &self.statics
                }

                /// The action in PDDL syntax, e.g. `(stack a b)`.
                pub fn describe(&self, action: Action) -> ::std::string::String {
                    match action {
                        #(#describe_arms)*
                    }
                }

                /// Whether the fact `pred(args)` holds, by predicate index and object numbers.
                pub fn fact(&self, state: &State, pred: usize, args: &[u32]) -> bool {
                    match pred {
                        #(#read_fact)*
                        _ => false,
                    }
                }

                /// The value of the fluent `func(args)`, by function index and object numbers.
                pub fn value(&self, state: &State, func: usize, args: &[u32]) -> ::core::option::Option<f64> {
                    match func {
                        #(#read_value)*
                        _ => ::core::option::Option::None,
                    }
                }

                #(#applies)*

                #derive
            }

            #allow
            impl ::pddl::runtime::Task for Task {
                type State = State;
                type Action = Action;

                fn initial_state(&self) -> State {
                    self.initial.clone()
                }

                fn successors(&self, state: &State, out: &mut ::std::vec::Vec<(Action, State)>) {
                    let undefined = ::core::cell::Cell::new(false);
                    #(#successors)*
                }

                fn apply(&self, state: &State, action: Action) -> State {
                    match action {
                        #(#apply_arms)*
                    }
                    .expect("the action is applicable in the state")
                }

                fn is_goal(&self, state: &State) -> bool {
                    self.goal.holds(
                        &|pred, args| self.fact(state, pred, args),
                        &|func, args| self.value(state, func, args),
                    )
                }

                fn unsatisfied_goals(&self, state: &State) -> u32 {
                    self.goal.unsatisfied(
                        &|pred, args| self.fact(state, pred, args),
                        &|func, args| self.value(state, func, args),
                    )
                }

                fn cost(&self, action: Action) -> u32 {
                    if !self.use_costs {
                        return 1;
                    }
                    match action {
                        #(#cost_arms)*
                    }
                }
            }
        }
    }
}
