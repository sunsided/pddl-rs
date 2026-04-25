//! Contains the [`Problem`] type.

use crate::types::{
    InitElements, LengthSpec, MetricSpec, Name, Objects, ProblemConstraintsDef,
    ProblemGoalDefinition, Requirements,
};
use crate::{PreconditionGoalDefinitions, PreferenceConstraintGoalDefinitions};

/// A domain-specific problem declaration.
///
/// ## Usages
/// This is the top-level type of a problem description within a [`Domain`](crate::Domain).
///
/// ## Example
/// ```
/// # #[cfg(feature = "parser")]
/// # fn main() {
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
/// # }
/// # #[cfg(not(feature = "parser"))]
/// # fn main() {}
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
    goal: ProblemGoalDefinition,
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
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        name: Name,
        domain: Name,
        requires: Requirements,
        objects: Objects,
        init: InitElements,
        goal: ProblemGoalDefinition,
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
        goal: ProblemGoalDefinition,
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
    pub const fn constraints(&self) -> &PreferenceConstraintGoalDefinitions {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{InitElements, ProblemGoalDefinition};
    use crate::{PreconditionGoalDefinitions, Requirement, Requirements};

    fn make_minimal_problem() -> Problem {
        let init = InitElements::new(Vec::new());
        let goal = ProblemGoalDefinition::new(PreconditionGoalDefinitions::default());
        Problem::builder("test-problem", "test-domain", init, goal)
    }

    #[test]
    fn builder_works() {
        let problem = make_minimal_problem();
        assert_eq!(problem.name(), &Name::new("test-problem"));
        assert_eq!(problem.domain(), &Name::new("test-domain"));
        assert!(problem.requirements().is_empty());
        assert!(problem.init().is_empty());
        assert!(problem.goals().is_empty());
        assert!(problem.constraints().is_empty());
        assert!(problem.metric_spec().is_none());
        assert!(problem.length_spec().is_none());
    }

    #[test]
    fn with_requirements() {
        let problem =
            make_minimal_problem().with_requirements(Requirements::new([Requirement::Strips]));
        assert_eq!(problem.requirements().len(), 1);
    }

    #[test]
    fn with_objects() {
        let problem = make_minimal_problem().with_objects(Objects::default());
        assert!(problem.objects().is_empty());
    }

    #[test]
    fn with_constraints() {
        let problem = make_minimal_problem().with_constraints(ProblemConstraintsDef::default());
        assert!(problem.constraints().is_empty());
    }

    #[test]
    fn new_full() {
        let init = InitElements::new(Vec::new());
        let goal = ProblemGoalDefinition::new(PreconditionGoalDefinitions::default());
        let problem = Problem::new(
            Name::new("p"),
            Name::new("d"),
            Requirements::new([Requirement::Typing]),
            Objects::default(),
            init,
            goal,
            ProblemConstraintsDef::default(),
            None,
            None,
        );
        assert_eq!(problem.name(), &Name::new("p"));
        assert_eq!(problem.domain(), &Name::new("d"));
        assert_eq!(problem.requirements().len(), 1);
    }

    #[test]
    fn clone_works() {
        let problem = make_minimal_problem();
        let clone = problem.clone();
        assert_eq!(problem, clone);
    }

    #[test]
    fn debug_impl() {
        let problem = make_minimal_problem();
        let dbg = format!("{problem:?}");
        assert!(dbg.contains("Problem"));
        assert!(dbg.contains("test-problem"));
    }
}
