//! Pretty-printing of function types.

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::FunctionType;
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for FunctionType {}

impl Visitor<FunctionType, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &FunctionType) -> RcDoc<'static> {
        let t: &crate::types::Type = value;
        t.accept(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::FunctionType;
    use assert2::check;

    #[test]
    fn function_type_number() {
        let x = FunctionType::NUMBER;
        check!(prettify!(x, 10) == "number");
    }

    #[test]
    fn function_type_custom() {
        let x = FunctionType::new("fuel");
        check!(prettify!(x, 10) == "fuel");
    }
}
