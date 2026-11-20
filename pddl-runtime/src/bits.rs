//! Bit-packed fact storage: [`BitSet`], [`BitRelation`] and [`BitTable`].

use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::Id;

/// The bits of a fixed number of facts, packed into words.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
struct Bits(Box<[u64]>);

impl Bits {
    /// `len` bits, all clear.
    fn new(len: usize) -> Self {
        Self(vec![0; len.div_ceil(64)].into_boxed_slice())
    }

    /// Whether bit `i` is set.
    fn get(&self, i: usize) -> bool {
        self.0[i / 64] & (1 << (i % 64)) != 0
    }

    /// Sets bit `i` to `value` and returns whether it changed.
    fn set(&mut self, i: usize, value: bool) -> bool {
        let word = &mut self.0[i / 64];
        let mask = 1 << (i % 64);
        let old = *word & mask != 0;
        if value {
            *word |= mask;
        } else {
            *word &= !mask;
        }
        old != value
    }

    /// The number of set bits.
    fn count(&self) -> usize {
        self.0.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Clears every bit.
    fn clear(&mut self) {
        self.0.fill(0);
    }

    /// The indices of the set bits, ascending.
    fn ones(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().enumerate().flat_map(|(w, &word)| {
            let mut word = word;
            std::iter::from_fn(move || {
                (word != 0).then(|| {
                    let bit = word.trailing_zeros() as usize;
                    word &= word - 1;
                    w * 64 + bit
                })
            })
        })
    }
}

/// The facts of a unary predicate: a set of ids of one type, with a fixed
/// capacity.
///
/// ```
/// use pddl_runtime::BitSet;
///
/// let mut clear: BitSet<usize> = BitSet::new(3);
/// assert!(clear.insert(2));
/// assert!(clear.contains(2) && !clear.contains(0));
/// assert_eq!(clear.iter().collect::<Vec<_>>(), [2]);
/// ```
pub struct BitSet<T> {
    /// The capacity.
    len: usize,
    /// One bit per id.
    bits: Bits,
    /// The id type; `fn() -> T` keeps the set `Send` and `Sync` for any `T`.
    ids: PhantomData<fn() -> T>,
}

impl<T: Id> BitSet<T> {
    /// An empty set for the ids `0..len`.
    #[must_use]
    pub fn new(len: usize) -> Self {
        Self {
            len,
            bits: Bits::new(len),
            ids: PhantomData,
        }
    }

    /// The number of ids the set can hold.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.len
    }

    /// Whether the set contains `id`.
    ///
    /// # Panics
    /// If `id` is not below the capacity.
    #[must_use]
    pub fn contains(&self, id: T) -> bool {
        self.bits.get(self.check(id))
    }

    /// Adds `id`; returns whether it was not in the set.
    ///
    /// # Panics
    /// If `id` is not below the capacity.
    pub fn insert(&mut self, id: T) -> bool {
        let i = self.check(id);
        self.bits.set(i, true)
    }

    /// Removes `id`; returns whether it was in the set.
    ///
    /// # Panics
    /// If `id` is not below the capacity.
    pub fn remove(&mut self, id: T) -> bool {
        let i = self.check(id);
        self.bits.set(i, false)
    }

    /// Adds or removes `id`.
    ///
    /// # Panics
    /// If `id` is not below the capacity.
    pub fn set(&mut self, id: T, value: bool) {
        let i = self.check(id);
        self.bits.set(i, value);
    }

    /// The number of ids in the set.
    #[must_use]
    pub fn count(&self) -> usize {
        self.bits.count()
    }

    /// Whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.0.iter().all(|&w| w == 0)
    }

    /// Removes every id.
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    /// The ids in the set, ascending.
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.bits.ones().map(T::from_index)
    }

    /// The index of `id`, checked against the capacity.
    fn check(&self, id: T) -> usize {
        let i = id.index();
        assert!(
            i < self.len,
            "{id:?} is out of range for a set of {}",
            self.len
        );
        i
    }
}

impl<T> Clone for BitSet<T> {
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            bits: self.bits.clone(),
            ids: PhantomData,
        }
    }
}

impl<T> PartialEq for BitSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.bits == other.bits
    }
}

impl<T> Eq for BitSet<T> {}

impl<T> Hash for BitSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}

impl<T: Id> Debug for BitSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

/// The facts of a binary predicate: a set of id pairs, with fixed capacities.
///
/// ```
/// use pddl_runtime::BitRelation;
///
/// let mut on: BitRelation<usize, usize> = BitRelation::new(3, 3);
/// on.insert(0, 1);
/// assert!(on.contains(0, 1) && !on.contains(1, 0));
/// assert_eq!(on.iter().collect::<Vec<_>>(), [(0, 1)]);
/// ```
pub struct BitRelation<A, B> {
    /// The number of `A` ids.
    rows: usize,
    /// The number of `B` ids.
    cols: usize,
    /// One bit per pair, row by row.
    bits: Bits,
    /// The id types.
    ids: PhantomData<fn() -> (A, B)>,
}

impl<A: Id, B: Id> BitRelation<A, B> {
    /// An empty relation over the ids `0..rows` and `0..cols`.
    #[must_use]
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            bits: Bits::new(rows * cols),
            ids: PhantomData,
        }
    }

    /// Whether the relation contains `(a, b)`.
    ///
    /// # Panics
    /// If `a` or `b` is out of range.
    #[must_use]
    pub fn contains(&self, a: A, b: B) -> bool {
        self.bits.get(self.check(a, b))
    }

    /// Adds `(a, b)`; returns whether it was not in the relation.
    ///
    /// # Panics
    /// If `a` or `b` is out of range.
    pub fn insert(&mut self, a: A, b: B) -> bool {
        let i = self.check(a, b);
        self.bits.set(i, true)
    }

    /// Removes `(a, b)`; returns whether it was in the relation.
    ///
    /// # Panics
    /// If `a` or `b` is out of range.
    pub fn remove(&mut self, a: A, b: B) -> bool {
        let i = self.check(a, b);
        self.bits.set(i, false)
    }

    /// Adds or removes `(a, b)`.
    ///
    /// # Panics
    /// If `a` or `b` is out of range.
    pub fn set(&mut self, a: A, b: B, value: bool) {
        let i = self.check(a, b);
        self.bits.set(i, value);
    }

    /// The number of pairs in the relation.
    #[must_use]
    pub fn count(&self) -> usize {
        self.bits.count()
    }

    /// Removes every pair.
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    /// The pairs in the relation, ordered by `a`, then `b`.
    pub fn iter(&self) -> impl Iterator<Item = (A, B)> + '_ {
        self.bits
            .ones()
            .map(|i| (A::from_index(i / self.cols), B::from_index(i % self.cols)))
    }

    /// The flat index of `(a, b)`, checked against the capacities.
    fn check(&self, a: A, b: B) -> usize {
        let (i, j) = (a.index(), b.index());
        assert!(
            i < self.rows && j < self.cols,
            "({a:?}, {b:?}) is out of range for a {}x{} relation",
            self.rows,
            self.cols
        );
        i * self.cols + j
    }
}

impl<A, B> Clone for BitRelation<A, B> {
    fn clone(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            bits: self.bits.clone(),
            ids: PhantomData,
        }
    }
}

impl<A, B> PartialEq for BitRelation<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.bits == other.bits
    }
}

impl<A, B> Eq for BitRelation<A, B> {}

impl<A, B> Hash for BitRelation<A, B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}

impl<A: Id, B: Id> Debug for BitRelation<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

/// The facts of a predicate of any arity: a set of index tuples, stored as one
/// flat bit set with a stride per argument.
///
/// Arguments are plain indices; generated code converts its typed ids.
///
/// ```
/// use pddl_runtime::BitTable;
///
/// let mut between = BitTable::new(&[2, 3, 4]);
/// between.insert(&[1, 2, 3]);
/// assert!(between.contains(&[1, 2, 3]) && !between.contains(&[0, 2, 3]));
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitTable {
    /// The number of indices per argument.
    dims: Box<[usize]>,
    /// One bit per tuple, in row-major order.
    bits: Bits,
}

impl BitTable {
    /// An empty table over tuples whose `i`th index is below `dims[i]`.
    #[must_use]
    pub fn new(dims: &[usize]) -> Self {
        Self {
            dims: dims.into(),
            bits: Bits::new(dims.iter().product()),
        }
    }

    /// Whether the table contains `tuple`.
    ///
    /// # Panics
    /// If `tuple` has the wrong length or an index is out of range.
    #[must_use]
    pub fn contains(&self, tuple: &[usize]) -> bool {
        self.bits.get(self.flat(tuple))
    }

    /// Adds `tuple`; returns whether it was not in the table.
    ///
    /// # Panics
    /// If `tuple` has the wrong length or an index is out of range.
    pub fn insert(&mut self, tuple: &[usize]) -> bool {
        let i = self.flat(tuple);
        self.bits.set(i, true)
    }

    /// Removes `tuple`; returns whether it was in the table.
    ///
    /// # Panics
    /// If `tuple` has the wrong length or an index is out of range.
    pub fn remove(&mut self, tuple: &[usize]) -> bool {
        let i = self.flat(tuple);
        self.bits.set(i, false)
    }

    /// Adds or removes `tuple`.
    ///
    /// # Panics
    /// If `tuple` has the wrong length or an index is out of range.
    pub fn set(&mut self, tuple: &[usize], value: bool) {
        let i = self.flat(tuple);
        self.bits.set(i, value);
    }

    /// The number of tuples in the table.
    #[must_use]
    pub fn count(&self) -> usize {
        self.bits.count()
    }

    /// Removes every tuple.
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    /// The tuples in the table, in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = Vec<usize>> + '_ {
        self.bits.ones().map(|mut i| {
            let mut tuple = vec![0; self.dims.len()];
            for (slot, &dim) in tuple.iter_mut().zip(self.dims.iter()).rev() {
                *slot = i % dim;
                i /= dim;
            }
            tuple
        })
    }

    /// The flat index of `tuple`, checked against the dimensions.
    fn flat(&self, tuple: &[usize]) -> usize {
        assert_eq!(tuple.len(), self.dims.len(), "wrong number of arguments");
        tuple
            .iter()
            .zip(self.dims.iter())
            .fold(0, |flat, (&i, &dim)| {
                assert!(i < dim, "index {i} is out of range for {dim} objects");
                flat * dim + i
            })
    }
}

impl Debug for BitTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    /// The hash of `value` with a fixed hasher.
    fn hash(value: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn bit_set_inserts_removes_and_iterates() {
        let mut set: BitSet<usize> = BitSet::new(130);
        assert!(set.is_empty());
        assert!(set.insert(0) && set.insert(64) && set.insert(129));
        assert!(!set.insert(64));
        assert_eq!(set.iter().collect::<Vec<_>>(), [0, 64, 129]);
        assert_eq!(set.count(), 3);
        assert!(set.remove(64) && !set.remove(64));
        assert!(!set.contains(64) && set.contains(129));
        set.set(5, true);
        assert_eq!(format!("{set:?}"), "{0, 5, 129}");
        assert_eq!(set.capacity(), 130);
        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn bit_set_rejects_ids_beyond_its_capacity() {
        let set: BitSet<usize> = BitSet::new(3);
        let _ = set.contains(3);
    }

    #[test]
    fn equal_sets_hash_equally() {
        let mut a: BitSet<u32> = BitSet::new(70);
        let mut b: BitSet<u32> = BitSet::new(70);
        a.insert(3);
        a.insert(69);
        b.insert(69);
        assert_ne!(a, b);
        b.insert(3);
        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));
        let c = a.clone();
        a.remove(3);
        assert_ne!(a, c);
    }

    #[test]
    fn relation_keeps_pairs_apart() {
        let mut on: BitRelation<usize, u32> = BitRelation::new(3, 5);
        assert!(on.insert(1, 4));
        assert!(!on.contains(2, 1) && on.contains(1, 4));
        on.set(2, 0, true);
        assert_eq!(on.iter().collect::<Vec<_>>(), [(1, 4), (2, 0)]);
        assert_eq!(format!("{on:?}"), "{(1, 4), (2, 0)}");
        assert_eq!(on.count(), 2);
        let copy = on.clone();
        assert_eq!(hash(&copy), hash(&on));
        assert!(on.remove(1, 4));
        assert_ne!(copy, on);
        on.clear();
        assert_eq!(on.count(), 0);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn relation_rejects_pairs_beyond_its_capacity() {
        let mut on: BitRelation<usize, usize> = BitRelation::new(2, 3);
        on.insert(0, 3);
    }

    #[test]
    fn table_handles_any_arity() {
        let mut table = BitTable::new(&[2, 3, 4]);
        assert!(table.insert(&[1, 2, 3]) && table.insert(&[0, 0, 1]));
        assert!(!table.insert(&[1, 2, 3]));
        assert_eq!(
            table.iter().collect::<Vec<_>>(),
            [vec![0, 0, 1], vec![1, 2, 3]]
        );
        assert_eq!(format!("{table:?}"), "{[0, 0, 1], [1, 2, 3]}");
        let copy = table.clone();
        assert_eq!(hash(&copy), hash(&table));
        assert!(table.remove(&[0, 0, 1]));
        assert_ne!(copy, table);
        table.set(&[0, 1, 2], true);
        assert_eq!(table.count(), 2);
        table.clear();
        assert_eq!(table.count(), 0);

        let mut nullary = BitTable::new(&[]);
        assert!(!nullary.contains(&[]));
        nullary.insert(&[]);
        assert_eq!(nullary.iter().collect::<Vec<_>>(), [Vec::<usize>::new()]);
    }

    #[test]
    #[should_panic(expected = "wrong number of arguments")]
    fn table_rejects_the_wrong_arity() {
        let table = BitTable::new(&[2, 2]);
        let _ = table.contains(&[1]);
    }
}
