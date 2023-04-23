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

    pub const fn serial(&self) -> Option<u64> {
        self.serial
    }

    pub const fn parallel(&self) -> Option<u64> {
        self.parallel
    }
}
