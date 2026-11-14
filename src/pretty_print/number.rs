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
    use assert2::check;

    #[test]
    fn number_works() {
        let x = Number::from(42);
        check!(prettify!(x, 10) == "42");
    }

    #[test]
    fn float_number_works() {
        let x = Number::from(2.71);
        check!(prettify!(x, 10) == "2.71");
    }

    #[test]
    fn large_number_prints_without_exponent() {
        // PDDL numbers have no exponent syntax, so the long form is the only valid one.
        let x = Number::from(1e300);
        let text = prettify!(x, 10);
        check!(text.len() == 301);
        check!(text.bytes().all(|b| b.is_ascii_digit()));
        check!((text.parse::<Number>().unwrap()) == x);
    }
}
