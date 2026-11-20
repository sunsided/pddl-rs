//! Plans and the plan file format.
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
//! Temporal planners prefix each step with its start time and follow a durative action
//! with its duration, as VAL reads them:
//!
//! ```text
//! 0.000: (load p1 t1 depot) [2.000]
//! 2.010: (drive t1 depot shop) [5.000]
//! 7.020: (refuel t1)
//! ```
//!
//! Times and durations are non-negative decimal numbers. Steps need not be sorted by
//! time. A plan either times every step or none.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use crate::atom::lower;

/// A plan: ground actions, executed one after the other or, with timestamps, at the
/// times given.
#[derive(Debug, Clone, PartialEq, Default)]
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

/// One ground action of a plan, such as `(drive t1 home shop)`, with its start time
/// and duration in a temporal plan.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanStep {
    /// The action name, in ASCII lowercase.
    action: String,
    /// The objects the action is applied to, in ASCII lowercase.
    arguments: Vec<String>,
    /// The 1-based line of the plan file the step starts on; 0 for a step built in code.
    line: usize,
    /// The start time, finite and non-negative; `None` in a sequential plan.
    time: Option<f64>,
    /// The duration of a durative action, finite and non-negative; `None` otherwise.
    duration: Option<f64>,
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
            time: None,
            duration: None,
        }
    }

    /// Returns the step starting at `time`, for a temporal plan.
    ///
    /// # Examples
    ///
    /// ```
    /// use pddl_validate::PlanStep;
    ///
    /// let step = PlanStep::new("drive", ["t1"]).at(1.5).lasting(2.0);
    /// assert_eq!((step.time(), step.duration()), (Some(1.5), Some(2.0)));
    /// ```
    #[must_use]
    pub const fn at(mut self, time: f64) -> Self {
        self.time = Some(time);
        self
    }

    /// Returns the step with a duration, for a durative action.
    #[must_use]
    pub const fn lasting(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Returns the start time, or `None` in a sequential plan.
    #[must_use]
    pub const fn time(&self) -> Option<f64> {
        self.time
    }

    /// Returns the duration, or `None` if the plan gives none.
    #[must_use]
    pub const fn duration(&self) -> Option<f64> {
        self.duration
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
    /// Text outside parentheses that is neither whitespace, a comment, a timestamp nor a
    /// duration.
    UnexpectedText(String),
    /// A timestamp (`0.5:`) without a step after it.
    DanglingTimestamp,
    /// A `[` without its `]`, or a duration that is not one number.
    MalformedDuration,
    /// A time or duration that is not a finite, non-negative number.
    InvalidNumber(String),
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
            PlanErrorKind::DanglingTimestamp => f.write_str("a timestamp needs a step after it"),
            PlanErrorKind::MalformedDuration => {
                f.write_str("a duration is one number in `[` and `]`")
            }
            PlanErrorKind::InvalidNumber(text) => {
                write!(f, "`{text}` is not a finite, non-negative number")
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
                    let step = self.step(start)?;
                    steps.push(self.duration(step)?);
                }
                ')' => return Err(error(start, PlanErrorKind::Unopened)),
                _ => {
                    let time = self.timestamp(start)?;
                    let at = self.at;
                    if self.skip_blank() != Some('(') {
                        return Err(error(at, PlanErrorKind::DanglingTimestamp));
                    }

                    let step_start = self.at;
                    self.bump();
                    let step = self.step(step_start)?.at(time);
                    steps.push(self.duration(step)?);
                }
            }
        }

        Ok(Plan::new(steps))
    }

    /// Reads a timestamp such as `0.500:`.
    fn timestamp(&mut self, start: Position) -> Result<f64, PlanError> {
        let mut word = self.word();
        if word.is_empty() {
            // A lone `[` or `]`, which `word` does not consume.
            word.extend(self.chars.next());
        }

        let Some(number) = word.strip_suffix(':') else {
            return Err(error(start, PlanErrorKind::UnexpectedText(word)));
        };

        number_at(start, number)
    }

    /// Reads an optional `[duration]` after a step and attaches it.
    fn duration(&mut self, step: PlanStep) -> Result<PlanStep, PlanError> {
        if self.skip_blank() != Some('[') {
            return Ok(step);
        }

        let start = self.at;
        self.bump();
        if self.skip_blank().is_none() {
            return Err(error(start, PlanErrorKind::MalformedDuration));
        }

        let number_start = self.at;
        let word = self.word();
        let duration = number_at(number_start, &word)?;
        if self.skip_blank() != Some(']') {
            return Err(error(start, PlanErrorKind::MalformedDuration));
        }

        self.bump();
        Ok(step.lasting(duration))
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
                Some(c @ ('[' | ']')) => {
                    let kind = PlanErrorKind::UnexpectedText(String::from(c));
                    return Err(error(self.at, kind));
                }
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
            time: None,
            duration: None,
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
            if c.is_whitespace() || matches!(c, '(' | ')' | ';' | '[' | ']') {
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

/// Parses a time or duration written at `at`.
fn number_at(at: Position, text: &str) -> Result<f64, PlanError> {
    match text.parse::<f64>() {
        Ok(value) if value.is_finite() && value >= 0.0 => Ok(value),
        _ => Err(error(at, PlanErrorKind::InvalidNumber(String::from(text)))),
    }
}
