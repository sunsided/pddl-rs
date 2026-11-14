//! Contains the [`LengthSpec`] type.

/// Deprecated since PDDL 2.1.
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct LengthSpec {
    /// The maximum serial plan length, `(:serial <integer>)`.
    serial: Option<u64>,
    /// The maximum parallel plan length, `(:parallel <integer>)`.
    parallel: Option<u64>,
}

impl LengthSpec {
    /// Creates a length specification `(:length [(:serial n)] [(:parallel n)])`.
    pub const fn new(serial: Option<u64>, parallel: Option<u64>) -> Self {
        Self { serial, parallel }
    }

    /// Creates a length specification with only a serial length.
    pub const fn new_serial(serial: u64) -> Self {
        Self::new(Some(serial), None)
    }

    /// Creates a length specification with only a parallel length.
    pub const fn new_parallel(parallel: u64) -> Self {
        Self::new(None, Some(parallel))
    }

    /// Gets the serial value.
    pub const fn serial(&self) -> Option<u64> {
        self.serial
    }

    /// Gets the parallel value.
    pub const fn parallel(&self) -> Option<u64> {
        self.parallel
    }
}
