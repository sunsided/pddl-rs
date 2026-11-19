//! The `#[task]` attribute: planning code generated from a domain by
//! `pddl-codegen`.

use pddl_core::{Domain, Parser, Problem};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemMod, LitStr};

use crate::expand::{self, CallSite, Form, Kind};

/// What the attribute says.
#[derive(Default)]
struct Arguments {
    /// The domain literal and whether it names a file or is the PDDL.
    domain: Option<(LitStr, Form)>,
    /// The same for the problem; given, the task is grounded (level B2).
    problem: Option<(LitStr, Form)>,
    /// The size limits of a grounded task.
    limits: pddl_codegen::Limits,
    /// Types bound to Rust types, and attached predicates and functions.
    bindings: pddl_codegen::Bindings,
}

/// What the attribute accepts, for error messages.
const EXPECTED: &str = "expected `domain = \"file.pddl\"` or `domain_source = \"...\"`, \
     optionally `problem`/`problem_source`, `max_operators`, `max_facts`, \
     `types(name = Type, ...)` and `attach(name = path, ...)`";

/// Parses `domain = "file.pddl"` or `domain_source = "(define ...)"`, and the
/// same for `problem`, plus `max_operators = N` and `max_facts = N`.
fn arguments(attr: TokenStream) -> syn::Result<Arguments> {
    let mut args = Arguments::default();
    let parser = syn::meta::parser(|meta| {
        let path = &meta.path;
        let (slot, form) = if path.is_ident("domain") {
            (&mut args.domain, Form::File)
        } else if path.is_ident("domain_source") {
            (&mut args.domain, Form::Inline)
        } else if path.is_ident("problem") {
            (&mut args.problem, Form::File)
        } else if path.is_ident("problem_source") {
            (&mut args.problem, Form::Inline)
        } else if path.is_ident("types") || path.is_ident("attach") {
            let types = path.is_ident("types");
            return meta.parse_nested_meta(|inner| {
                // PDDL names may have `-`, which an identifier can't; codegen
                // also matches `_` as `-`.
                let name = inner
                    .path
                    .get_ident()
                    .ok_or_else(|| inner.error("expected a PDDL name"))?
                    .to_string();
                let value = inner.value()?;
                if types {
                    let ty: syn::Type = value.parse()?;
                    args.bindings.types.push((name, quote!(#ty)));
                } else {
                    let function: syn::Path = value.parse()?;
                    args.bindings.attachments.push((name, quote!(#function)));
                }
                Ok(())
            });
        } else if path.is_ident("max_operators") || path.is_ident("max_facts") {
            let value: syn::LitInt = meta.value()?.parse()?;
            let value = value.base10_parse()?;
            if path.is_ident("max_operators") {
                args.limits.max_operators = value;
            } else {
                args.limits.max_facts = value;
            }
            return Ok(());
        } else {
            return Err(meta.error(EXPECTED));
        };
        if slot.is_some() {
            return Err(meta.error("given twice"));
        }
        *slot = Some((meta.value()?.parse()?, form));
        Ok(())
    });
    syn::parse::Parser::parse2(parser, attr)?;
    Ok(args)
}

/// Expands the attribute on `item`, a module, inside the compiler.
pub fn run(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let call_site = expand::call_site(&attr);
    expand_task(attr.into(), item.into(), &call_site)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

/// Fills the module `item` with the code generated for the domain the
/// attribute names.
///
/// # Errors
/// If the arguments or the item are malformed, the PDDL does not parse as one
/// domain, or code generation fails.
pub fn expand_task(
    attr: TokenStream,
    item: TokenStream,
    call_site: &CallSite,
) -> syn::Result<TokenStream> {
    let args = arguments(attr.clone())?;
    let mut module: ItemMod = syn::parse2(item)?;
    let Some((literal, form)) = args.domain else {
        return Err(syn::Error::new_spanned(attr, EXPECTED));
    };
    let source = expand::load(form, &literal, call_site)?;
    expand::validate(Kind::Domain, &source, &literal)?;
    let domain = Domain::from_str(&source.text).expect("validated as a domain");
    let mut tracked = vec![source.expr];
    let items = match args.problem {
        None => pddl_codegen::lifted_with(&domain, &args.bindings),
        Some(_) if !args.bindings.types.is_empty() || !args.bindings.attachments.is_empty() => {
            return Err(syn::Error::new_spanned(
                attr,
                "`types` and `attach` need the lifted form: leave out `problem`",
            ));
        }
        Some((problem_literal, form)) => {
            let problem_source = expand::load(form, &problem_literal, call_site)?;
            expand::validate(Kind::Problem, &problem_source, &problem_literal)?;
            let problem = Problem::from_str(&problem_source.text).expect("validated as a problem");
            tracked.push(problem_source.expr);
            pddl_codegen::grounded(&domain, &problem, &args.limits)
        }
    }
    .map_err(|e| syn::Error::new(literal.span(), e.to_string()))?;

    let generated: syn::File = syn::parse2(quote! {
        // Rebuild when a PDDL file changes.
        #(const _: &str = #tracked;)*
        #items
    })?;
    let content = &mut module
        .content
        .get_or_insert_with(|| (syn::token::Brace::default(), Vec::new()))
        .1;
    content.extend(generated.items);
    module.semi = None;
    Ok(quote!(#module))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Expands the attribute outside the compiler.
    fn expand_str(attr: &str, item: &str) -> syn::Result<String> {
        let call_site = CallSite {
            base_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            ..CallSite::default()
        };
        expand_task(attr.parse().unwrap(), item.parse().unwrap(), &call_site).map(|t| t.to_string())
    }

    #[test]
    fn fills_the_module() {
        let tokens = expand_str(
            r#"domain_source = "(define (domain d) (:predicates (p)) (:action a :parameters () :precondition () :effect (p)))""#,
            "pub mod d {}",
        )
        .unwrap();
        assert!(tokens.starts_with("pub mod d {"), "{tokens}");
        assert!(tokens.contains("pub struct State"), "{tokens}");
    }

    #[test]
    fn rejects_bad_arguments() {
        let domain = r#""(define (domain d))""#;
        let error = expand_str("", "mod d {}").unwrap_err();
        assert!(error.to_string().starts_with("expected `domain"), "{error}");
        let error = expand_str(
            &format!("domain_source = {domain}, max_operators = x"),
            "mod d {}",
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "expected integer literal");
        let error = expand_str(&format!("file = {domain}"), "mod d {}").unwrap_err();
        assert!(error.to_string().starts_with("expected `domain"), "{error}");
        let error = expand_str(
            &format!("domain_source = {domain}, domain_source = {domain}"),
            "mod d {}",
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "given twice");
        assert!(expand_str(&format!("domain_source = {domain}"), "fn d() {}").is_err());
    }

    #[test]
    fn grounds_with_a_problem() {
        let domain = r#""(define (domain d) (:predicates (p ?x)) (:action a :parameters (?x) :precondition () :effect (p ?x)))""#;
        let problem =
            r#""(define (problem q) (:domain d) (:objects o1 o2) (:init) (:goal (p o1)))""#;
        let tokens = expand_str(
            &format!("domain_source = {domain}, problem_source = {problem}"),
            "mod d {}",
        )
        .unwrap();
        assert!(tokens.contains("pub static OPERATORS"), "{tokens}");
        let error = expand_str(
            &format!("domain_source = {domain}, problem_source = {problem}, max_operators = 1"),
            "mod d {}",
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .starts_with("the grounded task has 2 operators"),
            "{error}"
        );
    }

    #[test]
    fn checks_bindings() {
        let domain = r#""(define (domain d) (:types robot) (:predicates (p ?x) (q ?x)) (:action a :parameters (?x) :precondition (q ?x) :effect (p ?x)))""#;
        let ok = expand_str(
            &format!("domain_source = {domain}, types(robot = u8), attach(q = crate::q)"),
            "mod d {}",
        )
        .unwrap();
        assert!(ok.contains("pub robot"), "{ok}");
        let error = |args: &str| {
            expand_str(&format!("domain_source = {domain}, {args}"), "mod d {}")
                .unwrap_err()
                .to_string()
        };
        assert_eq!(
            error("attach(p = crate::p)"),
            "can't attach `p`: actions or rules change it, and attachments must be static"
        );
        assert_eq!(
            error("attach(r = crate::r)"),
            "can't attach `r`: no such predicate or function"
        );
        assert_eq!(
            error("types(truck = u8)"),
            "can't bind `truck`: no such type"
        );
        let problem = r#""(define (problem q) (:domain d) (:init) (:goal (and)))""#;
        assert_eq!(
            error(&format!("problem_source = {problem}, types(robot = u8)")),
            "`types` and `attach` need the lifted form: leave out `problem`"
        );
    }

    #[test]
    fn reports_unsupported_constructs() {
        let error = expand_str(
            r#"domain_source = "(define (domain d) (:requirements :durative-actions))""#,
            "mod d {}",
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "unsupported requirement: :durative-actions"
        );
    }

    #[test]
    fn reports_syntax_errors() {
        let error = expand_str(
            r#"domain_source = "(define (domain d) (:predicats (p)))""#,
            "mod d {}",
        )
        .unwrap_err();
        assert!(error.to_string().contains("unknown section"), "{error}");
    }
}
