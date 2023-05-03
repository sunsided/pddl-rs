//! Contains durative action definitions via the [`DurativeActionDefinition`] type.

use crate::types::TypedVariables;
use crate::types::{
    DurationConstraint, DurativeActionEffect, DurativeActionGoalDefinition, DurativeActionSymbol,
};

/// A durative action represents an action which takes an amount
/// of time to complete. The amount of time is expressible as either a value or as
/// an inequality (allow for both fixed duration and ranged duration actions).
///
/// Similar to traditional actions ([`ActionDefinition`](crate::types::ActionDefinition))
/// we have conditions and effects, but it should be noted that the keyword in
/// durative actions is [`condition`](DurativeActionDefinition::condition), not
/// [`precondition`](crate::types::ActionDefinition::precondition).
///
/// This semantic change is designed to represent that a durative action may not just
/// condition when the action starts, but may have conditions which need to be true
/// at the end or over the duration of the action. A good example of this can be
/// found in flight planning, where an action fly requires that a runway be free at
/// the start and end of an action, in order for the plane to take off and land
/// whilst the runway does not need to be free whilst the plane is flying.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
pub struct DurativeActionDefinition<'a> {
    symbol: DurativeActionSymbol<'a>,
    parameters: TypedVariables<'a>,
    duration: Option<DurationConstraint<'a>>,
    condition: Option<DurativeActionGoalDefinition<'a>>,
    effect: Option<DurativeActionEffect<'a>>,
}

impl<'a> DurativeActionDefinition<'a> {
    pub const fn new(
        symbol: DurativeActionSymbol<'a>,
        parameters: TypedVariables<'a>,
        duration: Option<DurationConstraint<'a>>,
        condition: Option<DurativeActionGoalDefinition<'a>>,
        effect: Option<DurativeActionEffect<'a>>,
    ) -> Self {
        Self {
            symbol,
            parameters,
            duration,
            condition,
            effect,
        }
    }

    pub const fn symbol(&self) -> &DurativeActionSymbol<'a> {
        &self.symbol
    }

    /// The parameters defines the type of object we’re interested in.
    /// Note that a parameter can take any type or subtype.
    ///
    /// ## Example
    ///
    /// ```pddl
    /// (?s - site ?b - bricks)
    /// ```
    ///
    /// If we have for example three instances of site such as `s1`, `s2` and `s3`
    /// and we have two instances of bricks `b1`, `b2`, our planner considers all
    /// possible actions such as:
    ///
    /// `(BUILD-WALL s1 b1)` `(BUILD-WALL s2 b1)` `(BUILD-WALL s3 b1)`
    /// `(BUILD-WALL s1 b2)` `(BUILD-WALL s2 b2)` `(BUILD-WALL s3 b2)`
    ///
    /// Therefore it is not up to us as a user to specify the specific object to which
    /// an action applies but rather the type of objects to which the action applies.
    ///
    /// In this case, when we build a wall we need to know what bricks we're using to build
    /// it and where we’re building it. Our actions are specific to the problem we've chosen
    /// to consider and model, therefore there might be additional things that other users
    /// want/need to model that this model doesn't.
    ///
    /// Your domain and problem should only consider and model the aspects of the problem
    /// which you're trying to solve. For example here we haven't modelled the person who's
    /// actually going to perform the work, but maybe if we were the manager of a larger
    /// building site we might want to and therefore we would need to adapt my models.
    pub const fn parameters(&self) -> &TypedVariables<'a> {
        &self.parameters
    }

    /// A duration can be expressed as either a fixed value or an inequality.
    /// It is also possible to express duration as the value of a Numeric Fluent
    /// ([AtomicFunctionSkeleton](crate::types::AtomicFunctionSkeleton)), which means
    /// an action such as move can have a duration dependent on say distance between two points.
    ///
    /// ```pddl
    /// :duration (= ?duration 10)
    /// ```
    pub const fn duration(&self) -> &Option<DurationConstraint<'a>> {
        &self.duration
    }

    /// A condition is a logical and temporal expression which must be met in order for a
    /// durative action to execute.
    ///
    /// Because a durative action occurs over time, we may wish to express that additional
    /// conditions be met for the duration or end of the action, not just the start.
    /// This gives rise to three new keywords `at start`, `at end` and `over all`.
    pub const fn condition(&self) -> &Option<DurativeActionGoalDefinition<'a>> {
        &self.condition
    }

    /// An effect is a condition which is made true when an action is applied.
    /// Note that the effect is always more restrictive than an action and typically only
    /// allows `and` and `not` as logical expressions.
    pub const fn effect(&self) -> &Option<DurativeActionEffect<'a>> {
        &self.effect
    }
}

impl<'a> AsRef<DurativeActionSymbol<'a>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &DurativeActionSymbol<'a> {
        self.symbol()
    }
}

impl<'a> AsRef<TypedVariables<'a>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &TypedVariables<'a> {
        self.parameters()
    }
}

impl<'a> AsRef<Option<DurationConstraint<'a>>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &Option<DurationConstraint<'a>> {
        self.duration()
    }
}

impl<'a> AsRef<Option<DurativeActionGoalDefinition<'a>>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &Option<DurativeActionGoalDefinition<'a>> {
        self.condition()
    }
}

impl<'a> AsRef<Option<DurativeActionEffect<'a>>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &Option<DurativeActionEffect<'a>> {
        self.effect()
    }
}
