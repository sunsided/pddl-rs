//! Pretty-printing of operators, keywords and requirement keys.

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    AssignOp, BinaryComparison, BinaryOp, DurationOperator, Interval, MultiOp, Optimization,
    Requirement, TimeSpecifier, TimedAssignOperator,
};
use crate::visitor::Visitor;
use pretty::RcDoc;

impl sealed::Sealed for AssignOp {}
impl sealed::Sealed for TimedAssignOperator {}
impl sealed::Sealed for BinaryOp {}
impl sealed::Sealed for BinaryComparison {}
impl sealed::Sealed for DurationOperator {}
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

impl Visitor<TimedAssignOperator, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &TimedAssignOperator) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<BinaryOp, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &BinaryOp) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<BinaryComparison, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &BinaryComparison) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<DurationOperator, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurationOperator) -> RcDoc<'static> {
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
    use assert2::check;

    #[test]
    fn assign_op_works() {
        check!(prettify!(AssignOp::Assign, 10) == "assign");
        check!(prettify!(AssignOp::ScaleUp, 10) == "scale-up");
        check!(prettify!(AssignOp::Increase, 10) == "increase");
    }

    #[test]
    fn binary_op_works() {
        check!(prettify!(BinaryOp::Addition, 10) == "+");
        check!(prettify!(BinaryOp::Multiplication, 10) == "*");
    }

    #[test]
    fn requirement_works() {
        check!(prettify!(Requirement::Strips, 10) == ":strips");
        check!(prettify!(Requirement::Typing, 10) == ":typing");
    }

    #[test]
    fn time_specifier_works() {
        check!(prettify!(TimeSpecifier::Start, 10) == "start");
        check!(prettify!(TimeSpecifier::End, 10) == "end");
    }

    #[test]
    fn assign_op_t_works() {
        check!(prettify!(TimedAssignOperator::Increase, 10) == "increase");
        check!(prettify!(TimedAssignOperator::Decrease, 10) == "decrease");
    }

    #[test]
    fn binary_comp_all_variants() {
        check!(prettify!(BinaryComparison::GreaterThan, 10) == ">");
        check!(prettify!(BinaryComparison::LessThan, 10) == "<");
        check!(prettify!(BinaryComparison::Equal, 10) == "=");
        check!(prettify!(BinaryComparison::GreaterOrEqual, 10) == ">=");
        check!(prettify!(BinaryComparison::LessThanOrEqual, 10) == "<=");
    }

    #[test]
    fn d_op_all_variants() {
        check!(prettify!(DurationOperator::Equal, 10) == "=");
        check!(prettify!(DurationOperator::GreaterOrEqual, 10) == ">=");
        check!(prettify!(DurationOperator::LessThanOrEqual, 10) == "<=");
    }

    #[test]
    fn multi_op_works() {
        check!(prettify!(MultiOp::Addition, 10) == "+");
        check!(prettify!(MultiOp::Multiplication, 10) == "*");
    }

    #[test]
    fn optimization_works() {
        check!(prettify!(Optimization::Minimize, 10) == "minimize");
        check!(prettify!(Optimization::Maximize, 10) == "maximize");
    }

    #[test]
    fn interval_works() {
        check!(prettify!(Interval::All, 10) == "all");
    }

    #[test]
    fn requirement_more_variants() {
        check!(prettify!(Requirement::NegativePreconditions, 10) == ":negative-preconditions");
        check!(
            prettify!(Requirement::DisjunctivePreconditions, 10) == ":disjunctive-preconditions"
        );
        check!(prettify!(Requirement::DurativeActions, 10) == ":durative-actions");
    }
}
