use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    BinaryComparison, BinaryOp, DurativeActionFluentExpression, DurativeActionFunctionAssignment,
    FluentComparison, FluentExpression, FunctionHead, MetricFluentExpression, MultiOp,
    TimedFluentExpression,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for FluentExpression {}
impl sealed::Sealed for FunctionHead {}
impl sealed::Sealed for FluentComparison {}
impl sealed::Sealed for TimedFluentExpression {}
impl sealed::Sealed for DurativeActionFluentExpression {}
impl sealed::Sealed for DurativeActionFunctionAssignment {}
impl sealed::Sealed for MetricFluentExpression {}

impl Visitor<FluentExpression, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &FluentExpression) -> RcDoc<'static> {
        match value {
            FluentExpression::Number(n) => n.accept(self),
            FluentExpression::Function(head) => head.accept(self),
            FluentExpression::Negative(e) => RcDoc::text("(")
                .append("-")
                .append(RcDoc::softline())
                .append(self.visit(&**e))
                .nest(4)
                .group()
                .append(")"),
            FluentExpression::BinaryOp(op, a, b) => {
                let op_str = match op {
                    BinaryOp::Addition => "+",
                    BinaryOp::Subtraction => "-",
                    BinaryOp::Multiplication => "*",
                    BinaryOp::Division => "/",
                };
                self.sexpr(op_str, [self.visit(&**a), self.visit(&**b)])
            }
            FluentExpression::MultiOp(op, first, rest) => {
                let op_str = match op {
                    MultiOp::Addition => "+",
                    MultiOp::Multiplication => "*",
                };
                self.sexpr(
                    op_str,
                    std::iter::once(self.visit(&**first)).chain(rest.iter().map(|e| self.visit(e))),
                )
            }
        }
    }
}

impl Visitor<FunctionHead, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &FunctionHead) -> RcDoc<'static> {
        match value {
            FunctionHead::Simple(sym) => sym.accept(self),
            FunctionHead::WithTerms(sym, terms) => RcDoc::text("(")
                .append(sym.accept(self))
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    terms.iter().map(|t| t.accept(self)),
                    RcDoc::softline(),
                ))
                .nest(4)
                .group()
                .append(")"),
        }
    }
}

impl Visitor<FluentComparison, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &FluentComparison) -> RcDoc<'static> {
        let op_str = match value.comparison() {
            BinaryComparison::GreaterThan => ">",
            BinaryComparison::LessThan => "<",
            BinaryComparison::Equal => "=",
            BinaryComparison::GreaterOrEqual => ">=",
            BinaryComparison::LessThanOrEqual => "<=",
        };
        self.sexpr(
            op_str,
            [self.visit(value.first()), self.visit(value.second())],
        )
    }
}

impl Visitor<TimedFluentExpression, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &TimedFluentExpression) -> RcDoc<'static> {
        match value {
            TimedFluentExpression::Now => RcDoc::text("now"),
            TimedFluentExpression::Scaled(e) => self.visit(e),
        }
    }
}

impl Visitor<DurativeActionFluentExpression, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionFluentExpression) -> RcDoc<'static> {
        match value {
            DurativeActionFluentExpression::Duration => RcDoc::text("#t"),
            DurativeActionFluentExpression::FluentExpression(e) => self.visit(e),
            DurativeActionFluentExpression::Negative(e) => RcDoc::text("(")
                .append("-")
                .append(RcDoc::softline())
                .append(self.visit(&**e))
                .nest(4)
                .group()
                .append(")"),
            DurativeActionFluentExpression::BinaryOp(op, a, b) => {
                let op_str = match op {
                    BinaryOp::Addition => "+",
                    BinaryOp::Subtraction => "-",
                    BinaryOp::Multiplication => "*",
                    BinaryOp::Division => "/",
                };
                self.sexpr(op_str, [self.visit(&**a), self.visit(&**b)])
            }
            DurativeActionFluentExpression::MultiOp(op, first, rest) => {
                let op_str = match op {
                    MultiOp::Addition => "+",
                    MultiOp::Multiplication => "*",
                };
                self.sexpr(
                    op_str,
                    std::iter::once(self.visit(&**first)).chain(rest.iter().map(|e| self.visit(e))),
                )
            }
            DurativeActionFluentExpression::Assign(op, head, expr) => {
                self.sexpr(op.as_str(), [self.visit(head), self.visit(&**expr)])
            }
        }
    }
}

impl Visitor<DurativeActionFunctionAssignment, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionFunctionAssignment) -> RcDoc<'static> {
        self.sexpr(
            value.operation().as_str(),
            [
                self.visit(value.function()),
                self.visit(value.function_expr()),
            ],
        )
    }
}

impl Visitor<MetricFluentExpression, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &MetricFluentExpression) -> RcDoc<'static> {
        match value {
            MetricFluentExpression::Number(n) => n.accept(self),
            MetricFluentExpression::Function(sym, names) => {
                if names.is_empty() {
                    sym.accept(self)
                } else {
                    RcDoc::text("(")
                        .append(sym.accept(self))
                        .append(RcDoc::softline())
                        .append(RcDoc::intersperse(
                            names.iter().map(|n| n.accept(self)),
                            RcDoc::softline(),
                        ))
                        .nest(4)
                        .group()
                        .append(")")
                }
            }
            MetricFluentExpression::TotalTime => RcDoc::text("total-time"),
            MetricFluentExpression::IsViolated(pref) => RcDoc::text("(")
                .append("is-violated")
                .append(RcDoc::softline())
                .append(pref.accept(self))
                .nest(4)
                .group()
                .append(")"),
            MetricFluentExpression::Negative(e) => RcDoc::text("(")
                .append("-")
                .append(RcDoc::softline())
                .append(self.visit(&**e))
                .nest(4)
                .group()
                .append(")"),
            MetricFluentExpression::BinaryOp(op, a, b) => {
                let op_str = match op {
                    BinaryOp::Addition => "+",
                    BinaryOp::Subtraction => "-",
                    BinaryOp::Multiplication => "*",
                    BinaryOp::Division => "/",
                };
                self.sexpr(op_str, [self.visit(&**a), self.visit(&**b)])
            }
            MetricFluentExpression::MultiOp(op, first, rest) => {
                let op_str = match op {
                    MultiOp::Addition => "+",
                    MultiOp::Multiplication => "*",
                };
                self.sexpr(
                    op_str,
                    std::iter::once(self.visit(&**first)).chain(rest.iter().map(|e| self.visit(e))),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{
        AssignOp, BinaryComparison, BinaryOp, FunctionSymbol, MultiOp, Name, Number,
        PreferenceName, Variable,
    };

    #[test]
    fn fexp_number_works() {
        let n = FluentExpression::number(Number::from(42));
        assert_eq!(prettify!(n, 10), "42");
    }

    #[test]
    fn fexp_simple_function_works() {
        let head = FunctionHead::Simple(FunctionSymbol::from("fuel"));
        let e = FluentExpression::function(head);
        assert_eq!(prettify!(e, 10), "fuel");
    }

    #[test]
    fn fexp_function_with_terms_works() {
        let head = FunctionHead::WithTerms(
            FunctionSymbol::from("fuel"),
            vec![crate::Term::new_variable(Variable::from("r"))],
        );
        let e = FluentExpression::function(head);
        assert_eq!(prettify!(e, 20), "(fuel ?r)");
    }

    #[test]
    fn fexp_negative_works() {
        let inner = FluentExpression::number(Number::from(5));
        let e = FluentExpression::negative(inner);
        assert_eq!(prettify!(e, 10), "(- 5)");
    }

    #[test]
    fn fexp_binary_op_works() {
        let a = FluentExpression::number(Number::from(3));
        let b = FluentExpression::number(Number::from(4));
        let e = FluentExpression::binary_op(BinaryOp::Addition, a, b);
        assert_eq!(prettify!(e, 10), "(+ 3 4)");
    }

    #[test]
    fn fcomp_works() {
        let a = FluentExpression::number(Number::from(3));
        let b = FluentExpression::number(Number::from(4));
        let fc = FluentComparison::new(BinaryComparison::GreaterThan, a, b);
        assert_eq!(prettify!(fc, 10), "(> 3 4)");
    }

    #[test]
    fn fexp_t_now_works() {
        let e = TimedFluentExpression::Now;
        assert_eq!(prettify!(e, 10), "now");
    }

    #[test]
    fn fexp_t_scaled_works() {
        let inner = FluentExpression::number(Number::from(2));
        let e = TimedFluentExpression::Scaled(inner);
        assert_eq!(prettify!(e, 10), "2");
    }

    #[test]
    fn fexp_da_duration_works() {
        let e = DurativeActionFluentExpression::duration();
        assert_eq!(prettify!(e, 10), "#t");
    }

    #[test]
    fn fexp_da_fexp_works() {
        let inner = FluentExpression::number(Number::from(10));
        let e = DurativeActionFluentExpression::fluent_expression(inner);
        assert_eq!(prettify!(e, 10), "10");
    }

    #[test]
    fn f_assign_da_works() {
        let head = FunctionHead::Simple(FunctionSymbol::from("fuel"));
        let expr = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(5),
        ));
        let assign = DurativeActionFunctionAssignment::new(AssignOp::Assign, head, expr);
        assert_eq!(prettify!(assign, 20), "(assign fuel 5)");
    }

    #[test]
    fn metric_fexp_number_works() {
        let m = MetricFluentExpression::number(Number::from(10));
        assert_eq!(prettify!(m, 10), "10");
    }

    #[test]
    fn metric_fexp_total_time_works() {
        let m = MetricFluentExpression::total_time();
        assert_eq!(prettify!(m, 10), "total-time");
    }

    #[test]
    fn metric_fexp_function_works() {
        let m =
            MetricFluentExpression::function(FunctionSymbol::from("fuel"), vec![Name::new("r")]);
        assert_eq!(prettify!(m, 20), "(fuel r)");
    }

    #[test]
    fn metric_fexp_is_violated_works() {
        let m = MetricFluentExpression::is_violated(PreferenceName::string("p1"));
        assert_eq!(prettify!(m, 20), "(is-violated p1)");
    }

    #[test]
    fn fexp_multi_op_addition_works() {
        let a = FluentExpression::number(Number::from(1));
        let b = FluentExpression::number(Number::from(2));
        let c = FluentExpression::number(Number::from(3));
        let e = FluentExpression::multi_op(MultiOp::Addition, a, [b, c]);
        assert_eq!(prettify!(e, 20), "(+ 1 2 3)");
    }

    #[test]
    fn fexp_multi_op_multiplication_works() {
        let a = FluentExpression::number(Number::from(2));
        let b = FluentExpression::number(Number::from(3));
        let c = FluentExpression::number(Number::from(4));
        let e = FluentExpression::multi_op(MultiOp::Multiplication, a, [b, c]);
        assert_eq!(prettify!(e, 20), "(* 2 3 4)");
    }

    #[test]
    fn fexp_binary_op_subtraction_works() {
        let a = FluentExpression::number(Number::from(5));
        let b = FluentExpression::number(Number::from(3));
        let e = FluentExpression::binary_op(BinaryOp::Subtraction, a, b);
        assert_eq!(prettify!(e, 10), "(- 5 3)");
    }

    #[test]
    fn fexp_binary_op_division_works() {
        let a = FluentExpression::number(Number::from(10));
        let b = FluentExpression::number(Number::from(2));
        let e = FluentExpression::binary_op(BinaryOp::Division, a, b);
        assert_eq!(prettify!(e, 10), "(/ 10 2)");
    }

    #[test]
    fn fexp_binary_op_multiplication_works() {
        let a = FluentExpression::number(Number::from(3));
        let b = FluentExpression::number(Number::from(4));
        let e = FluentExpression::binary_op(BinaryOp::Multiplication, a, b);
        assert_eq!(prettify!(e, 10), "(* 3 4)");
    }

    #[test]
    fn fexp_da_negative_works() {
        let inner = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(5),
        ));
        let e = DurativeActionFluentExpression::negative(inner);
        assert_eq!(prettify!(e, 10), "(- 5)");
    }

    #[test]
    fn fexp_da_binary_op_subtraction_works() {
        let a = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(5),
        ));
        let b = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(3),
        ));
        let e = DurativeActionFluentExpression::binary_op(BinaryOp::Subtraction, a, b);
        assert_eq!(prettify!(e, 10), "(- 5 3)");
    }

    #[test]
    fn fexp_da_binary_op_division_works() {
        let a = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(10),
        ));
        let b = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(2),
        ));
        let e = DurativeActionFluentExpression::binary_op(BinaryOp::Division, a, b);
        assert_eq!(prettify!(e, 10), "(/ 10 2)");
    }

    #[test]
    fn fexp_da_binary_op_multiplication_works() {
        let a = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(3),
        ));
        let b = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(4),
        ));
        let e = DurativeActionFluentExpression::binary_op(BinaryOp::Multiplication, a, b);
        assert_eq!(prettify!(e, 10), "(* 3 4)");
    }

    #[test]
    fn fexp_da_multi_op_addition_works() {
        let a = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(1),
        ));
        let b = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(2),
        ));
        let c = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(3),
        ));
        let e = DurativeActionFluentExpression::multi_op(MultiOp::Addition, a, [b, c]);
        assert_eq!(prettify!(e, 20), "(+ 1 2 3)");
    }

    #[test]
    fn fexp_da_multi_op_multiplication_works() {
        let a = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(2),
        ));
        let b = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(3),
        ));
        let c = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(4),
        ));
        let e = DurativeActionFluentExpression::multi_op(MultiOp::Multiplication, a, [b, c]);
        assert_eq!(prettify!(e, 20), "(* 2 3 4)");
    }

    #[test]
    fn fexp_da_assign_works() {
        use crate::FunctionHead;
        let head = FunctionHead::Simple(FunctionSymbol::from("x"));
        let expr = DurativeActionFluentExpression::fluent_expression(FluentExpression::number(
            Number::from(10),
        ));
        let e = DurativeActionFluentExpression::Assign(AssignOp::Increase, head, Box::new(expr));
        assert_eq!(prettify!(e, 20), "(increase x 10)");
    }

    #[test]
    fn fcomp_less_than_works() {
        let a = FluentExpression::number(Number::from(3));
        let b = FluentExpression::number(Number::from(4));
        let fc = FluentComparison::new(BinaryComparison::LessThan, a, b);
        assert_eq!(prettify!(fc, 10), "(< 3 4)");
    }

    #[test]
    fn fcomp_equal_works() {
        let a = FluentExpression::number(Number::from(3));
        let b = FluentExpression::number(Number::from(4));
        let fc = FluentComparison::new(BinaryComparison::Equal, a, b);
        assert_eq!(prettify!(fc, 10), "(= 3 4)");
    }

    #[test]
    fn fcomp_greater_or_equal_works() {
        let a = FluentExpression::number(Number::from(3));
        let b = FluentExpression::number(Number::from(4));
        let fc = FluentComparison::new(BinaryComparison::GreaterOrEqual, a, b);
        assert_eq!(prettify!(fc, 10), "(>= 3 4)");
    }

    #[test]
    fn fcomp_less_than_or_equal_works() {
        let a = FluentExpression::number(Number::from(3));
        let b = FluentExpression::number(Number::from(4));
        let fc = FluentComparison::new(BinaryComparison::LessThanOrEqual, a, b);
        assert_eq!(prettify!(fc, 10), "(<= 3 4)");
    }

    #[test]
    fn metric_fexp_negative_works() {
        let inner = MetricFluentExpression::number(Number::from(5));
        let m = MetricFluentExpression::negative(inner);
        assert_eq!(prettify!(m, 10), "(- 5)");
    }

    #[test]
    fn metric_fexp_binary_op_subtraction_works() {
        let a = MetricFluentExpression::number(Number::from(5));
        let b = MetricFluentExpression::number(Number::from(3));
        let m = MetricFluentExpression::binary_op(BinaryOp::Subtraction, a, b);
        assert_eq!(prettify!(m, 10), "(- 5 3)");
    }

    #[test]
    fn metric_fexp_binary_op_multiplication_works() {
        let a = MetricFluentExpression::number(Number::from(3));
        let b = MetricFluentExpression::number(Number::from(4));
        let m = MetricFluentExpression::binary_op(BinaryOp::Multiplication, a, b);
        assert_eq!(prettify!(m, 10), "(* 3 4)");
    }

    #[test]
    fn metric_fexp_multi_op_addition_works() {
        let a = MetricFluentExpression::number(Number::from(1));
        let b = MetricFluentExpression::number(Number::from(2));
        let c = MetricFluentExpression::number(Number::from(3));
        let m = MetricFluentExpression::multi_op(MultiOp::Addition, a, [b, c]);
        assert_eq!(prettify!(m, 20), "(+ 1 2 3)");
    }

    #[test]
    fn metric_fexp_multi_op_multiplication_works() {
        let a = MetricFluentExpression::number(Number::from(2));
        let b = MetricFluentExpression::number(Number::from(3));
        let c = MetricFluentExpression::number(Number::from(4));
        let m = MetricFluentExpression::multi_op(MultiOp::Multiplication, a, [b, c]);
        assert_eq!(prettify!(m, 20), "(* 2 3 4)");
    }
}
