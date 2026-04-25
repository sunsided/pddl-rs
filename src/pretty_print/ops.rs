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

impl Visitor<AssignOp, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &AssignOp) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<AssignOpT, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &AssignOpT) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<BinaryOp, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &BinaryOp) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<BinaryComp, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &BinaryComp) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<DOp, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DOp) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<MultiOp, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &MultiOp) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<Optimization, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Optimization) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<Interval, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Interval) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<TimeSpecifier, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &TimeSpecifier) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<Requirement, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Requirement) -> RcDoc<'static> {
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

    #[test]
    fn assign_op_t_works() {
        assert_eq!(prettify!(AssignOpT::Increase, 10), "increase");
        assert_eq!(prettify!(AssignOpT::Decrease, 10), "decrease");
    }

    #[test]
    fn binary_comp_all_variants() {
        assert_eq!(prettify!(BinaryComp::GreaterThan, 10), ">");
        assert_eq!(prettify!(BinaryComp::LessThan, 10), "<");
        assert_eq!(prettify!(BinaryComp::Equal, 10), "=");
        assert_eq!(prettify!(BinaryComp::GreaterOrEqual, 10), ">=");
        assert_eq!(prettify!(BinaryComp::LessThanOrEqual, 10), "<=");
    }

    #[test]
    fn d_op_all_variants() {
        assert_eq!(prettify!(DOp::Equal, 10), "=");
        assert_eq!(prettify!(DOp::GreaterOrEqual, 10), ">=");
        assert_eq!(prettify!(DOp::LessThanOrEqual, 10), "<=");
    }

    #[test]
    fn multi_op_works() {
        assert_eq!(prettify!(MultiOp::Addition, 10), "+");
        assert_eq!(prettify!(MultiOp::Multiplication, 10), "*");
    }

    #[test]
    fn optimization_works() {
        assert_eq!(prettify!(Optimization::Minimize, 10), "minimize");
        assert_eq!(prettify!(Optimization::Maximize, 10), "maximize");
    }

    #[test]
    fn interval_works() {
        assert_eq!(prettify!(Interval::All, 10), "all");
    }

    #[test]
    fn requirement_more_variants() {
        assert_eq!(
            prettify!(Requirement::NegativePreconditions, 10),
            ":negative-preconditions"
        );
        assert_eq!(
            prettify!(Requirement::DisjunctivePreconditions, 10),
            ":disjunctive-preconditions"
        );
        assert_eq!(
            prettify!(Requirement::DurativeActions, 10),
            ":durative-actions"
        );
    }
}
