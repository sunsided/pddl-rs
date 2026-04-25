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

mod sealed {
    pub trait Sealed {}
}

pub trait PrettyVisit: sealed::Sealed {
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

#[derive(Default)]
pub struct PrettyRenderer;

impl PrettyRenderer {
    pub fn to_pretty(&self, doc: RcDoc<'static>, width: usize) -> String {
        let mut w = Vec::new();
        doc.render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }

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

    pub fn keyword_line(&self, name: &'static str) -> RcDoc<'static> {
        RcDoc::text(":").append(RcDoc::text(name))
    }

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
    value: &'a T,
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
