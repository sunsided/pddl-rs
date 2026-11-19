//! The `#[task]` attribute: planning code generated from a domain by
//! `pddl-codegen`.

use pddl_core::{Domain, Parser};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemMod, LitStr};

use crate::expand::{self, CallSite, Form, Kind};

/// Where the attribute takes the domain from.
struct Arguments {
    /// The literal and whether it names a file or is the PDDL.
    domain: Option<(LitStr, Form)>,
}

/// Parses `domain = "file.pddl"` or `domain_source = "(define ...)"`.
fn arguments(attr: TokenStream) -> syn::Result<Arguments> {
    let mut args = Arguments { domain: None };
    let parser = syn::meta::parser(|meta| {
        let form = if meta.path.is_ident("domain") {
            Form::File
        } else if meta.path.is_ident("domain_source") {
            Form::Inline
        } else {
            return Err(
                meta.error("expected `domain = \"file.pddl\"` or `domain_source = \"...\"`")
            );
        };
        if args.domain.is_some() {
            return Err(meta.error("the domain is given twice"));
        }
        args.domain = Some((meta.value()?.parse()?, form));
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
        return Err(syn::Error::new_spanned(
            attr,
            "expected `domain = \"file.pddl\"` or `domain_source = \"...\"`",
        ));
    };
    let source = expand::load(form, &literal, call_site)?;
    expand::validate(Kind::Domain, &source, &literal)?;
    let domain = Domain::from_str(&source.text).expect("validated as a domain");
    let items = pddl_codegen::lifted(&domain)
        .map_err(|e| syn::Error::new(literal.span(), e.to_string()))?;

    let tracked = &source.expr;
    let generated: syn::File = syn::parse2(quote! {
        // Rebuild when the PDDL file changes.
        const _: &str = #tracked;
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
        let error = expand_str(&format!("file = {domain}"), "mod d {}").unwrap_err();
        assert!(error.to_string().starts_with("expected `domain"), "{error}");
        let error = expand_str(
            &format!("domain_source = {domain}, domain_source = {domain}"),
            "mod d {}",
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "the domain is given twice");
        assert!(expand_str(&format!("domain_source = {domain}"), "fn d() {}").is_err());
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
