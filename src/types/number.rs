//! Contains the [`Number`] type.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::ParseFloatError;
use std::ops::Deref;
use std::str::FromStr;

type UnderlyingType = f32;

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
/// assert!(Number::try_new(f32::NAN).is_err());
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
    pub fn new(value: UnderlyingType) -> Self {
        assert!(value.is_finite(), "The value must be finite");
        Self(value)
    }

    /// Attempts to construct a new number from the provided value.
    ///
    /// ## Arguments
    /// * `value` - A finite value to construct the instance from.
    ///
    /// ## Returns
    /// A new [`Number`] instance if successful or an error if the
    /// input was not a finite number.
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
        match UnderlyingType::from_str(s) {
            Ok(x) => Ok(Number::new(x)),
            Err(e) => Err(NumberError::InvalidFormat(e)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NumberError {
    NotFinite,
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
        let value: Number = (*other).into();
        self.total_cmp(&value) == Ordering::Equal
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.0;
        let rhs = other.0;
        debug_assert!(lhs.is_finite());
        debug_assert!(rhs.is_finite());

        // Ensure plus/minus 0 is equal.
        if lhs == 0.0 && rhs == -0.0 || lhs == -0.0 && rhs == 0.0 {
            return Ordering::Equal;
        }

        // Treat NaN as equal.
        // Should never happen according to construction rules.
        if lhs.is_nan() || rhs.is_nan() {
            unreachable!("Proper construction of this type prevents NaN");
        }

        // Treat infinities as equal.
        // Should never happen according to construction rules.
        if lhs.is_infinite() || rhs.is_infinite() {
            unreachable!("Proper construction of this type prevents NaN");
        }

        lhs.partial_cmp(&rhs).expect("Values cannot be ambiguous")
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
            state.write_u32(0)
        } else {
            state.write_u32(self.0.to_bits())
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
        Number::new(value as UnderlyingType)
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::new(value as UnderlyingType)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::new(value as UnderlyingType)
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::new(value as UnderlyingType)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::new(value as UnderlyingType)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::new(value as UnderlyingType)
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

impl From<f64> for Number {
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
    fn from(value: f64) -> Self {
        Number::new(value as UnderlyingType)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn new_works() {
        let n = Number::new(42.0);
        assert_eq!(*n, 42.0);
    }

    #[test]
    fn try_new_finite_ok() {
        assert!(Number::try_new(0.0).is_ok());
        assert!(Number::try_new(-1.5).is_ok());
        assert!(Number::try_new(f32::MAX).is_ok());
        assert!(Number::try_new(f32::MIN).is_ok());
    }

    #[test]
    fn try_new_nan_err() {
        assert!(matches!(
            Number::try_new(f32::NAN),
            Err(NumberError::NotFinite)
        ));
    }

    #[test]
    fn try_new_inf_err() {
        assert!(matches!(
            Number::try_new(f32::INFINITY),
            Err(NumberError::NotFinite)
        ));
        assert!(matches!(
            Number::try_new(f32::NEG_INFINITY),
            Err(NumberError::NotFinite)
        ));
    }

    #[test]
    fn from_str_valid() {
        let n: Number = "2.5".parse().unwrap();
        assert_eq!(*n, 2.5);
    }

    #[test]
    fn from_str_invalid() {
        let err = "not_a_number".parse::<Number>().unwrap_err();
        assert!(matches!(err, NumberError::InvalidFormat(_)));
    }

    #[test]
    fn error_display_not_finite() {
        let err = NumberError::NotFinite;
        assert_eq!(format!("{err}"), "The input was not a finite number");
    }

    #[test]
    fn error_display_invalid_format() {
        let err = NumberError::InvalidFormat("abc".parse::<f32>().unwrap_err());
        let msg = format!("{err}");
        assert!(msg.contains("Invalid format"));
    }

    #[test]
    fn eq_int() {
        let n = Number::from(42i32);
        assert_eq!(n, 42);
        assert_eq!(n, 42.0);
    }

    #[test]
    fn eq_from_i8() {
        let n = Number::from(5i8);
        assert_eq!(*n, 5.0);
    }

    #[test]
    fn eq_from_u8() {
        let n = Number::from(5u8);
        assert_eq!(*n, 5.0);
    }

    #[test]
    fn eq_from_i16() {
        let n = Number::from(5i16);
        assert_eq!(*n, 5.0);
    }

    #[test]
    fn eq_from_u16() {
        let n = Number::from(5u16);
        assert_eq!(*n, 5.0);
    }

    #[test]
    fn eq_from_i32() {
        let n = Number::from(5i32);
        assert_eq!(*n, 5.0);
    }

    #[test]
    fn eq_from_u32() {
        let n = Number::from(5u32);
        assert_eq!(*n, 5.0);
    }

    #[test]
    fn eq_from_f32() {
        let n = Number::from(2.5f32);
        assert_eq!(*n, 2.5);
    }

    #[test]
    fn eq_from_f64() {
        let n = Number::from(2.5f64);
        assert!((*n - 2.5).abs() < 1e-6);
    }

    #[test]
    fn ord_works() {
        let a = Number::from(1i32);
        let b = Number::from(2i32);
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn ord_equal() {
        let a = Number::from(5i32);
        let b = Number::from(5i32);
        assert!(a <= b);
        assert!(a >= b);
    }

    #[test]
    fn ord_plus_minus_zero_equal() {
        let a = Number::new(0.0);
        let b = Number::new(-0.0);
        assert_eq!(a.cmp(&b), Ordering::Equal);
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
        assert_eq!(ha.finish(), hb.finish());
    }

    #[test]
    fn default_is_zero() {
        let n = Number::default();
        assert_eq!(*n, 0.0);
    }

    #[test]
    fn deref_works() {
        let n = Number::from(7i32);
        let val: &f32 = &n;
        assert_eq!(*val, 7.0);
    }

    #[test]
    fn new_nan_panics() {
        assert!(panic::catch_unwind(|| Number::new(f32::NAN)).is_err());
    }

    #[test]
    fn from_f32_nan_panics() {
        assert!(panic::catch_unwind(|| Number::from(f32::NAN)).is_err());
    }

    #[test]
    fn from_f64_nan_panics() {
        assert!(panic::catch_unwind(|| Number::from(f64::NAN)).is_err());
    }

    #[test]
    fn copy_clone() {
        let a = Number::from(10i32);
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn debug_impl() {
        let n = Number::from(42i32);
        let dbg = format!("{n:?}");
        assert!(dbg.contains("42"));
    }
}
