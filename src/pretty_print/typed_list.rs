use std::ops::Deref;

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{FunctionTyped, FunctionTypedList, Type, Typed, TypedList};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl<T> sealed::Sealed for Typed<T> where T: sealed::Sealed {}
impl<T> sealed::Sealed for TypedList<T> where T: sealed::Sealed {}
impl<T> sealed::Sealed for FunctionTyped<T> where T: sealed::Sealed {}
impl<T> sealed::Sealed for FunctionTypedList<T> where T: sealed::Sealed {}

impl<'a, T> Visitor<Typed<T>, RcDoc<'a>> for PrettyRenderer
where
    T: sealed::Sealed,
    PrettyRenderer: Visitor<T, RcDoc<'a>>,
{
    fn visit(&self, value: &Typed<T>) -> RcDoc<'a> {
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

impl<'a, T> Visitor<TypedList<T>, RcDoc<'a>> for PrettyRenderer
where
    T: sealed::Sealed + Clone + PartialEq,
    PrettyRenderer: Visitor<T, RcDoc<'a>>,
{
    fn visit(&self, list: &TypedList<T>) -> RcDoc<'a> {
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

impl<'a, T> Visitor<FunctionTyped<T>, RcDoc<'a>> for PrettyRenderer
where
    T: sealed::Sealed,
    PrettyRenderer: Visitor<T, RcDoc<'a>>,
{
    fn visit(&self, value: &FunctionTyped<T>) -> RcDoc<'a> {
        let value_doc = value.value_ref().accept(self);
        let ft: &Type = value.type_ref().deref();
        if ft.eq(&Type::NUMBER) {
            value_doc
        } else {
            value_doc.append(RcDoc::text(" - ")).append(ft.accept(self))
        }
    }
}

impl<'a, T> Visitor<FunctionTypedList<T>, RcDoc<'a>> for PrettyRenderer
where
    T: sealed::Sealed + Clone + PartialEq,
    PrettyRenderer: Visitor<T, RcDoc<'a>>,
{
    fn visit(&self, list: &FunctionTypedList<T>) -> RcDoc<'a> {
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
        let x = Name::new("x").to_typed(Type::new_exactly("letter"));
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
            Name::new("x").to_typed(Type::new_exactly("letter")),
            Name::new("y").to_typed(Type::new_exactly("letter")),
            Name::new("z").to_typed(Type::new_exactly("letter")),
        ]);
        assert_eq!(prettify!(list, 30), "x y z - letter");
    }

    #[test]
    fn typed_list_interleaved() {
        let list = TypedList::from_iter([
            Name::new("x").to_typed(Type::new_exactly("letter")),
            Name::new("y").to_typed(Type::new_exactly("car")),
            Name::new("z").to_typed(Type::new_exactly("letter")),
        ]);
        assert_eq!(prettify!(list, 30), "x - letter y - car z - letter");
    }

    #[test]
    fn typed_list_object_at_end() {
        let list = TypedList::from_iter([
            Name::new("x").to_typed(Type::new_exactly("letter")),
            Name::new("y").to_typed(Type::OBJECT),
            Name::new("z").to_typed(Type::OBJECT),
        ]);
        assert_eq!(prettify!(list, 30), "x - letter y z");
    }

    #[test]
    fn typed_list_object_at_start() {
        let list = TypedList::from_iter([
            Variable::new_string("x").to_typed(Type::OBJECT),
            Variable::new_string("y").to_typed(Type::OBJECT),
            Variable::new_string("z").to_typed(Type::new_exactly("letter")),
        ]);
        assert_eq!(prettify!(list, 30), "?x ?y - object ?z - letter");
    }
}
