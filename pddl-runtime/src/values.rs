//! Storage for numeric fluents: [`Values`].

use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

/// The values of a numeric fluent of any arity, e.g. `(fuel ?t - truck)`, as
/// one flat table with a stride per argument. A value can be undefined, as
/// PDDL functions are until the initial state or an effect assigns them.
///
/// Values compare and hash by their bits. `-0.0` is stored as `0.0`, and NaN
/// is not a value: [`set`](Self::set) treats it as undefined. So equal values
/// are equal bits, and the table can be part of a hashed state.
///
/// ```
/// use pddl_runtime::Values;
///
/// let mut fuel = Values::new(&[2]);
/// assert_eq!(fuel.get(&[0]), None);
/// fuel.set(&[0], Some(5.0));
/// assert_eq!(fuel.get(&[0]), Some(5.0));
/// ```
#[derive(Clone)]
pub struct Values {
    /// The number of indices per argument.
    dims: Box<[usize]>,
    /// One value per tuple, in row-major order; NaN where undefined.
    data: Box<[f64]>,
}

impl Values {
    /// A table over tuples whose `i`th index is below `dims[i]`, with every
    /// value undefined.
    #[must_use]
    pub fn new(dims: &[usize]) -> Self {
        Self {
            dims: dims.into(),
            data: vec![f64::NAN; dims.iter().product()].into_boxed_slice(),
        }
    }

    /// The value at `tuple`, or `None` if it is undefined.
    ///
    /// # Panics
    /// If `tuple` has the wrong length or an index is out of range.
    #[must_use]
    pub fn get(&self, tuple: &[usize]) -> Option<f64> {
        let value = self.data[self.flat(tuple)];
        (!value.is_nan()).then_some(value)
    }

    /// Sets the value at `tuple`; `None`, NaN and infinities make it undefined.
    ///
    /// # Panics
    /// If `tuple` has the wrong length or an index is out of range.
    pub fn set(&mut self, tuple: &[usize], value: Option<f64>) {
        let i = self.flat(tuple);
        self.data[i] = match value {
            Some(v) if v.is_finite() => v + 0.0, // turns -0.0 into 0.0
            _ => f64::NAN,
        };
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

    /// The canonical bits of each value: one NaN for every undefined value.
    fn bits(&self) -> impl Iterator<Item = u64> + '_ {
        self.data.iter().map(|v| {
            if v.is_nan() {
                f64::NAN.to_bits()
            } else {
                v.to_bits()
            }
        })
    }
}

impl PartialEq for Values {
    fn eq(&self, other: &Self) -> bool {
        self.dims == other.dims && self.bits().eq(other.bits())
    }
}

impl Eq for Values {}

impl Hash for Values {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for bits in self.bits() {
            bits.hash(state);
        }
    }
}

impl Debug for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.data.iter().map(|v| (!v.is_nan()).then_some(v)))
            .finish()
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
    fn values_start_undefined() {
        let mut v = Values::new(&[2, 3]);
        assert_eq!(v.get(&[1, 2]), None);
        v.set(&[1, 2], Some(1.5));
        assert_eq!(v.get(&[1, 2]), Some(1.5));
        v.set(&[1, 2], None);
        assert_eq!(v.get(&[1, 2]), None);
        assert_eq!(format!("{:?}", Values::new(&[1])), "[None]");
    }

    #[test]
    fn signed_zeros_and_nans_hash_equally() {
        let mut a = Values::new(&[]);
        let mut b = Values::new(&[]);
        a.set(&[], Some(0.0));
        b.set(&[], Some(-0.0));
        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));
        a.set(&[], Some(f64::NAN));
        b.set(&[], Some(f64::INFINITY));
        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));
        assert_eq!(a.get(&[]), None);
        b.set(&[], Some(2.0));
        assert_ne!(a, b);
    }
}
