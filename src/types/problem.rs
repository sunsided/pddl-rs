//! Contains the [`Problem`] type.

use crate::types::{
    GoalDef, InitElements, LengthSpec, MetricSpec, Name, Objects, PreGD, PrefConGD,
    ProblemConstraintsDef, Requirements,
};

/// A domain-specific problem declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Problem<'a> {
    name: Name<'a>,
    domain: Name<'a>,
    requires: Requirements,
    objects: Objects<'a>,
    init: InitElements<'a>,
    goal: GoalDef<'a>,
    /// Requires [Constraints](crate::types::Requirement::Constraints).
    constraints: ProblemConstraintsDef<'a>,
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    metric_spec: Option<MetricSpec<'a>>,
    /// Deprecated since PDDL 2.1.
    length_spec: Option<LengthSpec>,
}

impl<'a> Problem<'a> {
    pub const fn new(
        name: Name<'a>,
        domain: Name<'a>,
        requires: Requirements,
        objects: Objects<'a>,
        init: InitElements<'a>,
        goal: GoalDef<'a>,
        constraints: ProblemConstraintsDef<'a>,
        metric_spec: Option<MetricSpec<'a>>,
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
    pub fn builder<P: Into<Name<'a>>, D: Into<Name<'a>>>(
        problem_name: P,
        domain_name: D,
        init: InitElements<'a>,
        goal: GoalDef<'a>,
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
    pub fn with_objects<O: Into<Objects<'a>>>(mut self, objects: O) -> Self {
        self.objects = objects.into();
        self
    }

    /// Adds a list of constraints to the problem.
    pub fn with_constraints<C: Into<ProblemConstraintsDef<'a>>>(mut self, constraints: C) -> Self {
        self.constraints = constraints.into();
        self
    }

    /// Adds a list of metric specifications to the problem.
    pub fn with_metric_spec<M: Into<MetricSpec<'a>>>(mut self, metric: M) -> Self {
        self.metric_spec = Some(metric.into());
        self
    }

    /// Adds a list of length specifications to the problem.
    pub fn with_length_spec<L: Into<LengthSpec>>(mut self, length: L) -> Self {
        self.length_spec = Some(length.into());
        self
    }

    /// Returns the problem name.
    pub const fn name(&self) -> &Name<'a> {
        &self.name
    }

    /// Returns the domain name.
    pub const fn domain(&self) -> &Name<'a> {
        &self.domain
    }

    /// Returns the optional problem requirements.
    pub const fn requirements(&self) -> &Requirements {
        &self.requires
    }

    /// Returns the optional object declarations.
    pub const fn objects(&self) -> &Objects<'a> {
        &self.objects
    }

    /// Returns the initialization of the problem.
    pub const fn init(&self) -> &InitElements<'a> {
        &self.init
    }

    /// Returns the goal statement of the problem.
    pub const fn goal(&self) -> &PreGD<'a> {
        &self.goal.value()
    }

    /// Returns the optional constraints of the problem.
    /// Requires [Constraints](crate::types::Requirement::Constraints).
    pub const fn constraints(&self) -> &PrefConGD<'a> {
        &self.constraints.value()
    }

    /// Returns the optional metric specification of the problem.
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    pub const fn metric_spec(&self) -> &Option<MetricSpec<'a>> {
        &self.metric_spec
    }

    /// Returns the optional length specification of the problem.
    /// Deprecated since PDDL 2.1.
    pub const fn length_spec(&self) -> &Option<LengthSpec> {
        &self.length_spec
    }
}
