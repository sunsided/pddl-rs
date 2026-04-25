use std::ops::Deref;

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{FunctionTyped, FunctionTypedList, Type, Typed, TypedList};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl<T> sealed::Sealed for Typed<T> where T: sealed::Sealed {}
impl<T> sealed::Sealed for TypedList<T> where T: sealed::Sealed {}
impl<T> sealed::Sealed for FunctionTyped<T> where T: sealed::Sealed {}
impl<T> sealed::Sealed for FunctionTypedList<T> where T: sealed::Sealed {}

impl<T> Visitor<Typed<T>, RcDoc<'static>> for PrettyRenderer
where
    T: sealed::Sealed,
    PrettyRenderer: Visitor<T, RcDoc<'static>>,
{
    fn visit(&self, value: &Typed<T>) -> RcDoc<'static> {
        let value_doc = value.value().accept(self);
        if value.type_().eq(&Type::OBJECT) {
            value_doc
        } else {
            value_doc
                .append(RcDoc::text(" - "))
                .append(value.type_().accept(self))
        }
    }
}

impl<T> Visitor<TypedList<T>, RcDoc<'static>> for PrettyRenderer
where
    T: sealed::Sealed + Clone + PartialEq,
    PrettyRenderer: Visitor<T, RcDoc<'static>>,
{
    fn visit(&self, list: &TypedList<T>) -> RcDoc<'static> {
        if list.is_empty() {
            return RcDoc::nil();
        }

        let mut docs = Vec::with_capacity(list.len());
        let mut type_required = false;
        let mut last_type = &Type::OBJECT;

        for item in list.iter().rev() {
            let value_doc = item.value().accept(self);
            if !type_required && item.type_().eq(&Type::OBJECT) {
                docs.push(value_doc);
                continue;
            }

            type_required = true;

            if last_type.eq(item.type_()) {
                docs.push(value_doc);
            } else {
                docs.push(
                    value_doc
                        .append(RcDoc::text(" - "))
                        .append(item.type_().accept(self)),
                );
                last_type = item.type_();
            }
        }

        docs.reverse();
        RcDoc::intersperse(docs, RcDoc::text(" "))
    }
}

impl<T> Visitor<FunctionTyped<T>, RcDoc<'static>> for PrettyRenderer
where
    T: sealed::Sealed,
    PrettyRenderer: Visitor<T, RcDoc<'static>>,
{
    fn visit(&self, value: &FunctionTyped<T>) -> RcDoc<'static> {
        let value_doc = value.value_ref().accept(self);
        let ft: &Type = value.type_ref().deref();
        if ft.eq(&Type::NUMBER) {
            value_doc
        } else {
            value_doc.append(RcDoc::text(" - ")).append(ft.accept(self))
        }
    }
}

impl<T> Visitor<FunctionTypedList<T>, RcDoc<'static>> for PrettyRenderer
where
    T: sealed::Sealed + Clone + PartialEq,
    PrettyRenderer: Visitor<T, RcDoc<'static>>,
{
    fn visit(&self, list: &FunctionTypedList<T>) -> RcDoc<'static> {
        if list.is_empty() {
            return RcDoc::nil();
        }

        let mut docs = Vec::with_capacity(list.len());
        let mut type_required = false;
        let mut last_type: &Type = &Type::NUMBER;

        for item in list.iter().rev() {
            let value_doc = item.value_ref().accept(self);
            let ft: &Type = item.type_ref().deref();
            if !type_required && ft.eq(&Type::NUMBER) {
                docs.push(value_doc);
                continue;
            }

            type_required = true;

            if last_type.eq(ft) {
                docs.push(value_doc);
            } else {
                docs.push(value_doc.append(RcDoc::text(" - ")).append(ft.accept(self)));
                last_type = ft;
            }
        }

        docs.reverse();
        RcDoc::intersperse(docs, RcDoc::text(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{Name, ToTyped, Type, Variable};

    #[test]
    fn typed_object_default() {
        let x = Name::new("x").to_typed(Type::OBJECT);
        assert_eq!(prettify!(x, 10), "x");
    }

    #[test]
    fn typed_explicit() {
        let x = Name::new("x").to_typed(Type::exactly("letter"));
        assert_eq!(prettify!(x, 20), "x - letter");
    }

    #[test]
    fn typed_list_object_default() {
        let list = TypedList::from_iter([
            Name::new("x").to_typed(Type::OBJECT),
            Name::new("y").to_typed(Type::OBJECT),
            Name::new("z").to_typed(Type::OBJECT),
        ]);
        assert_eq!(prettify!(list, 20), "x y z");
    }

    #[test]
    fn typed_list_same_type() {
        let list = TypedList::from_iter([
            Name::new("x").to_typed(Type::exactly("letter")),
            Name::new("y").to_typed(Type::exactly("letter")),
            Name::new("z").to_typed(Type::exactly("letter")),
        ]);
        assert_eq!(prettify!(list, 30), "x y z - letter");
    }

    #[test]
    fn typed_list_interleaved() {
        let list = TypedList::from_iter([
            Name::new("x").to_typed(Type::exactly("letter")),
            Name::new("y").to_typed(Type::exactly("car")),
            Name::new("z").to_typed(Type::exactly("letter")),
        ]);
        assert_eq!(prettify!(list, 30), "x - letter y - car z - letter");
    }

    #[test]
    fn typed_list_object_at_end() {
        let list = TypedList::from_iter([
            Name::new("x").to_typed(Type::exactly("letter")),
            Name::new("y").to_typed(Type::OBJECT),
            Name::new("z").to_typed(Type::OBJECT),
        ]);
        assert_eq!(prettify!(list, 30), "x - letter y z");
    }

    #[test]
    fn typed_list_object_at_start() {
        let list = TypedList::from_iter([
            Variable::string("x").to_typed(Type::OBJECT),
            Variable::string("y").to_typed(Type::OBJECT),
            Variable::string("z").to_typed(Type::exactly("letter")),
        ]);
        assert_eq!(prettify!(list, 30), "?x ?y - object ?z - letter");
    }

    #[test]
    fn typed_list_empty_works() {
        let list = TypedList::<Name>::new(vec![]);
        assert_eq!(prettify!(list, 20), "");
    }

    #[test]
    fn function_typed_number_default() {
        use crate::{FunctionSymbol, FunctionTyped};
        let ft = FunctionTyped::new(FunctionSymbol::from("fuel"), crate::FunctionType::NUMBER);
        assert_eq!(prettify!(ft, 10), "fuel");
    }

    #[test]
    fn function_typed_explicit_type() {
        use crate::{FunctionSymbol, FunctionTyped};
        let ft = FunctionTyped::new(
            FunctionSymbol::from("loc"),
            crate::FunctionType::new(crate::Type::exactly("location")),
        );
        assert_eq!(prettify!(ft, 20), "loc - location");
    }

    #[test]
    fn function_typed_list_empty_works() {
        let list = crate::FunctionTypedList::<crate::AtomicFunctionSkeleton>::new(vec![]);
        assert_eq!(prettify!(list, 20), "");
    }

    #[test]
    fn function_typed_list_same_type() {
        use crate::{AtomicFunctionSkeleton, FunctionSymbol, FunctionTyped};
        let list = crate::FunctionTypedList::new(vec![
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("x"), vec![].into()),
                crate::FunctionType::NUMBER,
            ),
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("y"), vec![].into()),
                crate::FunctionType::NUMBER,
            ),
        ]);
        // Both are number-typed, so type annotation is omitted
        assert_eq!(prettify!(list, 20), "(x) (y)");
    }

    #[test]
    fn function_typed_list_interleaved() {
        use crate::{AtomicFunctionSkeleton, FunctionSymbol, FunctionTyped};
        let list = crate::FunctionTypedList::new(vec![
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("x"), vec![].into()),
                crate::FunctionType::NUMBER,
            ),
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("y"), vec![].into()),
                crate::FunctionType::new(crate::Type::exactly("location")),
            ),
        ]);
        assert_eq!(prettify!(list, 30), "(x) - number (y) - location");
    }

    #[test]
    fn function_typed_list_number_at_end() {
        use crate::{AtomicFunctionSkeleton, FunctionSymbol, FunctionTyped};
        let list = crate::FunctionTypedList::new(vec![
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("x"), vec![].into()),
                crate::FunctionType::new(crate::Type::exactly("location")),
            ),
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("y"), vec![].into()),
                crate::FunctionType::NUMBER,
            ),
        ]);
        assert_eq!(prettify!(list, 30), "(x) - location (y)");
    }

    #[test]
    fn function_typed_list_number_at_start() {
        use crate::{AtomicFunctionSkeleton, FunctionSymbol, FunctionTyped};
        let list = crate::FunctionTypedList::new(vec![
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("x"), vec![].into()),
                crate::FunctionType::NUMBER,
            ),
            FunctionTyped::new(
                AtomicFunctionSkeleton::new(FunctionSymbol::from("y"), vec![].into()),
                crate::FunctionType::new(crate::Type::exactly("location")),
            ),
        ]);
        assert_eq!(prettify!(list, 30), "(x) - number (y) - location");
    }
}
