# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `pddl::Definition`, a single domain or problem. `Definition::from_str` parses exactly one
  `(define ...)` block and takes its kind from the header, so callers no longer parse a `PddlFile`
  and check that it holds exactly one definition.
- `ParseError::span()`, the byte range of the input that parsing stopped at: an atom, the start of a
  list up to its first atom (e.g. `(:bogus`), a single parenthesis, or an empty range at the end.
- The optional `miette` feature implements `miette::Diagnostic` for `ParseError`, labelling that
  span. Render an error with `miette::Report::new(error).with_source_code(input)`.

### Changed

- [#104](https://github.com/sunsided/pddl-rs/issues/104): The parser is hand-written instead of
  built from `nom` combinators. It reads the input into a tree of atoms and parenthesized lists,
  then turns the tree into the AST, trying the grammar's alternatives in the same order as before.
  The example corpus parses to the same ASTs and errors, about five times as fast (6.3 ms → 1.2 ms
  for 88 files). `nom` and `nom_locate` are no longer dependencies. Where the old parser's token
  boundaries were accidental, the new rules are:
  - Whitespace may follow any `(`. `( and (p) (q))` used to be read as a predicate named `and`, and
    `( define ...)` was an error.
  - Whitespace is optional after a `)` as well as before a `(`, so `(at b roomb)x` reads as two items.
  - A `?` or `:` starts a new atom, so `(on?x ?y)` and `(:requirements:strips)` parse everywhere. The
    old parser accepted them only in some positions.
  - A variable can't contain whitespace: `? x` used to be read as `?x` and is now an error.
  - `(at end <GD>)` in constraints allows any whitespace and comments between `at` and `end`, not
    just one space.
  - `( )` is the empty list like `()`, e.g. for an action without precondition.
- An invalid element of a list, such as `5` in `(:init (p a) 5)`, is reported at that element
  without the reason ``expected `)` ``. A missing `)` is reported at the end of the input, but only
  if no other mistake comes first.

- The minimum supported Rust version is now 1.86 (was 1.80), as required by the `criterion`
  benchmark dependency.
- `Number` stores an `f64` instead of an `f32`, so `Deref` and `Number::new` / `try_new` use `f64`.
  Whole numbers are exact up to 2^53 instead of 2^24: `16777217` used to parse as `16777216`.
  `Number::from` an `f64` beyond the `f32` range, such as `1e300`, used to overflow to infinity and
  panic; it now keeps the value. `From<f32>` widens through the shortest decimal, so
  `Number::from(1.1f32)` is `1.1` and prints as `1.1`. `From<f32>` and `From<f64>` still panic on NaN
  and infinity.
- `Name::len` returns the length in bytes, as `str::len` does, instead of in characters. For a
  non-ASCII name built by hand, such as `Name::new("é")`, it used to return a value that can't be used
  to slice the name. Parsed names are ASCII only, so their lengths don't change.
- A space after an arithmetic operator is optional, so `(+x 1)` parses as `(-x)` already did. The
  equality form accepts whitespace after its `(`, as `( = a b)`.

### Fixed

- `(assign <function-term> undefined)` pretty-prints with `undefined` again. It printed as
  `(assign <function-term>)`, which doesn't parse.
- A function term without terms pretty-prints as `(f)` instead of `f`, which parsed back as the
  name `f`: `(at (loc) y)` printed as `(at loc y)`, and `(assign (loc) y)` turned into a numeric
  assignment.
- `Requirement::Adl.contains` claimed `:continuous-effects` instead of `:conditional-effects`, and
  missed `:existential-preconditions` and `:universal-preconditions`. It now agrees with
  `Requirement::expand`.
- [#143](https://github.com/sunsided/pddl-rs/pull/143): Deeply nested input no longer overflows the
  stack and aborts the process. More than 64 open parentheses is a `ParseError` with the reason
  `nesting too deep`. Nested `(- ...)` in fluent, durative and `:metric` expressions parses in linear
  time; each level used to take about three times as long as the one below.

### Removed

- [#104](https://github.com/sunsided/pddl-rs/issues/104): The nom-based parsing API: the
  `pddl::parsers::parse_*` functions, the combinators `typed_list`, `function_typed_list`,
  `literal`, `atomic_formula` and `empty_or`, `Parser::parse` and `Parser::parse_span`, and the
  types `Span`, `ParseResult`, `NomError` and `UnwrapValue` with `pddl::parsers::preamble`. Parse
  with `T::from_str`; `PddlFile::from_str(input)?.domains` (or `.problems`) replaces
  `parse_domains` and `parse_problems`.
- The inherent `Name::len` and `Name::is_empty`. Method calls still work through `Deref` to `str`;
  path calls such as `Name::len(&name)` don't.
- [#138](https://github.com/sunsided/pddl-rs/issues/138): `pddl::parsers::ignore_eol_comment`. The
  parsers skip whitespace and `;` comments themselves, and nothing used it.

## [0.3.1] - 2026-11-13

[0.3.1]: https://github.com/sunsided/pddl-rs/releases/tag/v0.3.1

### Fixed

- [#119](https://github.com/sunsided/pddl-rs/issues/119): Whitespace and `;` comments before the `)`
  that closes an atomic formula, predicate skeleton, parameter list or function term are accepted, so
  `(p ?x )` and `:parameters (?x\n  )` parse. A function head written as `(f )` now parses to
  `FunctionHead::Simple`, the same as `(f)`, instead of `FunctionHead::WithTerms` with no terms.

## [0.3.0] - 2026-11-13

[0.3.0]: https://github.com/sunsided/pddl-rs/releases/tag/v0.3.0

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
  instead of the start of the enclosing form, and say where and why: the new `ParseError::keyword()`
  names the innermost PDDL form (e.g. `:precondition`), and `ParseError::reason()` returns e.g.
  `duplicate section`, `unknown section`, ``missing `:goal` `` or ``expected `)` `` for a form that
  isn't closed. `Display` includes both. Once a `:keyword` such as `:action`, `:precondition` or
  `:requirements` has matched, a failure inside it is no longer backtracked.
- `pddl::parsers::NomError<'a>`, the error type of the `parse_*` combinators, is now a crate-defined
  struct instead of an alias for `nom::error::Error`. It keeps the public `input` and `code` fields and
  adds `keyword` and `context`.

### Internal

- [#109](https://github.com/sunsided/pddl-rs/issues/109): Keyword parsers (operators, time specifiers,
  requirement keys, …) map each keyword directly to its enum variant instead of converting the matched
  text with `try_from(...).expect(...)`, which removes a per-keyword allocation and a panic path.
- Releases are cut with `cargo release` (see `release.toml`); pushing the `vX.Y.Z` tag publishes to
  crates.io via trusted publishing and creates the GitHub Release from this changelog.
- GitHub Actions workflows are checked with actionlint and zizmor, pin all actions to commit SHAs, and
  run with minimal token permissions.

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
