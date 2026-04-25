use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    AssignOp, AssignOpT, BinaryComp, BinaryOp, DOp, Interval, MultiOp, Optimization, Requirement,
    TimeSpecifier,
};
use crate::visitor::Visitor;
use pretty::RcDoc;

impl sealed::Sealed for AssignOp {}
impl sealed::Sealed for AssignOpT {}
impl sealed::Sealed for BinaryOp {}
impl sealed::Sealed for BinaryComp {}
impl sealed::Sealed for DOp {}
impl sealed::Sealed for MultiOp {}
impl sealed::Sealed for Optimization {}
impl sealed::Sealed for Interval {}
impl sealed::Sealed for TimeSpecifier {}
impl sealed::Sealed for Requirement {}

impl<'a> Visitor<AssignOp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &AssignOp) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<AssignOpT, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &AssignOpT) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<BinaryOp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &BinaryOp) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<BinaryComp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &BinaryComp) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<DOp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &DOp) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<MultiOp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &MultiOp) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<Optimization, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Optimization) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<Interval, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Interval) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<TimeSpecifier, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &TimeSpecifier) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<Requirement, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Requirement) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn assign_op_works() {
        assert_eq!(prettify!(AssignOp::Assign, 10), "assign");
        assert_eq!(prettify!(AssignOp::ScaleUp, 10), "scale-up");
        assert_eq!(prettify!(AssignOp::Increase, 10), "increase");
    }

    #[test]
    fn binary_op_works() {
        assert_eq!(prettify!(BinaryOp::Addition, 10), "+");
        assert_eq!(prettify!(BinaryOp::Multiplication, 10), "*");
    }

    #[test]
    fn requirement_works() {
        assert_eq!(prettify!(Requirement::Strips, 10), ":strips");
        assert_eq!(prettify!(Requirement::Typing, 10), ":typing");
    }

    #[test]
    fn time_specifier_works() {
        assert_eq!(prettify!(TimeSpecifier::Start, 10), "start");
        assert_eq!(prettify!(TimeSpecifier::End, 10), "end");
    }
}
