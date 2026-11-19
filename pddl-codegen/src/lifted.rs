//! Level B1: typed planning code for a lifted domain.

use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::model::{ActionInfo, Arg, Cond, Effect, Model};
use crate::Error;

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
    let mut seen = HashSet::new();
    names
        .iter()
        .map(|name| {
            let ident = case(name);
            if !seen.insert(ident.clone()) {
                return Err(Error::new(format!(
                    "two {kind}s map to the Rust name `{ident}`; rename `{name}`"
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
    /// One `Action` variant per action.
    actions: Vec<Ident>,
    /// The fields of each action variant, one per parameter.
    params: Vec<Vec<Ident>>,
}

impl Names {
    /// Names every item, failing on collisions.
    fn new(model: &Model) -> Result<Self, Error> {
        Ok(Self {
            types: idents(
                "type",
                &model
                    .types
                    .iter()
                    .map(|t| t.name.as_str())
                    .collect::<Vec<_>>(),
                camel,
            )?,
            predicates: idents(
                "predicate",
                &model
                    .predicates
                    .iter()
                    .map(|p| p.name.as_str())
                    .collect::<Vec<_>>(),
                snake,
            )?,
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
}

/// The variable holding slot `i`'s object number.
fn var(i: usize) -> Ident {
    format_ident!("v{}", i)
}

/// Generates the items of level B1 for `model`.
pub fn generate(model: &Model) -> Result<TokenStream, Error> {
    let generator = Gen {
        model,
        names: Names::new(model)?,
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
        let checks: Vec<TokenStream> = self.model.predicates[p]
            .args
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
            Cond::And(parts) => {
                let parts: Vec<_> = parts.iter().map(|p| self.cond(action, p, state)).collect();
                if parts.is_empty() {
                    quote!(true)
                } else {
                    quote!((#(#parts)&&*))
                }
            }
        }
    }

    /// The highest variable slot a condition reads, if any.
    fn last_slot(cond: &Cond) -> Option<usize> {
        let of = |args: &[Arg]| {
            args.iter()
                .filter_map(|a| match a {
                    Arg::Var(slot) => Some(*slot),
                    Arg::Const(_) => None,
                })
                .max()
        };
        match cond {
            Cond::Atom { args, .. } => of(args),
            Cond::Eq(a, b) => of(&[*a, *b]),
            Cond::Not(inner) => Self::last_slot(inner),
            Cond::And(parts) => parts.iter().filter_map(Self::last_slot).max(),
        }
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
            let depth = Self::last_slot(cond).map_or(0, |s| s + 1);
            checks[depth].push(self.cond(action, cond, &state));
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
        let mut body = quote! {
            out.push((
                Action::#variant { #(#fields: #values),* },
                self.#apply(state, #(#vars),*),
            ));
        };
        for depth in (0..action.arity).rev() {
            let v = var(depth);
            let slot = &action.slots[depth];
            let ty = slot.ty;
            let member = slot.either.as_ref().map_or_else(
                || quote!(true),
                |members| {
                    quote!((#((self.starts[#members]..self.ends[#members]).contains(&#v))||*))
                },
            );
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

    /// The apply method of action `a`: deletes, then adds, so an add wins.
    fn apply(&self, a: usize) -> TokenStream {
        let action = &self.model.actions[a];
        let name = Self::apply_fn(a);
        let vars: Vec<Ident> = (0..action.arity).map(var).collect();
        let next = quote!(next);
        let effect = |effect: &Effect| {
            let args: Vec<_> = effect
                .args
                .iter()
                .map(|&arg| self.arg(action, arg))
                .collect();
            let guard = self.type_guard(effect.pred, &args);
            let globals: Vec<TokenStream> = args.into_iter().map(|(g, _)| g).collect();
            let set = self.fact(
                &next,
                effect.pred,
                &globals,
                &quote!(self.starts),
                Some(effect.add),
            );
            quote!(if #guard { #set; })
        };
        let deletes = action.effects.iter().filter(|e| !e.add).map(effect);
        let adds = action.effects.iter().filter(|e| e.add).map(effect);
        let doc = format!("Applies `{}` to `state`.", action.name);
        quote! {
            #[doc = #doc]
            fn #name(&self, state: &State, #(#vars: u32),*) -> State {
                let mut next = state.clone();
                #(#deletes)*
                #(#adds)*
                next
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
                functions: &[],
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
                .filter(move |(_, p)| p.is_static == static_)
                .map(|(i, p)| {
                    let ident = &self.names.predicates[i];
                    let ty = self.storage_type(i);
                    let doc = format!("The facts of `{}`.", p.name);
                    quote!(#[doc = #doc] pub #ident: #ty)
                })
                .collect::<Vec<_>>()
        };
        let state_fields = field(false);
        let static_fields = field(true);
        let new_fields = |static_: bool| {
            model
                .predicates
                .iter()
                .enumerate()
                .filter(move |(_, p)| p.is_static == static_)
                .map(|(i, _)| {
                    let ident = &self.names.predicates[i];
                    let new = self.storage_new(i);
                    quote!(#ident: #new)
                })
                .collect::<Vec<_>>()
        };
        let new_state = new_fields(false);
        let new_statics = new_fields(true);

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
                .zip(&self.names.params[a])
                .map(|(slot, field)| {
                    let t = slot.ty;
                    quote!(#field.0 + self.starts[#t])
                })
                .collect::<Vec<_>>()
        };
        let apply_arms = (0..model.actions.len()).map(|a| {
            let variant = &self.names.actions[a];
            let fields = &self.names.params[a];
            let apply = Self::apply_fn(a);
            let globals = globals(a);
            quote!(Action::#variant { #(#fields),* } => self.#apply(state, #(#globals),*),)
        });
        let describe_arms = (0..model.actions.len()).map(|a| {
            let variant = &self.names.actions[a];
            let fields = &self.names.params[a];
            let name = &model.actions[a].name;
            let globals = globals(a);
            quote! {
                Action::#variant { #(#fields),* } => {
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
                /// The initial state.
                initial: State,
                /// The goal.
                goal: ::pddl::runtime::Formula,
            }

            #allow
            impl Task {
                /// Loads a problem for this domain.
                ///
                /// # Errors
                /// A [`LoadError`](::pddl::codegen::LoadError) if the problem is for another
                /// domain, uses undeclared names, or uses an unsupported construct.
                pub fn new(problem: &::pddl::Problem) -> ::core::result::Result<Self, ::pddl::codegen::LoadError> {
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
                    Ok(Self {
                        names: loaded.names,
                        starts: loaded.starts,
                        ends: loaded.ends,
                        constants: loaded.constants,
                        statics,
                        initial: state,
                        goal: loaded.goal,
                    })
                }

                /// The name of each object, by number.
                pub fn names(&self) -> &[::std::string::String] {
                    &self.names
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

                #(#applies)*
            }

            #allow
            impl ::pddl::runtime::Task for Task {
                type State = State;
                type Action = Action;

                fn initial_state(&self) -> State {
                    self.initial.clone()
                }

                fn successors(&self, state: &State, out: &mut ::std::vec::Vec<(Action, State)>) {
                    #(#successors)*
                }

                fn apply(&self, state: &State, action: Action) -> State {
                    match action {
                        #(#apply_arms)*
                    }
                }

                fn is_goal(&self, state: &State) -> bool {
                    self.goal.holds(&|pred, args| self.fact(state, pred, args), &|_, _| None)
                }

                fn unsatisfied_goals(&self, state: &State) -> u32 {
                    self.goal.unsatisfied(&|pred, args| self.fact(state, pred, args), &|_, _| None)
                }
            }
        }
    }
}
