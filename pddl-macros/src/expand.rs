//! The shared front end of all macros: read the input, check the PDDL, and
//! emit the expansion.
//!
//! ```mermaid
//! flowchart LR
//!     run["run (proc_macro)"] --> expand
//!     expand --> parse_literal --> load --> validate --> output
//!     validate --> diagnostics
//! ```

use std::path::{Path, PathBuf};

use pddl_core::parsers::check;
use pddl_core::{Domain, Parser, Problem};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{ParseStream, Parser as _};
use syn::{LitStr, Token};

mod diagnostics;

/// What the macro checks for and expands to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Exactly one domain; expands to `LazyLock<pddl::Domain>`.
    Domain,
    /// Exactly one problem; expands to `LazyLock<pddl::Problem>`.
    Problem,
    /// Any number of definitions; expands to `LazyLock<pddl::PddlFile>`.
    File,
    /// Any number of definitions; expands to the source text.
    Validated,
}

/// Where the PDDL comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// The string literal is the PDDL.
    Inline,
    /// The string literal is a path to a PDDL file.
    File,
}

/// What the compiler tells the macro about its call site. Built from
/// `proc_macro` spans by [`run`]; tests build it by hand, since `proc_macro`
/// only works inside the compiler.
#[derive(Debug, Clone, Default)]
pub struct CallSite {
    /// The directory that relative paths resolve against: the directory of the
    /// calling file, or the crate root if the compiler does not know the file.
    pub base_dir: PathBuf,
    /// The calling `.rs` file as the compiler shows it, e.g. `src/main.rs`.
    /// `None` outside the compiler.
    pub file: Option<String>,
    /// The 1-based line of the `.rs` file on which the string literal starts.
    /// `None` outside the compiler.
    pub line: Option<usize>,
}

/// The PDDL to check, with what the expansion needs to embed it.
struct Source {
    /// The PDDL text.
    text: String,
    /// An expression of type `&'static str` that yields the same text in the
    /// expansion: the literal itself, or `include_str!` of the file.
    expr: TokenStream,
    /// How error messages name the place of the PDDL.
    origin: diagnostics::Origin,
}

/// Expands a macro call inside the compiler.
pub fn run(kind: Kind, form: Form, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let call_site = call_site(&input);
    expand(kind, form, input.into(), &call_site)
        .unwrap_or_else(|error| compile_errors(&error))
        .into()
}

/// Turns errors into an expression that fails to compile with each of them.
///
/// `syn` emits one `compile_error!` per error, back to back. That is only a
/// valid expression for a single error, so the calls become statements of a
/// block whose value, `panic!()`, fits any expected type.
fn compile_errors(error: &syn::Error) -> TokenStream {
    let errors = error.to_compile_error();

    quote!({ #errors ::core::panic!() })
}

/// Describes the call site from the compiler's spans.
fn call_site(input: &proc_macro::TokenStream) -> CallSite {
    let span = proc_macro::Span::call_site();
    let base_dir = span
        .local_file()
        .and_then(|file| file.parent().map(Path::to_path_buf))
        .or_else(|| std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from))
        .unwrap_or_default();

    CallSite {
        base_dir,
        file: Some(span.file()),
        line: input
            .clone()
            .into_iter()
            .next()
            .map(|token| token.span().line()),
    }
}

/// Expands a macro call: one string literal, checked as PDDL of the given kind.
///
/// # Errors
/// Returns an error, spanning the literal, if the input is not one string
/// literal, the file cannot be read, or the PDDL does not parse as `kind`.
pub fn expand(
    kind: Kind,
    form: Form,
    input: TokenStream,
    call_site: &CallSite,
) -> syn::Result<TokenStream> {
    let literal = parse_literal.parse2(input)?;
    let source = load(form, &literal, call_site)?;
    validate(kind, &source, &literal)?;

    Ok(output(kind, &source.expr))
}

/// Parses the macro input: exactly one string literal, with an optional
/// trailing comma.
fn parse_literal(input: ParseStream<'_>) -> syn::Result<LitStr> {
    let literal = input
        .parse::<LitStr>()
        .map_err(|error| syn::Error::new(error.span(), "expected a string literal"))?;
    input.parse::<Option<Token![,]>>()?;

    Ok(literal)
}

/// Reads the PDDL, from the literal or from the file it names.
fn load(form: Form, literal: &LitStr, call_site: &CallSite) -> syn::Result<Source> {
    match form {
        Form::Inline => Ok(Source {
            text: literal.value(),
            expr: quote!(#literal),
            origin: diagnostics::Origin::Inline {
                file: call_site.file.clone(),
                line: call_site.line,
                lines_match: lines_match(literal),
            },
        }),
        Form::File => load_file(literal, call_site),
    }
}

/// Whether each line of the literal's value is a line of the source: true
/// for raw strings and for strings without any `\` escape.
fn lines_match(literal: &LitStr) -> bool {
    let token = literal.token().to_string();

    token.starts_with('r') || !token.contains('\\')
}

/// Reads a PDDL file and embeds it with `include_str!`, which also makes Cargo
/// rebuild the crate when the file changes.
fn load_file(literal: &LitStr, call_site: &CallSite) -> syn::Result<Source> {
    let relative = literal.value();
    let path = call_site.base_dir.join(&relative);
    let error = |message: String| syn::Error::new(literal.span(), message);

    let path = path
        .canonicalize()
        .map_err(|e| error(format!("couldn't read `{}`: {e}", path.display())))?;
    let text = std::fs::read_to_string(&path)
        .map_err(|e| error(format!("couldn't read `{}`: {e}", path.display())))?;
    let path = path
        .to_str()
        .ok_or_else(|| error(format!("the path `{}` is not valid UTF-8", path.display())))?;
    let path = LitStr::new(path, literal.span());

    Ok(Source {
        text,
        expr: quote!(::core::include_str!(#path)),
        origin: diagnostics::Origin::File { path: relative },
    })
}

/// Checks the PDDL: every syntax error, then whether it is the kind of
/// definition the macro expects.
fn validate(kind: Kind, source: &Source, literal: &LitStr) -> syn::Result<()> {
    let errors = check(&source.text)
        .iter()
        .map(|error| diagnostics::message(error, &source.text, &source.origin))
        .map(|message| syn::Error::new(literal.span(), message))
        .reduce(|mut all, error| {
            all.combine(error);
            all
        });
    if let Some(errors) = errors {
        return Err(errors);
    }

    let wrong_kind = match kind {
        Kind::Domain => Domain::from_str(&source.text)
            .err()
            .map(|e| ("a single domain", e)),
        Kind::Problem => Problem::from_str(&source.text)
            .err()
            .map(|e| ("a single problem", e)),
        Kind::File | Kind::Validated => None,
    };
    match wrong_kind {
        Some((expected, error)) => {
            let message = diagnostics::message(&error, &source.text, &source.origin);
            Err(syn::Error::new(
                literal.span(),
                format!("expected {expected}: {message}"),
            ))
        }
        None => Ok(()),
    }
}

/// Builds the expansion around the source expression.
fn output(kind: Kind, source: &TokenStream) -> TokenStream {
    let ty = match kind {
        Kind::Domain => quote!(::pddl::Domain),
        Kind::Problem => quote!(::pddl::Problem),
        Kind::File => quote!(::pddl::PddlFile),
        Kind::Validated => return source.clone(),
    };

    quote! {
        ::std::sync::LazyLock::new(|| {
            <#ty as ::pddl::Parser>::from_str(#source).expect(
                "pddl-macros checked this PDDL at compile time; \
                 the `pddl-core` version must match the one `pddl-macros` uses",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expands `input` as if called from this crate's root, outside the compiler.
    fn expand_str(kind: Kind, form: Form, input: &str) -> syn::Result<String> {
        let call_site = CallSite {
            base_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            ..CallSite::default()
        };
        expand(kind, form, input.parse().unwrap(), &call_site).map(|tokens| tokens.to_string())
    }

    /// Collects the messages of all combined errors.
    fn messages(error: &syn::Error) -> Vec<String> {
        error.clone().into_iter().map(|e| e.to_string()).collect()
    }

    #[test]
    fn domain_expands_to_lazy_parse() {
        let tokens = expand_str(Kind::Domain, Form::Inline, r#""(define (domain d))""#).unwrap();
        assert!(tokens.contains("LazyLock :: new"), "{tokens}");
        assert!(tokens.contains(":: pddl :: Domain"), "{tokens}");
    }

    #[test]
    fn validated_expands_to_the_literal() {
        let tokens =
            expand_str(Kind::Validated, Form::Inline, r#""(define (domain d))","#).unwrap();
        assert_eq!(tokens, r#""(define (domain d))""#);
    }

    #[test]
    fn reports_every_syntax_error() {
        let input =
            r#""(define (domain d)\n  (:predicats (p))\n  (:action a :parameters (?x -)))""#;
        let error = expand_str(Kind::Domain, Form::Inline, input).unwrap_err();
        let messages = messages(&error);
        assert_eq!(messages.len(), 2, "{messages:#?}");
        assert!(
            messages[0].starts_with("line 2, column 3: "),
            "{}",
            messages[0]
        );
        assert!(
            messages[0].contains("= help: did you mean `:predicates`?"),
            "{}",
            messages[0]
        );
        assert!(
            messages[0].contains("2 |   (:predicats (p))"),
            "{}",
            messages[0]
        );
    }

    #[test]
    fn rejects_the_wrong_kind() {
        let input = r#""(define (problem p) (:domain d) (:init) (:goal (and)))""#;
        let error = expand_str(Kind::Domain, Form::Inline, input).unwrap_err();
        assert!(
            error.to_string().starts_with("expected a single domain: "),
            "{error}"
        );
        assert!(expand_str(Kind::Problem, Form::Inline, input).is_ok());
        assert!(expand_str(Kind::File, Form::Inline, input).is_ok());
    }

    #[test]
    fn rejects_two_definitions_where_one_is_expected() {
        let input = r#""(define (domain a)) (define (domain b))""#;
        assert!(expand_str(Kind::Domain, Form::Inline, input).is_err());
        assert!(expand_str(Kind::File, Form::Inline, input).is_ok());
    }

    #[test]
    fn rejects_input_that_is_not_one_string_literal() {
        for input in ["", "42", "\"a\" \"b\"", "domain"] {
            let error = expand_str(Kind::Domain, Form::Inline, input).unwrap_err();
            assert!(!error.to_string().is_empty(), "{input}");
        }
        let error = expand_str(Kind::Domain, Form::Inline, "x").unwrap_err();
        assert_eq!(error.to_string(), "expected a string literal");
    }

    #[test]
    fn only_escape_free_literals_map_lines() {
        let literal = |text: &str| syn::parse_str::<LitStr>(text).unwrap();
        assert!(lines_match(&literal(r##"r#"(define\n(domain d))"#"##)));
        assert!(lines_match(&literal(r#""(define (domain d))""#)));
        assert!(!lines_match(&literal(r#""(define\n(domain d))""#)));
    }

    #[test]
    fn reads_escaped_strings() {
        let input = r#""(define (domain d)\n\t(:predicates (p)))""#;
        assert!(expand_str(Kind::Domain, Form::Inline, input).is_ok());
    }

    #[test]
    fn file_form_includes_the_absolute_path() {
        let tokens = expand_str(
            Kind::Domain,
            Form::File,
            r#""tests/fixtures/switches-domain.pddl""#,
        )
        .unwrap();
        assert!(tokens.contains("include_str !"), "{tokens}");
        assert!(tokens.contains(env!("CARGO_MANIFEST_DIR")), "{tokens}");
    }

    #[test]
    fn file_form_names_the_file_in_errors() {
        let error = expand_str(
            Kind::Domain,
            Form::File,
            r#""tests/fixtures/broken-domain.pddl""#,
        )
        .unwrap_err();
        let message = error.to_string();
        assert!(
            message.starts_with("tests/fixtures/broken-domain.pddl:3:3: "),
            "{message}"
        );
    }

    #[test]
    fn missing_file_is_an_error() {
        let error =
            expand_str(Kind::Domain, Form::File, r#""tests/fixtures/missing.pddl""#).unwrap_err();
        assert!(error.to_string().starts_with("couldn't read `"), "{error}");
    }
}
