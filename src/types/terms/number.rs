//! Contains the [`Number`] type.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::ParseFloatError;
use std::ops::Deref;
use std::str::FromStr;

/// The primitive type backing a [`Number`].
type UnderlyingType = f64;

/// A number.
///
/// ## Examples
/// ```
/// # use std::panic;
/// # use pddl::Number;
/// let number = Number::from(42);
/// assert_eq!(number, 42);
/// assert_eq!(number, 42.0);
///
/// // Only finite values are allowed.
/// assert!(Number::try_new(f64::NAN).is_err());
/// assert!(panic::catch_unwind(|| Number::from(f32::NAN)).is_err());
/// ```
///
/// ## Usage
/// Used by [`InitElement`](crate::InitElement), [`ConGD`](crate::ConGD),
/// [`MetricFluentExpression`](crate::MetricFluentExpression) and [`DurationValue`](crate::DurationValue).
#[derive(Debug, Copy, Clone, Default)]
pub struct Number(UnderlyingType);

impl Number {
    /// Constructs a new number from the provided value.
    ///
    /// ## Arguments
    /// * `value` - A finite value to construct the instance from.
    ///
    /// ## Returns
    /// A new [`Number`] instance.
    ///
    /// ## Panics
    /// Panics if the provided `value` is not finite. Use [`Number::try_new`]
    /// if you need the function to not panic.
    #[must_use]
    pub fn new(value: UnderlyingType) -> Self {
        assert!(value.is_finite(), "The value must be finite");
        Self(value)
    }

    /// Attempts to construct a new number from the provided value.
    ///
    /// ## Arguments
    /// * `value` - A finite value to construct the instance from.
    ///
    /// ## Errors
    /// Returns `NumberError::NotFinite` if `value` is NaN or infinite.
    pub fn try_new(value: UnderlyingType) -> Result<Self, NumberError> {
        if !value.is_finite() {
            return Err(NumberError::NotFinite);
        }

        Ok(Self(value))
    }
}

impl FromStr for Number {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = UnderlyingType::from_str(s).map_err(NumberError::InvalidFormat)?;
        Number::try_new(value)
    }
}

/// An error constructing or parsing a [`Number`].
#[derive(Debug, Clone)]
pub enum NumberError {
    /// The value was NaN or infinite.
    NotFinite,
    /// The string was not a valid floating-point number.
    InvalidFormat(ParseFloatError),
}

impl Display for NumberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberError::InvalidFormat(e) => write!(f, "Invalid format: {e}"),
            NumberError::NotFinite => write!(f, "The input was not a finite number"),
        }
    }
}

impl Error for NumberError {}

impl Eq for Number {}

impl<I: Into<Number> + Copy> PartialEq<I> for Number {
    fn eq(&self, other: &I) -> bool {
        // Plain `f64` equality: `-0.0 == 0.0`, as in `Ord` and `Hash`.
        self.0 == (*other).into().0
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        debug_assert!(self.0.is_finite() && other.0.is_finite());
        // `partial_cmp` treats `-0.0` and `0.0` as equal, and is only `None` for NaN.
        self.0
            .partial_cmp(&other.0)
            .expect("Number is finite by construction")
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        debug_assert!(self.0.is_finite());
        if self.0 == 0.0 || self.0 == -0.0 {
            state.write_u64(0);
        } else {
            state.write_u64(self.0.to_bits());
        }
    }
}

impl Deref for Number {
    type Target = UnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::new(UnderlyingType::from(value))
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::new(UnderlyingType::from(value))
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::new(UnderlyingType::from(value))
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::new(UnderlyingType::from(value))
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::new(UnderlyingType::from(value))
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::new(UnderlyingType::from(value))
    }
}

impl From<UnderlyingType> for Number {
    /// Constructs a new number from the provided value.
    ///
    /// ## Arguments
    /// * `value` - A finite value to construct the instance from.
    ///
    /// ## Returns
    /// A new [`Number`] instance.
    ///
    /// ## Panics
    /// Panics if the provided `value` is not finite. Use [`Number::try_new`]
    /// if you need the function to not panic.
    fn from(value: UnderlyingType) -> Self {
        Number::new(value)
    }
}

impl From<f32> for Number {
    /// Constructs a new number from the provided value.
    ///
    /// ## Arguments
    /// * `value` - A finite value to construct the instance from.
    ///
    /// ## Returns
    /// A new [`Number`] instance.
    ///
    /// ## Panics
    /// Panics if the provided `value` is not finite. Use [`Number::try_new`]
    /// if you need the function to not panic.
    fn from(value: f32) -> Self {
        // Through the shortest decimal, so `1.1f32` becomes `1.1`, not `1.100000023841858`.
        let value: f64 = value.to_string().parse().expect("f32 text is a valid f64");
        Number::new(value)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp, reason = "the tests check exact values")]
mod tests {
    use super::*;
    use assert2::check;
    use std::panic;

    #[test]
    fn from_str_rejects_infinity() {
        check!(let Err(NumberError::NotFinite) = "inf".parse::<Number>());
    }

    #[test]
    fn signed_zeros_are_equal() {
        use std::collections::hash_map::DefaultHasher;
        let (neg, pos) = (Number::from(-0.0f32), Number::from(0.0f32));
        let hash = |n: Number| {
            let mut h = DefaultHasher::new();
            n.hash(&mut h);
            h.finish()
        };
        check!(neg == pos);
        check!(neg.cmp(&pos) == Ordering::Equal);
        check!(hash(neg) == hash(pos));
    }

    #[test]
    fn new_works() {
        let n = Number::new(42.0);
        check!(*n == 42.0);
    }

    #[test]
    fn try_new_finite_ok() {
        check!(Number::try_new(0.0).is_ok());
        check!(Number::try_new(-1.5).is_ok());
        check!(Number::try_new(f64::MAX).is_ok());
        check!(Number::try_new(f64::MIN).is_ok());
    }

    #[test]
    fn try_new_nan_err() {
        check!(let Err(NumberError::NotFinite) = Number::try_new(f64::NAN));
    }

    #[test]
    fn try_new_inf_err() {
        check!(let Err(NumberError::NotFinite) = Number::try_new(f64::INFINITY));
        check!(let Err(NumberError::NotFinite) = Number::try_new(f64::NEG_INFINITY));
    }

    #[test]
    fn from_str_valid() {
        let n: Number = "2.5".parse().unwrap();
        check!(*n == 2.5);
    }

    #[test]
    fn from_str_invalid() {
        let err = "not_a_number".parse::<Number>().unwrap_err();
        check!(let NumberError::InvalidFormat(_) = err);
    }

    #[test]
    fn error_display_not_finite() {
        let err = NumberError::NotFinite;
        check!(format!("{err}") == "The input was not a finite number");
    }

    #[test]
    fn error_display_invalid_format() {
        let err = NumberError::InvalidFormat("abc".parse::<f64>().unwrap_err());
        let msg = format!("{err}");
        check!(msg.contains("Invalid format"));
    }

    #[test]
    fn eq_int() {
        let n = Number::from(42i32);
        check!(n == 42);
        check!(n == 42.0);
    }

    #[test]
    fn eq_from_i8() {
        let n = Number::from(5i8);
        check!(*n == 5.0);
    }

    #[test]
    fn eq_from_u8() {
        let n = Number::from(5u8);
        check!(*n == 5.0);
    }

    #[test]
    fn eq_from_i16() {
        let n = Number::from(5i16);
        check!(*n == 5.0);
    }

    #[test]
    fn eq_from_u16() {
        let n = Number::from(5u16);
        check!(*n == 5.0);
    }

    #[test]
    fn eq_from_i32() {
        let n = Number::from(5i32);
        check!(*n == 5.0);
    }

    #[test]
    fn eq_from_u32() {
        let n = Number::from(5u32);
        check!(*n == 5.0);
    }

    #[test]
    fn eq_from_f32() {
        let n = Number::from(2.5f32);
        check!(*n == 2.5);
    }

    #[test]
    fn eq_from_f64() {
        let n = Number::from(2.5f64);
        check!(*n == 2.5);
    }

    #[test]
    fn ord_works() {
        let a = Number::from(1i32);
        let b = Number::from(2i32);
        check!(a < b);
        check!(b > a);
    }

    #[test]
    fn ord_equal() {
        let a = Number::from(5i32);
        let b = Number::from(5i32);
        check!(a <= b);
        check!(a >= b);
    }

    #[test]
    fn ord_plus_minus_zero_equal() {
        let a = Number::new(0.0);
        let b = Number::new(-0.0);
        check!(a.cmp(&b) == Ordering::Equal);
    }

    #[test]
    fn hash_zero_equals_neg_zero() {
        use std::collections::hash_map::DefaultHasher;
        let a = Number::new(0.0);
        let b = Number::new(-0.0);
        let mut ha = DefaultHasher::new();
        let mut hb = DefaultHasher::new();
        a.hash(&mut ha);
        b.hash(&mut hb);
        check!(ha.finish() == hb.finish());
    }

    #[test]
    fn default_is_zero() {
        let n = Number::default();
        check!(*n == 0.0);
    }

    #[test]
    fn deref_works() {
        let n = Number::from(7i32);
        let val: &f64 = &n;
        check!(*val == 7.0);
    }

    #[test]
    fn new_nan_panics() {
        check!(panic::catch_unwind(|| Number::new(f64::NAN)).is_err());
    }

    #[test]
    fn from_f32_nan_panics() {
        check!(panic::catch_unwind(|| Number::from(f32::NAN)).is_err());
    }

    #[test]
    fn from_f64_keeps_range_and_whole_number_precision() {
        // As f32, 1e300 overflowed to infinity and panicked; 16777217 became 16777216.
        check!(*Number::from(1e300) == 1e300);
        check!(*Number::from(16_777_217) == 16_777_217.0);
        check!((*"16777217".parse::<Number>().unwrap()) == 16_777_217.0);
    }

    #[test]
    fn from_f32_keeps_the_decimal_written() {
        // Widening the bits would give 1.100000023841858.
        check!(*Number::from(1.1f32) == 1.1);
        check!(Number::from(1.1f64) == 1.1f32);
        check!(*Number::from(f32::MAX) == 3.402_823_5e38);
    }

    #[test]
    fn from_f64_nan_panics() {
        check!(panic::catch_unwind(|| Number::from(f64::NAN)).is_err());
    }

    #[test]
    fn copy_clone() {
        let a = Number::from(10i32);
        let b = a;
        check!(a == b);
    }

    #[test]
    fn debug_impl() {
        let n = Number::from(42i32);
        let dbg = format!("{n:?}");
        check!(dbg.contains("42"));
    }
}
