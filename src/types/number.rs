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
/// [`MetricFExp`](crate::MetricFExp) and [`DurationValue`](crate::DurationValue).
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
    /// A new [`Number`] instance if successful or [`NumberError::NotFinite`] if the
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

        return lhs.partial_cmp(&rhs).expect("Values cannot be ambiguous");
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.total_cmp(&other))
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
