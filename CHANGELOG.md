# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Added `TryInto<PreconditionGoalDefinition>` for `PreconditionGoalDefinitions` to get
  the only element of the list if it is a one-element list, or `None`.
- `TryInto` implementations were added for `CEffect` to allow deconstruction into
  `PEffect`, `ForallCEffect` and `WhenCEffect`.

### Changed

- The `PrefConGD::And` variant was removed and replaced with the `PrefConGDs` type.
- The `Effect::All` and `Effect::Single` variants were removed and the `Effect` type
  was changed to a struct wrapping a vector `Effects`.
- The `CEffect` variants were changed to wrap `ForallCEffect` and `WhenCEffect` types.
- The parser now uses [nom-greedyerror](https://github.com/dalance/nom-greedyerror) and 
  [nom_locate](https://github.com/fflorent/nom_locate) to improve error handling.
- The `Parser` trait now takes an `T: Into<Span<'a>>` argument.

### Fixed

- Fixed an issue where `(and ...)` conditional effects would be accidentally parsed
  into an atomic formula with predicate `and`.

## [0.0.4] - 2023-05-04

### Added

- Added `IntoIterator` implementations for `ConditionalEffect`, `DurationConstraint`
  and `Effect` that flatten the `Single` and `All` variants into a single iterator.
  In all these cases, the `(and ...)` representation allows for a cardinality of
  zero, one or many, which makes `x` and `(and x)` identical.
- Added `From<AtomicFormula<T>>` for `Literal<T>`.

### Changes

- The `PreGD` type was renamed to `PreconditionGoalDefinition`.
- The `PreconditionGoalDefinition::And` variant was removed and replaced with the `PreconditionGoalDefinitions` type.

## [0.0.3] - 2023-05-03

### Internal

- The `nom` dependency is now an implicit feature used by the `parser` crate feature. 

## [0.0.2] - 2023-04-24

### Added

- Added `Parser` trait.

### Changed

- Made `types` module private and re-exported all types top-level.

## [0.0.1] - 2023-04-23

### Added

- Minor usability adjustments such as added accessors for internal state of structs.

## [0.0.0] - 2023-04-23

### Internal

- 🎉 Initial release.

[0.0.4]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.4
[0.0.3]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.3
[0.0.2]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.2
[0.0.1]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.1
[0.0.0]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.0
