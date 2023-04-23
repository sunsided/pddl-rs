//! Contains the [`LengthSpec`] type.

/// Deprecated since PDDL 2.1.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct LengthSpec {
    serial: Option<u64>,
    parallel: Option<u64>,
}

impl LengthSpec {
    pub const fn new(serial: Option<u64>, parallel: Option<u64>) -> Self {
        Self { serial, parallel }
    }

    pub const fn new_serial(serial: u64) -> Self {
        Self::new(Some(serial), None)
    }

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
