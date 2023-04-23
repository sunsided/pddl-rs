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
