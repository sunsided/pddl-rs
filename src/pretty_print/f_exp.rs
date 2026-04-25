use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    BinaryComp, BinaryOp, FAssignDa, FComp, FExp, FExpDa, FExpT, FHead, MetricFExp, MultiOp,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for FExp {}
impl sealed::Sealed for FHead {}
impl sealed::Sealed for FComp {}
impl sealed::Sealed for FExpT {}
impl sealed::Sealed for FExpDa {}
impl sealed::Sealed for FAssignDa {}
impl sealed::Sealed for MetricFExp {}

impl<'a> Visitor<FExp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FExp) -> RcDoc<'a> {
        match value {
            FExp::Number(n) => n.accept(self),
            FExp::Function(head) => head.accept(self),
            FExp::Negative(e) => RcDoc::text("(")
                .append("-")
                .append(RcDoc::softline())
                .append(self.visit(&**e))
                .nest(4)
                .group()
                .append(")"),
            FExp::BinaryOp(op, a, b) => {
                let op_str = match op {
                    BinaryOp::Addition => "+",
                    BinaryOp::Subtraction => "-",
                    BinaryOp::Multiplication => "*",
                    BinaryOp::Division => "/",
                };
                self.sexpr(op_str, [self.visit(&**a), self.visit(&**b)])
            }
            FExp::MultiOp(op, first, rest) => {
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

impl<'a> Visitor<FHead, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FHead) -> RcDoc<'a> {
        match value {
            FHead::Simple(sym) => sym.accept(self),
            FHead::WithTerms(sym, terms) => RcDoc::text("(")
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

impl<'a> Visitor<FComp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FComp) -> RcDoc<'a> {
        let op_str = match value.comparison() {
            BinaryComp::GreaterThan => ">",
            BinaryComp::LessThan => "<",
            BinaryComp::Equal => "=",
            BinaryComp::GreaterOrEqual => ">=",
            BinaryComp::LessThanOrEqual => "<=",
        };
        self.sexpr(
            op_str,
            [self.visit(value.first()), self.visit(value.second())],
        )
    }
}

impl<'a> Visitor<FExpT, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FExpT) -> RcDoc<'a> {
        match value {
            FExpT::Now => RcDoc::text("now"),
            FExpT::Scaled(e) => self.visit(e),
        }
    }
}

impl<'a> Visitor<FExpDa, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FExpDa) -> RcDoc<'a> {
        match value {
            FExpDa::Duration => RcDoc::text("#t"),
            FExpDa::FExp(e) => self.visit(e),
            FExpDa::Negative(e) => RcDoc::text("(")
                .append("-")
                .append(RcDoc::softline())
                .append(self.visit(&**e))
                .nest(4)
                .group()
                .append(")"),
            FExpDa::BinaryOp(op, a, b) => {
                let op_str = match op {
                    BinaryOp::Addition => "+",
                    BinaryOp::Subtraction => "-",
                    BinaryOp::Multiplication => "*",
                    BinaryOp::Division => "/",
                };
                self.sexpr(op_str, [self.visit(&**a), self.visit(&**b)])
            }
            FExpDa::MultiOp(op, first, rest) => {
                let op_str = match op {
                    MultiOp::Addition => "+",
                    MultiOp::Multiplication => "*",
                };
                self.sexpr(
                    op_str,
                    std::iter::once(self.visit(&**first)).chain(rest.iter().map(|e| self.visit(e))),
                )
            }
            FExpDa::Assign(op, head, expr) => {
                self.sexpr(op.as_str(), [self.visit(head), self.visit(&**expr)])
            }
        }
    }
}

impl<'a> Visitor<FAssignDa, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FAssignDa) -> RcDoc<'a> {
        self.sexpr(
            value.operation().as_str(),
            [
                self.visit(value.function()),
                self.visit(value.function_expr()),
            ],
        )
    }
}

impl<'a> Visitor<MetricFExp, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &MetricFExp) -> RcDoc<'a> {
        match value {
            MetricFExp::Number(n) => n.accept(self),
            MetricFExp::Function(sym, names) => {
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
            MetricFExp::TotalTime => RcDoc::text("total-time"),
            MetricFExp::IsViolated(pref) => RcDoc::text("(")
                .append("is-violated")
                .append(RcDoc::softline())
                .append(pref.accept(self))
                .nest(4)
                .group()
                .append(")"),
            MetricFExp::Negative(e) => RcDoc::text("(")
                .append("-")
                .append(RcDoc::softline())
                .append(self.visit(&**e))
                .nest(4)
                .group()
                .append(")"),
            MetricFExp::BinaryOp(op, a, b) => {
                let op_str = match op {
                    BinaryOp::Addition => "+",
                    BinaryOp::Subtraction => "-",
                    BinaryOp::Multiplication => "*",
                    BinaryOp::Division => "/",
                };
                self.sexpr(op_str, [self.visit(&**a), self.visit(&**b)])
            }
            MetricFExp::MultiOp(op, first, rest) => {
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
    use crate::{AssignOp, FunctionSymbol, Name, Number, PreferenceName, Variable};

    #[test]
    fn fexp_number_works() {
        let n = FExp::new_number(Number::from(42));
        assert_eq!(prettify!(n, 10), "42");
    }

    #[test]
    fn fexp_simple_function_works() {
        let head = FHead::Simple(FunctionSymbol::from("fuel"));
        let e = FExp::new_function(head);
        assert_eq!(prettify!(e, 10), "fuel");
    }

    #[test]
    fn fexp_function_with_terms_works() {
        let head = FHead::WithTerms(
            FunctionSymbol::from("fuel"),
            vec![crate::Term::new_variable(Variable::from("r"))],
        );
        let e = FExp::new_function(head);
        assert_eq!(prettify!(e, 20), "(fuel ?r)");
    }

    #[test]
    fn fexp_negative_works() {
        let inner = FExp::new_number(Number::from(5));
        let e = FExp::new_negative(inner);
        assert_eq!(prettify!(e, 10), "(- 5)");
    }

    #[test]
    fn fexp_binary_op_works() {
        let a = FExp::new_number(Number::from(3));
        let b = FExp::new_number(Number::from(4));
        let e = FExp::new_binary_op(BinaryOp::Addition, a, b);
        assert_eq!(prettify!(e, 10), "(+ 3 4)");
    }

    #[test]
    fn fcomp_works() {
        let a = FExp::new_number(Number::from(3));
        let b = FExp::new_number(Number::from(4));
        let fc = FComp::new(BinaryComp::GreaterThan, a, b);
        assert_eq!(prettify!(fc, 10), "(> 3 4)");
    }

    #[test]
    fn fexp_t_now_works() {
        let e = FExpT::Now;
        assert_eq!(prettify!(e, 10), "now");
    }

    #[test]
    fn fexp_t_scaled_works() {
        let inner = FExp::new_number(Number::from(2));
        let e = FExpT::Scaled(inner);
        assert_eq!(prettify!(e, 10), "2");
    }

    #[test]
    fn fexp_da_duration_works() {
        let e = FExpDa::new_duration();
        assert_eq!(prettify!(e, 10), "#t");
    }

    #[test]
    fn fexp_da_fexp_works() {
        let inner = FExp::new_number(Number::from(10));
        let e = FExpDa::new_f_exp(inner);
        assert_eq!(prettify!(e, 10), "10");
    }

    #[test]
    fn f_assign_da_works() {
        let head = FHead::Simple(FunctionSymbol::from("fuel"));
        let expr = FExpDa::new_f_exp(FExp::new_number(Number::from(5)));
        let assign = FAssignDa::new(AssignOp::Assign, head, expr);
        assert_eq!(prettify!(assign, 20), "(assign fuel 5)");
    }

    #[test]
    fn metric_fexp_number_works() {
        let m = MetricFExp::new_number(Number::from(10));
        assert_eq!(prettify!(m, 10), "10");
    }

    #[test]
    fn metric_fexp_total_time_works() {
        let m = MetricFExp::new_total_time();
        assert_eq!(prettify!(m, 10), "total-time");
    }

    #[test]
    fn metric_fexp_function_works() {
        let m = MetricFExp::new_function(FunctionSymbol::from("fuel"), vec![Name::new("r")]);
        assert_eq!(prettify!(m, 20), "(fuel r)");
    }

    #[test]
    fn metric_fexp_is_violated_works() {
        let m = MetricFExp::new_is_violated(PreferenceName::new_string("p1"));
        assert_eq!(prettify!(m, 20), "(is-violated p1)");
    }
}
