# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- [#97](https://github.com/sunsided/pddl-rs/issues/97): `Parser::from_str` now returns the crate-owned
  `pddl::ParseError` instead of `nom::Err<nom::error::Error<_>>`. It exposes `line()`, `column()`,
  `offset()` and a bounded `found()` excerpt, and its `Display` no longer prints the whole remaining
  input. Consumers no longer need a direct `nom` dependency to handle parse errors.
- The nom error alias `pddl::parsers::ParseError<'a>` used by the `parse_*` combinators was renamed
  to `pddl::parsers::NomError<'a>`.
- [#98](https://github.com/sunsided/pddl-rs/issues/98): `Parser::from_str` now rejects trailing input
  instead of silently discarding it. Only whitespace and `;` comments may follow the parsed item. Use
  `PddlFile::from_str` to read several `(define ...)` blocks from one input.
- [#99](https://github.com/sunsided/pddl-rs/issues/99): Keywords are matched case-insensitively, so
  files such as `(DEFINE (DOMAIN D) (:REQUIREMENTS :STRIPS) ...)` parse. `Name` keeps the spelling it
  was written with (and pretty-prints it), but its `PartialEq`, `Eq` and `Hash` now ignore ASCII case,
  so `AT` and `at` are the same name. This also applies to every type built on `Name`, such as
  `Variable`, `Predicate` and `PrimitiveType`.
- [#100](https://github.com/sunsided/pddl-rs/issues/100): Domain and problem sections may appear in
  any order after the `(domain ...)` / `(problem ...)` header, and domain sections may be interleaved
  with actions (which keep their relative order). A section given twice is now an error at its second
  occurrence; a problem without `:domain`, `:init` or `:goal` is an error where its sections end.
- [#101](https://github.com/sunsided/pddl-rs/issues/101): Whitespace before a `(` is optional, so
  `:parameters(?x)` and `(and(p)(q))` parse. Between the parts of a form, `;` comments are accepted
  where whitespace is. The exception is the constraint form `(at end <GD>)`, whose `at end` still needs
  exactly one space and no comment between the words. Keywords that open a form now match as whole words, so a
  predicate such as `(preferences ...)` is no longer parsed as a `preference`.
- [#102](https://github.com/sunsided/pddl-rs/issues/102): Unknown `:requirements` keys (e.g. newer or
  planner-specific flags) no longer fail the whole file. They are kept, with their leading `:`, in the
  new `Requirements::unknown()`, separately from the known `Requirement`s, and are pretty-printed after
  them. `Requirement` itself is unchanged. Known keys now match as whole words, so `:typing-x` is an
  unknown key rather than `:typing`.
- [#103](https://github.com/sunsided/pddl-rs/issues/103): Parse errors point at the actual mistake
  instead of the start of the enclosing form, and say why where possible: the new
  `ParseError::reason()` returns e.g. ``in `:precondition` ``, `duplicate section`, `unknown section`
  or ``missing `:goal` ``, which `Display` includes. Once a `:keyword` such as `:action`,
  `:precondition` or `:requirements` has matched, a failure inside it is no longer backtracked.
- `pddl::parsers::NomError<'a>`, the error type of the `parse_*` combinators, is now a crate-defined
  struct instead of an alias for `nom::error::Error`. It keeps the public `input` and `code` fields and
  adds `context`.

### Internal

- [#109](https://github.com/sunsided/pddl-rs/issues/109): Keyword parsers (operators, time specifiers,
  requirement keys, …) map each keyword directly to its enum variant instead of converting the matched
  text with `try_from(...).expect(...)`, which removes a per-keyword allocation and a panic path.

## [0.1.0] - 2024-05-19

[0.1.0]: https://github.com/sunsided/pddl-rs/releases/tag/v0.1.0

### Internal

- [#84](https://github.com/sunsided/pddl-rs/pull/84): Added code coverage tracking and improved test coverage.

## [0.0.7] - 2023-05-27

### Added

- [#82](https://github.com/sunsided/pddl-rs/issues/82): Added support for PDDL line comments.

## [0.0.6] - 2023-05-07

### Added

- Added a `PartialEq` for `Name` that allows comparison with `str` and `String` directly.

### Changed

- Renamed `Problem::goal` to `Problem::goals` to reflect the fact that it is iterable.
- If well-known names such as `object` or `number` are used for a `Name`, these values
  will be interned to a `'static str`.

### Internal

- The `thiserror` dependency is now only brought in with the `parser` crate feature.
- Use string interning to reduce the number of allocations required for `Name` types.

## [0.0.5] - 2023-05-05

### Added

- Added `TryInto<PreconditionGoalDefinition>` for `PreconditionGoalDefinitions` to get
  the only element of the list if it is a one-element list, or `None`.
- `TryInto` implementations were added for `CEffect` to allow deconstruction into
  `PEffect`, `ForallCEffect` and `WhenCEffect`.
- Added the `Parser::from_str` method that performs a `Parse::parse` but discards
  the remaining unparsed text.

### Changed

- The `PrefConGD::And` variant was removed and replaced with the `PrefConGDs` type.
- The `Effect::All` and `Effect::Single` variants were removed and the `Effect` type
  was changed to a struct wrapping a vector `Effects`.
- The `CEffect` variants were changed to wrap `ForallCEffect` and `WhenCEffect` types.
- The parser now uses [nom-greedyerror](https://github.com/dalance/nom-greedyerror) and
  [nom_locate](https://github.com/fflorent/nom_locate) to improve error handling.
- The `Parser` trait now takes an `T: Into<Span<'a>>` argument.
- All parser methods now take an `T: Into<Span<'a>>` argument.

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

[0.0.7]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.7

[0.0.6]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.6

[0.0.5]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.5

[0.0.4]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.4

[0.0.3]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.3

[0.0.2]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.2

[0.0.1]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.1

[0.0.0]: https://github.com/sunsided/pddl-rs/releases/tag/0.0.0
