use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::Number;
use crate::visitor::Visitor;
use pretty::RcDoc;

impl sealed::Sealed for Number {}

impl<'a> Visitor<Number, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Number) -> RcDoc<'a> {
        RcDoc::text(format!("{}", **value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn number_works() {
        let x = Number::from(42);
        assert_eq!(prettify!(x, 10), "42");
    }

    #[test]
    fn float_number_works() {
        let x = Number::from(2.71);
        assert_eq!(prettify!(x, 10), "2.71");
    }
}
