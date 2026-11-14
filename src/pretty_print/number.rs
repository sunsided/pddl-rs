//! Pretty-printing of numbers.

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::Number;
use crate::visitor::Visitor;
use pretty::RcDoc;

impl sealed::Sealed for Number {}

impl Visitor<Number, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Number) -> RcDoc<'static> {
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

    #[test]
    fn large_number_prints_without_exponent() {
        // PDDL numbers have no exponent syntax, so the long form is the only valid one.
        let x = Number::from(1e300);
        let text = prettify!(x, 10);
        assert_eq!(text.len(), 301);
        assert!(text.bytes().all(|b| b.is_ascii_digit()));
        assert_eq!(text.parse::<Number>().unwrap(), x);
    }
}
