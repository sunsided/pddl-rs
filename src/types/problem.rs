//! Contains the [`Problem`] type.

use crate::types::{
    InitElements, LengthSpec, MetricSpec, Name, Objects, PreGD, PrefConGD, Requirements,
};

/// A domain-specific problem declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct Problem<'a> {
    name: Name<'a>,
    domain: Name<'a>,
    requires: Requirements,
    objects: Objects<'a>,
    init: InitElements<'a>,
    goal: PreGD<'a>,
    /// Requires [Constraints](crate::types::Requirement::Constraints).
    constraints: PrefConGD<'a>,
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
        goal: PreGD<'a>,
        constraints: PrefConGD<'a>,
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

    pub fn builder<P: Into<Name<'a>>, D: Into<Name<'a>>>(
        problem_name: P,
        domain_name: D,
        init: InitElements<'a>,
        goal: PreGD<'a>,
    ) -> Self {
        Self {
            name: problem_name.into(),
            domain: domain_name.into(),
            requires: Requirements::new([]), // TODO: Do we need to imply STRIPS?
            objects: Objects::default(),
            init,
            goal,
            constraints: PrefConGD::default(),
            metric_spec: None,
            length_spec: None,
        }
    }

    pub fn with_requirements<R: Into<Requirements>>(mut self, requirements: R) -> Self {
        self.requires = requirements.into();
        self
    }

    pub fn with_objects<O: Into<Objects<'a>>>(mut self, objects: O) -> Self {
        self.objects = objects.into();
        self
    }

    pub fn with_constraints<C: Into<PrefConGD<'a>>>(mut self, constraints: C) -> Self {
        self.constraints = constraints.into();
        self
    }

    pub fn with_metric_spec<M: Into<MetricSpec<'a>>>(mut self, metric: M) -> Self {
        self.metric_spec = Some(metric.into());
        self
    }

    pub fn with_length_spec<L: Into<LengthSpec>>(mut self, length: L) -> Self {
        self.length_spec = Some(length.into());
        self
    }

    pub const fn name(&self) -> &Name<'a> {
        &self.name
    }

    pub const fn domain(&self) -> &Name<'a> {
        &self.domain
    }

    pub const fn requirements(&self) -> &Requirements {
        &self.requires
    }

    pub const fn objects(&self) -> &Objects<'a> {
        &self.objects
    }

    pub const fn init(&self) -> &InitElements<'a> {
        &self.init
    }

    pub const fn goal(&self) -> &PreGD<'a> {
        &self.goal
    }

    pub const fn constraints(&self) -> &PrefConGD<'a> {
        &self.constraints
    }

    pub const fn metric_spec(&self) -> &Option<MetricSpec<'a>> {
        &self.metric_spec
    }

    /// Deprecated since PDDL 2.1.
    pub const fn length_spec(&self) -> &Option<LengthSpec> {
        &self.length_spec
    }
}
