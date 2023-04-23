/// A visitor.
pub trait Visitor<T, O> {
    fn visit(&self, value: &T) -> O;
}

/// A mutable visitor.
pub trait VisitorMut<T, O> {
    fn visit_mut(&mut self, value: &T) -> O;
}

/// Trait to accept immutable visitors.
pub trait Accept<V: Visitor<T, O>, T, O> {
    fn accept(&self, v: &V) -> O;
}

/// Trait to accept mutable visitors.
pub trait AcceptMut<V: VisitorMut<T, O>, T, O> {
    fn accept_mut(&self, v: &mut V) -> O;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Visi(isize);
    struct Item(i32);

    impl VisitorMut<Item, ()> for Visi {
        fn visit_mut(&mut self, value: &Item) -> () {
            self.0 += value.0 as isize;
        }
    }

    impl AcceptMut<Visi, Item, ()> for Item {
        fn accept_mut(&self, v: &mut Visi) -> () {
            v.visit_mut(self)
        }
    }

    #[test]
    fn it_works() {
        let a = Item(10);
        let b = Item(7);
        let mut v = Visi(20);

        a.accept_mut(&mut v);
        b.accept_mut(&mut v);

        assert_eq!(v.0, 37);
    }
}
