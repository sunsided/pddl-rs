//! The check that no two timed initial literals contradict each other.
//!
//! Literals at the same time (within [`SAME_TIME`]) take effect in one happening. If they
//! set one atom both true and false, every plan that reaches that time would fail on the
//! problem's own timeline, so the problem is rejected before any plan runs.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use pddl::{AtomicFormula, InitElement, Literal, Problem};

use crate::atom::{lower, Atom};
use crate::error::{ModelError, ModelErrorKind};
use crate::model::SAME_TIME;

/// A timed initial literal over a predicate.
struct Timed {
    /// When it takes effect.
    time: f64,
    /// The ground atom it sets, lowercased.
    atom: Atom,
    /// `true` if it adds the atom, `false` if it deletes it.
    positive: bool,
}

/// Checks that no two timed initial literals at the same time set one atom both true
/// and false.
///
/// # Errors
///
/// [`ModelErrorKind::ContradictoryTimedLiterals`] at `init` for the first such pair, in
/// the order of the problem.
pub fn contradictions(problem: &Problem) -> Result<(), ModelError> {
    let literals = timed_literals(problem);
    for (index, first) in literals.iter().enumerate() {
        let clash = literals.iter().skip(index + 1).any(|second| {
            first.positive != second.positive
                && (first.time - second.time).abs() <= SAME_TIME
                && first.atom == second.atom
        });
        if clash {
            let kind = ModelErrorKind::ContradictoryTimedLiterals {
                atom: format!("{}", first.atom),
                time: format!("{}", first.time),
            };
            return Err(ModelError::new("init", kind));
        }
    }

    Ok(())
}

/// Returns the timed initial literals over predicates, in the order of the problem.
/// Timed equalities are left to the formula check, which rejects them.
fn timed_literals(problem: &Problem) -> Vec<Timed> {
    let mut literals = Vec::new();
    for element in problem.init().iter() {
        let InitElement::At(time, literal) = element else {
            continue;
        };

        let (formula, positive) = match literal {
            Literal::AtomicFormula(formula) => (formula, true),
            Literal::NotAtomicFormula(formula) => (formula, false),
        };
        let AtomicFormula::Predicate(predicate) = formula else {
            continue;
        };

        let arguments: Vec<String> = predicate.values().iter().map(|name| lower(name)).collect();
        literals.push(Timed {
            time: **time,
            atom: Atom::lowered(lower(predicate.predicate()), arguments),
            positive,
        });
    }

    literals
}
