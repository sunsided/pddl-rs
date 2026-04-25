#![allow(dead_code)]
use std::fmt;

use crate::visitor::Accept;
use pretty::RcDoc;

mod name;
mod r#type;

#[derive(Default)]
pub struct PrettyRenderer;

impl PrettyRenderer {
    pub fn to_pretty(&self, doc: RcDoc<'_>, width: usize) -> String {
        let mut w = Vec::new();
        doc.render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
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

impl<'a> fmt::Display for PrettyPrinted<'a, crate::types::Name> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let renderer = PrettyRenderer;
        let doc = self.value.accept(&renderer);
        doc.render_fmt(self.width, f)
    }
}

impl<'a> fmt::Display for PrettyPrinted<'a, crate::types::Variable> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let renderer = PrettyRenderer;
        let doc = self.value.accept(&renderer);
        doc.render_fmt(self.width, f)
    }
}

impl<'a> fmt::Display for PrettyPrinted<'a, crate::types::FunctionSymbol> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let renderer = PrettyRenderer;
        let doc = self.value.accept(&renderer);
        doc.render_fmt(self.width, f)
    }
}

impl<'a> fmt::Display for PrettyPrinted<'a, crate::types::PrimitiveType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let renderer = PrettyRenderer;
        let doc = self.value.accept(&renderer);
        doc.render_fmt(self.width, f)
    }
}

impl<'a> fmt::Display for PrettyPrinted<'a, crate::types::Type> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let renderer = PrettyRenderer;
        let doc = self.value.accept(&renderer);
        doc.render_fmt(self.width, f)
    }
}

/// Extension trait to obtain a [`Display`](fmt::Display)-able pretty-printed
/// adapter for any type with a registered pretty-print visitor impl.
pub trait Pretty: Sized {
    /// Wrap `self` together with a target line `width`. The returned value
    /// implements [`Display`](fmt::Display).
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self>;
}

impl Pretty for crate::types::Name {
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self> {
        PrettyPrinted { value: self, width }
    }
}

impl Pretty for crate::types::Variable {
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self> {
        PrettyPrinted { value: self, width }
    }
}

impl Pretty for crate::types::FunctionSymbol {
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self> {
        PrettyPrinted { value: self, width }
    }
}

impl Pretty for crate::types::PrimitiveType {
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self> {
        PrettyPrinted { value: self, width }
    }
}

impl Pretty for crate::types::Type {
    fn pretty(&self, width: usize) -> PrettyPrinted<'_, Self> {
        PrettyPrinted { value: self, width }
    }
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
