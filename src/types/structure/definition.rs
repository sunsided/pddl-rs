//! Defines the [`Definition`] type for a single domain or problem.

use crate::{Domain, Problem};

/// A single `(define ...)` block: either a domain or a problem.
///
/// Parse it when the input must hold exactly one definition of either kind;
/// use [`PddlFile`](crate::PddlFile) for inputs with several definitions.
#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    /// A `(define (domain ...))` block.
    Domain(Domain),
    /// A `(define (problem ...))` block.
    Problem(Problem),
}
