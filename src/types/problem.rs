//! Contains the [`Problem`] type.

use crate::types::{
    GoalDef, InitElements, LengthSpec, MetricSpec, Name, Objects, ProblemConstraintsDef,
    Requirements,
};
use crate::{PreconditionGoalDefinitions, PrefConGDs};

/// A domain-specific problem declaration.
///
/// ## Usages
/// This is the top-level type of a problem description within a [`Domain`](crate::Domain).
///
/// ## Example
/// ```
/// # use pddl::{Name, Parser, Problem};
/// let input = r#"(define (problem get-paid)
///         (:domain briefcase-world)
///         (:init (place home) (place office)
///                (object p) (object d) (object b)
///                (at B home) (at P home) (at D home) (in P))
///         (:goal (and (at B office) (at D office) (at P home)))
///     )"#;
///
/// let problem = Problem::from_str(input).unwrap();
///
/// assert_eq!(problem.name(), "get-paid");
/// assert_eq!(problem.domain(), "briefcase-world");
/// assert!(problem.requirements().is_empty());
/// assert_eq!(problem.init().len(), 9);
/// assert_eq!(problem.goals().len(), 3);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Problem {
    // The problem name.
    name: Name,
    /// The name of the [`Domain`] this problem belongs to.
    domain: Name,
    /// The optional list of requirements.
    requires: Requirements,
    /// The optional list of object declarations.
    objects: Objects,
    /// The initial state definition.
    init: InitElements,
    /// The goal definition.
    goal: GoalDef,
    /// The optional list of constraints.
    ///
    /// ## Requirements
    /// Requires [Constraints](crate::Requirement::Constraints).
    constraints: ProblemConstraintsDef,
    /// The optional list of metrics specifications.
    ///
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    metric_spec: Option<MetricSpec>,
    /// The optional goal length specification.
    ///
    /// Deprecated since PDDL 2.1.
    length_spec: Option<LengthSpec>,
}

impl Problem {
    /// Creates a new [`Problem`] instance.
    pub const fn new(
        name: Name,
        domain: Name,
        requires: Requirements,
        objects: Objects,
        init: InitElements,
        goal: GoalDef,
        constraints: ProblemConstraintsDef,
        metric_spec: Option<MetricSpec>,
        length_spec: Option<LengthSpec>,
    ) -> Self {
        Self {
            name,
            domain,
            requires,
            objects,
            init,
            goal,
            constraints,
            metric_spec,
            length_spec,
        }
    }

    /// Creates a builder to easily construct problems.
    pub fn builder<P: Into<Name>, D: Into<Name>>(
        problem_name: P,
        domain_name: D,
        init: InitElements,
        goal: GoalDef,
    ) -> Self {
        Self {
            name: problem_name.into(),
            domain: domain_name.into(),
            requires: Requirements::new([]), // TODO: Do we need to imply STRIPS?
            objects: Objects::default(),
            init,
            goal,
            constraints: ProblemConstraintsDef::default(),
            metric_spec: None,
            length_spec: None,
        }
    }

    /// Adds a list of requirements to the problem.
    pub fn with_requirements<R: Into<Requirements>>(mut self, requirements: R) -> Self {
        self.requires = requirements.into();
        self
    }

    /// Adds a list of object declarations to the problem.
    pub fn with_objects<O: Into<Objects>>(mut self, objects: O) -> Self {
        self.objects = objects.into();
        self
    }

    /// Adds a list of constraints to the problem.
    pub fn with_constraints<C: Into<ProblemConstraintsDef>>(mut self, constraints: C) -> Self {
        self.constraints = constraints.into();
        self
    }

    /// Adds a list of metric specifications to the problem.
    pub fn with_metric_spec<M: Into<MetricSpec>>(mut self, metric: M) -> Self {
        self.metric_spec = Some(metric.into());
        self
    }

    /// Adds a list of length specifications to the problem.
    pub fn with_length_spec<L: Into<LengthSpec>>(mut self, length: L) -> Self {
        self.length_spec = Some(length.into());
        self
    }

    /// Returns the problem name.
    pub const fn name(&self) -> &Name {
        &self.name
    }

    /// Returns the domain name.
    pub const fn domain(&self) -> &Name {
        &self.domain
    }

    /// Returns the optional problem requirements.
    pub const fn requirements(&self) -> &Requirements {
        &self.requires
    }

    /// Returns the optional object declarations.
    pub const fn objects(&self) -> &Objects {
        &self.objects
    }

    /// Returns the initialization of the problem.
    pub const fn init(&self) -> &InitElements {
        &self.init
    }

    /// Returns the goal statement of the problem.
    pub const fn goals(&self) -> &PreconditionGoalDefinitions {
        self.goal.value()
    }

    /// Returns the optional constraints of the problem.
    /// ## Requirements
    /// Requires [Constraints](crate::Requirement::Constraints).
    pub const fn constraints(&self) -> &PrefConGDs {
        self.constraints.value()
    }

    /// Returns the optional metric specification of the problem.
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    pub const fn metric_spec(&self) -> &Option<MetricSpec> {
        &self.metric_spec
    }

    /// Returns the optional length specification of the problem.
    /// Deprecated since PDDL 2.1.
    pub const fn length_spec(&self) -> &Option<LengthSpec> {
        &self.length_spec
    }
}
