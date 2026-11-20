//! Sequential plans and the plan file format.
//!
//! A plan file lists one ground action per step, as Fast Downward and most classical
//! planners write it:
//!
//! ```text
//! ; a comment
//! (pick ball1 rooma left)
//! (move rooma roomb)
//! ; cost = 4 (general cost)
//! ```
//!
//! Comments run from `;` to the end of the line, so cost lines are comments too and the
//! validator computes the cost itself. Steps may share a line or span several. Names are
//! case-insensitive and stored in ASCII lowercase.
//!
//! Timestamped steps (`0.000: (a) [1.000]`) belong to temporal plans, which this crate
//! does not validate yet; [`Plan::from_str`] fails on them with
//! [`PlanErrorKind::Timestamped`], which [`Validator::validate_text`](crate::Validator::validate_text)
//! reports as [`ValidationStatus::Unsupported`](crate::ValidationStatus::Unsupported).

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use crate::atom::lower;

/// A sequential plan: ground actions, executed one after the other.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Plan {
    /// The steps in execution order.
    steps: Vec<PlanStep>,
}

impl Plan {
    /// Builds a plan from steps.
    #[must_use]
    pub const fn new(steps: Vec<PlanStep>) -> Self {
        Self { steps }
    }

    /// Returns the steps in execution order.
    #[must_use]
    pub fn steps(&self) -> &[PlanStep] {
        &self.steps
    }

    /// Returns the number of steps.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Returns `true` for the empty plan, which is valid exactly when the initial state
    /// satisfies the goal.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

impl FromIterator<PlanStep> for Plan {
    fn from_iter<T: IntoIterator<Item = PlanStep>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl FromStr for Plan {
    type Err = PlanError;

    /// Parses a plan file.
    ///
    /// # Errors
    ///
    /// Returns a [`PlanError`] with the line and column of the first problem; see
    /// [`PlanErrorKind`] for the kinds.
    ///
    /// # Examples
    ///
    /// ```
    /// use pddl_validate::Plan;
    ///
    /// let plan: Plan = "(move a b) ; first\n(MOVE b c)\n; cost = 2 (unit cost)".parse().unwrap();
    /// assert_eq!(plan.len(), 2);
    /// assert_eq!(plan.steps()[1].to_string(), "(move b c)");
    /// assert_eq!(plan.steps()[1].line(), 2);
    /// ```
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Scanner::new(text).plan()
    }
}

/// One ground action of a plan, such as `(drive t1 home shop)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanStep {
    /// The action name, in ASCII lowercase.
    action: String,
    /// The objects the action is applied to, in ASCII lowercase.
    arguments: Vec<String>,
    /// The 1-based line of the plan file the step starts on; 0 for a step built in code.
    line: usize,
}

impl PlanStep {
    /// Builds a step, lowercasing the action and its arguments. Its [`line`](Self::line) is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use pddl_validate::PlanStep;
    ///
    /// let step = PlanStep::new("Drive", ["t1", "home", "shop"]);
    /// assert_eq!(step.to_string(), "(drive t1 home shop)");
    /// ```
    #[must_use]
    pub fn new<S, I, A>(action: S, arguments: I) -> Self
    where
        S: AsRef<str>,
        I: IntoIterator<Item = A>,
        A: AsRef<str>,
    {
        Self {
            action: lower(action.as_ref()),
            arguments: arguments.into_iter().map(|a| lower(a.as_ref())).collect(),
            line: 0,
        }
    }

    /// Returns the action name, in ASCII lowercase.
    #[must_use]
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Returns the arguments, in ASCII lowercase.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    /// Returns the 1-based line the step starts on in its plan file, or 0 if it was built
    /// in code.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for PlanStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}", self.action)?;
        for argument in &self.arguments {
            write!(f, " {argument}")?;
        }

        f.write_str(")")
    }
}

/// Why a plan file could not be read, and where.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PlanError {
    /// The 1-based line of the problem.
    pub line: usize,
    /// The 1-based column of the problem, counted in characters.
    pub column: usize,
    /// What is wrong.
    pub kind: PlanErrorKind,
}

/// The kinds of [`PlanError`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlanErrorKind {
    /// A `(` without its `)` before the end of the file.
    Unclosed,
    /// A `)` without a matching `(`.
    Unopened,
    /// A `(` inside a step. Steps are flat lists of names.
    Nested,
    /// A step `()` without an action name.
    EmptyStep,
    /// Text outside parentheses that is neither whitespace nor a comment.
    UnexpectedText(String),
    /// A timestamp (`0.5:`) or duration (`[2]`) of a temporal plan.
    Timestamped,
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}: ", self.line, self.column)?;
        match &self.kind {
            PlanErrorKind::Unclosed => f.write_str("`(` is never closed"),
            PlanErrorKind::Unopened => f.write_str("`)` has no matching `(`"),
            PlanErrorKind::Nested => f.write_str("a step cannot contain `(`"),
            PlanErrorKind::EmptyStep => f.write_str("a step needs an action name"),
            PlanErrorKind::UnexpectedText(text) => {
                write!(f, "unexpected `{text}` outside a step")
            }
            PlanErrorKind::Timestamped => {
                f.write_str("timestamps and durations belong to temporal plans")
            }
        }
    }
}

impl core::error::Error for PlanError {}

/// A position in the plan text.
#[derive(Debug, Clone, Copy)]
struct Position {
    /// The 1-based line.
    line: usize,
    /// The 1-based column, in characters.
    column: usize,
}

/// A single-pass, non-recursive reader of plan files.
///
/// It never indexes into the text, so no input can make it panic, and it keeps no stack,
/// so no nesting depth can overflow one.
struct Scanner<'t> {
    /// The remaining characters.
    chars: core::iter::Peekable<core::str::Chars<'t>>,
    /// The position of the next character.
    at: Position,
}

impl<'t> Scanner<'t> {
    /// Starts reading `text`, skipping a leading byte order mark.
    fn new(text: &'t str) -> Self {
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        Self {
            chars: text.chars().peekable(),
            at: Position { line: 1, column: 1 },
        }
    }

    /// Reads the whole plan.
    fn plan(mut self) -> Result<Plan, PlanError> {
        let mut steps = Vec::new();
        while let Some(c) = self.skip_blank() {
            let start = self.at;
            match c {
                '(' => {
                    self.bump();
                    steps.push(self.step(start)?);
                }
                ')' => return Err(error(start, PlanErrorKind::Unopened)),
                '[' => return Err(error(start, PlanErrorKind::Timestamped)),
                _ => {
                    let word = self.word();
                    return Err(error(start, outside_kind(word)));
                }
            }
        }

        Ok(Plan::new(steps))
    }

    /// Reads the names of a step after its `(`, through its `)`.
    fn step(&mut self, start: Position) -> Result<PlanStep, PlanError> {
        let mut names = Vec::new();
        loop {
            match self.skip_blank() {
                None => return Err(error(start, PlanErrorKind::Unclosed)),
                Some(')') => {
                    self.bump();
                    break;
                }
                Some('(') => return Err(error(self.at, PlanErrorKind::Nested)),
                Some(_) => names.push(lower(&self.word())),
            }
        }

        let mut names = names.into_iter();
        let action = names
            .next()
            .ok_or_else(|| error(start, PlanErrorKind::EmptyStep))?;

        Ok(PlanStep {
            action,
            arguments: names.collect(),
            line: start.line,
        })
    }

    /// Skips whitespace and comments and returns the next character without consuming it.
    fn skip_blank(&mut self) -> Option<char> {
        loop {
            let c = *self.chars.peek()?;
            if c == ';' {
                while self.chars.peek().is_some_and(|&c| c != '\n' && c != '\r') {
                    self.bump();
                }
            } else if c.is_whitespace() {
                self.bump();
            } else {
                return Some(c);
            }
        }
    }

    /// Reads a run of characters up to whitespace, a parenthesis or a comment.
    fn word(&mut self) -> String {
        let mut word = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() || matches!(c, '(' | ')' | ';') {
                break;
            }

            word.push(c);
            self.bump();
        }

        word
    }

    /// Consumes one character and advances the position. A `\r\n` pair, a lone `\r` and a
    /// lone `\n` each end one line.
    fn bump(&mut self) {
        match self.chars.next() {
            Some('\r') if self.chars.peek() == Some(&'\n') => {}
            Some('\n' | '\r') => {
                self.at.line += 1;
                self.at.column = 1;
            }
            Some(_) => self.at.column += 1,
            None => {}
        }
    }
}

/// Builds a [`PlanError`] at `at`.
const fn error(at: Position, kind: PlanErrorKind) -> PlanError {
    PlanError {
        line: at.line,
        column: at.column,
        kind,
    }
}

/// Classifies text found outside a step: a number followed by `:` is a timestamp of a
/// temporal plan, anything else is unexpected.
fn outside_kind(word: String) -> PlanErrorKind {
    let is_timestamp = word.strip_suffix(':').is_some_and(|number| {
        !number.is_empty() && number.chars().all(|c| c.is_ascii_digit() || c == '.')
    });
    if is_timestamp {
        PlanErrorKind::Timestamped
    } else {
        PlanErrorKind::UnexpectedText(word)
    }
}
