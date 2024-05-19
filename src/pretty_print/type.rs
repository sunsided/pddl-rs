use crate::pretty_print::PrettyRenderer;
use crate::types::{PrimitiveType, Type};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl<'a> Visitor<PrimitiveType, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &PrimitiveType) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<Type, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Type) -> RcDoc<'a> {
        match value {
            Type::Exactly(t) => RcDoc::text(t.to_string()),
            Type::EitherOf(ts) => RcDoc::text("(either")
                .append(RcDoc::softline())
                .group()
                .nest(4)
                .append(RcDoc::intersperse(
                    ts.iter().map(|t| t.accept(self)),
                    RcDoc::softline(),
                ))
                .nest(4)
                .group()
                .append(")"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn primitive_type_works() {
        let x = PrimitiveType::from("pt");
        assert_eq!(prettify!(x, 10), "pt");
    }

    #[test]
    fn simple_type_works() {
        let x = Type::from("a");
        assert_eq!(prettify!(x, 10), "a");
    }

    #[test]
    fn either_type_works() {
        let x = Type::from_iter(["a", "b"]);
        assert_eq!(prettify!(x, 12), "(either a b)");
        assert_eq!(prettify!(x, 10), "(either a\n    b)");
        assert_eq!(prettify!(x, 8), "(either\n    a b)");
    }
}