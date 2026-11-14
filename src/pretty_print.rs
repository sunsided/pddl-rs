//! Renders PDDL types back to PDDL text, wrapped to a target line width.
//!
//! Each type gets a [`Visitor`] impl on [`PrettyRenderer`] that turns it into a
//! [`pretty`] document; [`Pretty::pretty`] renders that document through `Display`.
//! The output parses back to an equal value.

use std::fmt;

use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

mod action;
mod atomic_formula;
mod decl;
mod domain;
mod effect;
mod f_exp;
mod formula_skeleton;
mod function_type;
mod gd;
mod name;
mod number;
mod ops;
mod predicate;
mod problem;
mod term;
mod timed;
mod timeless;
mod r#type;
mod typed_list;

/// Keeps [`PrettyVisit`] implemented only for this crate's types.
mod sealed {
    /// Marks a type that has a pretty-print [`Visitor`](crate::visitor::Visitor) impl.
    pub trait Sealed {}
}

/// Converts a value into a [`pretty`] document through its [`Visitor`] impl on
/// [`PrettyRenderer`]. Implemented for every type that [`Pretty`] supports.
pub trait PrettyVisit: sealed::Sealed {
    /// Builds the document for `self` with the renderer `r`.
    fn to_doc(&self, r: &PrettyRenderer) -> RcDoc<'static>;
}

impl<T> PrettyVisit for T
where
    T: sealed::Sealed,
    PrettyRenderer: Visitor<T, RcDoc<'static>>,
{
    fn to_doc(&self, r: &PrettyRenderer) -> RcDoc<'static> {
        self.accept(r)
    }
}

impl<T: PrettyVisit> Pretty for T {
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self> {
        PrettyPrinted { value: self, width }
    }
}

impl<T: PrettyVisit> fmt::Display for PrettyPrinted<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let renderer = PrettyRenderer;
        let doc = self.value.to_doc(&renderer);
        doc.render_fmt(self.width, f)
    }
}

/// Renders PDDL values as [`pretty`] documents. It holds no state; each value's
/// layout comes from its [`Visitor`] impl, built from the helpers below.
#[derive(Default)]
pub struct PrettyRenderer;

impl PrettyRenderer {
    /// Renders `doc` to a string, wrapping lines longer than `width`.
    pub fn to_pretty(&self, doc: RcDoc<'static>, width: usize) -> String {
        let mut w = Vec::new();
        doc.render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

    /// `(head child ...)` on one line if it fits within the width, otherwise
    /// with the children on their own lines, indented by four.
    pub fn sexpr(
        &self,
        head: &'static str,
        children: impl IntoIterator<Item = RcDoc<'static>>,
    ) -> RcDoc<'static> {
        RcDoc::text("(")
            .append(RcDoc::text(head))
            .append(RcDoc::softline())
            .append(RcDoc::intersperse(children, RcDoc::softline()))
            .nest(4)
            .group()
            .append(")")
    }

    /// `(head` followed by each child on its own line, indented by four, and the
    /// closing `)` on a line of its own. Used for top-level forms such as `define`.
    pub fn sexpr_nested(
        &self,
        head: &'static str,
        children: impl IntoIterator<Item = RcDoc<'static>>,
    ) -> RcDoc<'static> {
        RcDoc::text("(")
            .append(RcDoc::text(head))
            .append(
                RcDoc::hardline()
                    .append(RcDoc::intersperse(children, RcDoc::hardline()))
                    .nest(4),
            )
            .append(RcDoc::hardline())
            .append(")")
    }

    /// A section `(:keyword body ...)`, laid out like [`sexpr`](Self::sexpr);
    /// `keyword` is given without its colon.
    pub fn section(
        &self,
        keyword: &'static str,
        body: impl IntoIterator<Item = RcDoc<'static>>,
    ) -> RcDoc<'static> {
        RcDoc::text("(")
            .append(RcDoc::text(":"))
            .append(RcDoc::text(keyword))
            .append(RcDoc::softline())
            .append(RcDoc::intersperse(body, RcDoc::softline()))
            .nest(4)
            .group()
            .append(")")
    }

    /// The keyword `:name`, e.g. `:parameters` before an action's parameter list.
    pub fn keyword_line(&self, name: &'static str) -> RcDoc<'static> {
        RcDoc::text(":").append(RcDoc::text(name))
    }

    /// The `items` separated by `sep`, without surrounding parentheses.
    pub fn list(
        &self,
        items: impl IntoIterator<Item = RcDoc<'static>>,
        sep: RcDoc<'static>,
    ) -> RcDoc<'static> {
        RcDoc::intersperse(items, sep)
    }
}

/// A value paired with a target line width, printable via [`std::fmt::Display`].
///
/// Produced by [`Pretty::pretty`]. The `Display` impl runs the pretty-printer
/// at the stored `width`.
pub struct PrettyPrinted<'a, T> {
    /// The value to print.
    value: &'a T,
    /// The line width to wrap at.
    width: usize,
}

/// Extension trait to obtain a [`Display`](fmt::Display)-able pretty-printed
/// adapter for any type with a registered pretty-print visitor impl.
pub trait Pretty: Sized {
    /// Wrap `self` together with a target line `width`. The returned value
    /// implements [`Display`](fmt::Display).
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self>;
}

/// Helper macro to quickly prettify an element.
#[cfg(test)]
macro_rules! prettify {
    ($x:expr, $n:literal) => {{
        let renderer = PrettyRenderer::default();
        let doc = $x.accept(&renderer);
        renderer.to_pretty(doc, $n)
    }};
}

#[cfg(test)]
pub(crate) use prettify;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Name;

    #[test]
    fn to_pretty_works() {
        let renderer = PrettyRenderer;
        let doc = RcDoc::text("hello");
        assert_eq!(renderer.to_pretty(doc, 80), "hello");
    }

    #[test]
    fn sexpr_nested_works() {
        let renderer = PrettyRenderer;
        let doc = renderer.sexpr_nested("head", [RcDoc::text("a"), RcDoc::text("b")]);
        let out = renderer.to_pretty(doc, 80);
        assert!(out.starts_with("(head"));
        assert!(out.contains('\n'));
    }

    #[test]
    fn list_works() {
        let renderer = PrettyRenderer;
        let doc = renderer.list([RcDoc::text("a"), RcDoc::text("b")], RcDoc::text(", "));
        assert_eq!(renderer.to_pretty(doc, 80), "a, b");
    }

    #[test]
    fn pretty_display_works() {
        let out = Name::new("x").pretty(10).to_string();
        assert_eq!(out, "x");
    }

    #[test]
    fn keyword_line_works() {
        let renderer = PrettyRenderer;
        let doc = renderer.keyword_line("foo");
        assert_eq!(renderer.to_pretty(doc, 80), ":foo");
    }
}
