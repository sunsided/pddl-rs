//! Contains c-effects.

use crate::types::PEffect;

/// A c-effect.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CEffect<'a> {
    PEffect(PEffect<'a>),
    // TODO: Forall,
    // TODO: When
}

impl<'a> CEffect<'a> {
    pub const fn new_p_effect(effect: PEffect<'a>) -> Self {
        Self::PEffect(effect)
    }
}

impl<'a> From<PEffect<'a>> for CEffect<'a> {
    fn from(value: PEffect<'a>) -> Self {
        CEffect::new_p_effect(value)
    }
}
