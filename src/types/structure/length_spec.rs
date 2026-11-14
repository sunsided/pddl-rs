//! Contains the [`LengthSpec`] type.

/// Deprecated since PDDL 2.1.
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LengthSpec {
    /// The expected number of steps of a sequential plan, `(:serial <integer>)`; a hint to the planner.
    serial: Option<u64>,
    /// The expected number of parallel steps of a plan, `(:parallel <integer>)`; a hint to the planner.
    parallel: Option<u64>,
}

impl LengthSpec {
    /// Creates a length specification `(:length [(:serial n)] [(:parallel n)])`.
    #[must_use]
    pub const fn new(serial: Option<u64>, parallel: Option<u64>) -> Self {
        Self { serial, parallel }
    }

    /// Creates a length specification with only a serial length.
    #[must_use]
    pub const fn new_serial(serial: u64) -> Self {
        Self::new(Some(serial), None)
    }

    /// Creates a length specification with only a parallel length.
    #[must_use]
    pub const fn new_parallel(parallel: u64) -> Self {
        Self::new(None, Some(parallel))
    }

    /// Gets the serial value.
    #[must_use]
    pub const fn serial(&self) -> Option<u64> {
        self.serial
    }

    /// Gets the parallel value.
    #[must_use]
    pub const fn parallel(&self) -> Option<u64> {
        self.parallel
    }
}
