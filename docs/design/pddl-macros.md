# Design: compile-time PDDL macros (Level A)

Implemented in `pddl-macros` (sections 6-8), except the problem-against-domain
check (8.4). Covers #122 (Level A) and the parts of #121 it needs. Takes #132
and #131 into account. Line references are to commit 1381c8f.

## 1. What the issues ask for

None of the four issues has comments. Every decision below comes from the issue bodies.

- **#132 (tracking).** The issue embeds PDDL through a proc macro and `build.rs`. Its levels:
  - **A**: validate at compile time, emit `LazyLock<Domain>`.
  - **B1**: lifted domain becomes typed ids, a `State` struct, a `Copy` action enum. Problem loads at runtime.
  - **B2**: fully grounded, `[u64; N]` state, `const` mask tables.
  - **C** (`dyn Action`): dropped in favour of static dispatch through a `Task` trait.

  Already decided: input is a string literal or a file. Token-tree input is
  **out**, because `'` in `;` comments and `pick-up` break Rust's tokenizer.
  The crate list is `pddl`, `pddl-codegen`, `pddl-macros` and `pddl-runtime`.
  Durative actions and PDDL3 preferences emit `compile_error!("unsupported: ...")`, but only from B1 on.
- **#121.** This issue turns the repo into a workspace with `pddl` (unchanged), `pddl-codegen` (AST to
  `TokenStream`, shared with `build.rs`), `pddl-macros` (thin `proc-macro = true`)
  and later `pddl-runtime`. Each crate versions and publishes on its own.
  Part of this is already done: the root is a workspace with `pddl` as root package
  and `pddl-lsp` as member (`Cargo.toml:38-39`). Per-crate release commits exist (`release.toml:12-15`).
- **#122 (Level A).** Validate with `Parser::from_str` and report `compile_error!` with line:col.
  The span covers the whole literal, because `Literal::subspan` is nightly-only.
  The file form resolves paths against `CARGO_MANIFEST_DIR` and emits `include_bytes!` so
  Cargo rebuilds. Output is `LazyLock::new(|| Domain::from_str(SRC).expect(..))`.
  **Still open**: one `pddl!` for both kinds, or separate `domain!` and `problem!`.
  Acceptance: IPC files compile, a trybuild test covers the error, and editing a file triggers a rebuild.
- **#131 (after B1).** This issue binds PDDL types to Rust types, e.g. `types { robot => crate::Robot }`,
  with values in a side arena. It also binds predicates or functions to pure Rust functions.
  Open questions: syntax (macro arguments or a side config file), and whether attachments may read `State`.

## 2. Token input versus string and file input

`pddl! { (define ...) }` as Rust tokens fails on real PDDL:

| PDDL | What `proc_macro` delivers | Consequence |
|---|---|---|
| `?x` | `Punct('?')`, `Ident(x)` | `?x` and `? x` look the same without span positions |
| `total-cost`, `pick-up` | `Ident`, `Punct('-')`, `Ident` | the name must be glued back from span adjacency, and `?x - t` (type) must stay apart |
| `:action-costs` | `Punct(':')`, `Ident`, `Punct('-')`, `Ident` | same gluing |
| `; it's done` | `;` then tokens of the comment | `'s` lexes as a lifetime. A lone `'`, `"` or unbalanced `(` in a comment is a **lexer error before the macro runs**. |
| `AT` / `at` | idents keep their case | fine; `Name` compares ignoring case (`src/types/terms/name.rs:217-222`) |
| `1.5`, `10` | literals | fine. But Rust also accepts `1e3`, `0x10` and `1u8`, which PDDL rejects (`src/parsers/terms/number.rs:19-20`). |
| `move`, `type`, `in` | idents (keywords are idents in token streams) | fine |

`proc_macro` drops whitespace and comments entirely. Rebuilding the text needs
`Span::start()`/`end()` line and column. Those are stable only from Rust 1.88
(verify against the release notes), and the crate's MSRV is 1.86 (`Cargo.toml:15`).
proc-macro2's `span-locations` falls back to fake positions on older compilers.
The parser works on text (`src/parsers/sexpr.rs:57`, comment handling at `:67-73`).
Token input would need a second front end that produces text. That front end
still could not take real IPC files, because of the comment problem above.
Token input's only gain is per-token spans for errors.

**Conclusion: keep the #132/#122 decision, string and file input only.**

- `pddl::domain!(r#"..."#)`: parse the literal with `syn::LitStr::value()`, which handles escapes and raw strings.
  Raw strings are the documented form.
- `pddl::domain_file!("domains/blocks.pddl")`: the macro reads
  `$CARGO_MANIFEST_DIR/<path>` with `std::fs`. This differs from `include_str!`, which
  resolves against the calling file. Finding the calling file needs `Span::local_file`, which is also 1.88+.
  Document the difference.
  - **Rebuild tracking.** `proc_macro::tracked_path` is unstable. The expansion embeds
    the source with `include_str!("<absolute path>")`. That writes the file into rustc's dep-info,
    so Cargo recompiles the crate when the file changes, and proc macros re-expand on every compile.
    This replaces the separate `const _: &[u8] = include_bytes!(..)` from #122: the include is the source.
    Build the path literal with `LitStr::new` so Windows backslashes get escaped.
  - The macro embeds the text it validated as a literal. The `include_str!` sits in an unused
    `const _` and only feeds the dep-info. Embedding `include_str!`'s own read instead would let a file
    rewritten mid-build ship unchecked PDDL.
  - If `Span::local_file` is `None` (tokens from another macro, some IDE expansions), paths resolve
    against `CARGO_MANIFEST_DIR`. The "couldn't read" error names the absolute path it tried.

## 3. Diagnostics

Two existing building blocks:

- `ParseError` holds a byte `span()`, a 1-based `line()`/`column()` in characters, `reason()`, `found()` and `help()`
  (`src/parsers/error.rs:31-55`, `:57-118`). Its `Display` gives
  `line 2, column 3: unknown section: unexpected `(:bogus))`` (`error.rs:28`).
- `pddl::parsers::check(&str) -> Vec<ParseError>` (`src/parsers/structure/pddl_file.rs:61-85`, re-exported at
  `src/parsers.rs:108`) reports at most one error per section, action or top-level form.
  It is empty exactly when `PddlFile::from_str` succeeds (`pddl_file.rs:44-50`).

Plan:

1. Run `check(src)`. Map every error to `syn::Error::new(span, msg)` and fold them with
   `syn::Error::combine`. `to_compile_error()` then emits one `compile_error!` per error,
   so the user sees all of them in one build.
2. If `check` passes, run `Domain::from_str` (or `Problem::from_str`). That only catches a wrong kind or a second
   definition, which `check` accepts as a valid `PddlFile`. Report it as "expected a domain, found a problem".
3. Message format: `<Display>` + `\n  = help: <help>` when present. Then the offending source line with a `^~~~` marker
   under `span()`, about 15 lines of code in the macro crate.
   The file form prefixes `blocks.pddl:12:5:` so terminals and IDEs can jump to it.
4. Spans: stable Rust has no sub-span of a literal (`Literal::subspan` needs the nightly
   `proc_macro_span`). Every error therefore points at the whole string literal (or the path literal), and the
   line:col in the message does the locating. Leave out any nightly `cfg` path in v1.
   Optional later: add the literal's start line (`Span::start()`, 1.88+) to report a line number in the `.rs` file.
5. Don't pull in `miette` rendering. Its graphical handler would add a heavy host dependency
   to every build for a marker we draw in a few lines.

## 4. Output

| Option | Expansion | Macro cost | Runtime | Notes |
|---|---|---|---|---|
| (a) validated `&'static str` | the literal / `include_str!` | trivial | user parses | works in `const`; user chooses `Domain`, `PddlFile`, `syntax::read` |
| (b) `LazyLock<Domain>` | (a) + a lazy parse | trivial | one parse on first use | #122's choice; `LazyLock` is stable since 1.80 |
| (c) const AST | `TokenStream` of the full AST | large | none | not possible today, see below |

(c) cannot be `const` or a plain `static` without reshaping the public AST:

- Non-empty `Vec` and `Box` cannot be built in const context. They are everywhere:
  `Domain.extends: Vec<Name>` (`src/types/structure/domain.rs:73`),
  `GoalDescription::And(Vec<_>)`/`Not(Box<_>)` (`src/types/goals/gd.rs:22-37`),
  `Requirements(Vec<Requirement>, Vec<Name>)` (`src/types/structure/requirements.rs:69`).
- `Name::new_static` is `const` (`name.rs:146`), so names alone would work. But `Name::new` interns through a global
  `LazyLock<Mutex<_>>` of `Arc<str>` (`name.rs:15`, `:34-48`), and static names bypass the interner (`name.rs:127-131`).
- The builders are not `const` (`Domain::builder`, `domain.rs:107`). Only 67 `const fn new` exist across the types.
- A non-const variant (builder calls inside a `LazyLock`) would only skip a parse that takes micro- to milliseconds.
  It would need AST-to-`TokenStream` code for about 100 types, which is the B1 `pddl-codegen` work anyway.

Cost against use: Level A buys "syntax errors at compile time" and nothing else. (b) is #122's
decision and costs the same as (a). **v1 ships (b)**, built so that (a) falls out of it.

## 5. Crate layout

> **Update:** the facade split described at the end of this section is done. The types and the parser
> are now `pddl-core` (the root package), `pddl/` is a facade that re-exports it, and its `macros`
> feature re-exports `pddl-macros`, which depends on `pddl-core`.

Cycle check: `pddl-macros` must run the parser on the host, so it depends on
`pddl = { path = "..", default-features = false, features = ["parser"] }`. That mirrors `pddl-lsp/Cargo.toml:20`.
If `pddl` also re-exported the macros (`macros = ["dep:pddl-macros"]`), the graph
would be `pddl -> pddl-macros -> pddl`. Cargo rejects that as a cyclic package dependency:
the proc-macro crate being a host dependency does not help, and only dev-dependency cycles are allowed.

serde avoids the cycle because `serde_derive` does not depend on `serde`. It only emits paths into it.
`pddl`'s serde support is hand-written anyway (`src/serde.rs:105-109`, `:199-236`), so there is no
derive to copy. Crates whose macros need the library's logic split it out instead.
sqlx, for example, has `sqlx-core` <- `sqlx-macros-core` <- `sqlx-macros` <- `sqlx`.

**v1: no re-export.**

```text
pddl          (root, unchanged; no new feature)
pddl-macros   proc-macro = true; deps: pddl[parser], syn (parsing, proc-macro, printing), quote, proc-macro2
```

- Users add both crates: `pddl` and `pddl-macros`. The expansion names `::pddl::...`. A renamed `pddl` dependency is
  unsupported in v1. See the open questions.
- Version skew: none in practice. Cargo picks one version per semver-compatible range for the whole build,
  host dependencies included, so the macro validates with the same `pddl-core` the program parses with.
  An exact pin (`=0.4.x`) would only break `pddl`'s `macros` feature on every `pddl-core` release that
  skips `pddl-macros`, so `pddl-macros` uses a plain `0.4` requirement.
  Resolver 2/3 builds host `pddl` separately from target `pddl`, so the macro's features don't leak.
- Wait with `pddl-codegen` until B1. Level A has no AST to convert. A `build.rs` user can already call
  `pddl::parsers::check` and `include_str!` the file without a new crate.
- Re-exporting later as `pddl::domain!` needs the sqlx shape: move the code to `pddl-core` and make
  `pddl` a facade that re-exports `pddl-core` plus `pddl-macros` behind `macros`. That is a bigger
  publishing change, so do it only if users ask for it.

Wiring: add `"pddl-macros"` to `[workspace] members`, give it a `release.toml` like `pddl-lsp`, and add it to CI and
`Taskfile.dist.yaml`.

## 6. v1 API

```rust
use std::sync::LazyLock;
use pddl::{Domain, Problem};

static BLOCKS: LazyLock<Domain> = pddl_macros::domain_file!("domains/blocks.pddl");
static TASK: LazyLock<Problem> = pddl_macros::problem!(r#"
    (define (problem p1) (:domain blocks)
      (:objects a b) (:init (clear a)) (:goal (on a b)))
"#);
const SRC: &str = pddl_macros::validated_file!("domains/blocks.pddl"); // option (a)
static ALL: LazyLock<pddl::PddlFile> = pddl_macros::pddl_file!("ipc/blocks-all.pddl");
// v1.1: cross-check against the domain (section 8.4)
static P2: LazyLock<Problem> = pddl_macros::problem_file!("problems/p2.pddl", domain = "domains/blocks.pddl");

fn main() { assert_eq!(BLOCKS.name(), "blocks"); }
```

Compile error for a typo (whole literal underlined):

```text
error: domains/blocks.pddl:4:4: unknown section: unexpected `(:predicats (on ?x ?y))`
         = help: did you mean `:predicates`?
         (:predicats (on ?x ?y))
          ^~~~~~~~~~
 --> src/main.rs:4:65   (span: the whole path literal)
```

Expansion of `domain_file!("domains/blocks.pddl")`:

```rust
::std::sync::LazyLock::new(|| {
    const SRC: &str = ::core::include_str!("/abs/crate/domains/blocks.pddl");
    <::pddl::Domain as ::pddl::Parser>::from_str(SRC)
        .expect("PDDL validated at compile time by pddl-macros")
})
```

The closure captures nothing, so it coerces to `fn() -> Domain` and `LazyLock::new` works in a `static`.
`Domain: Sync` holds because the AST uses `Arc`, not `Rc`, and no `Cell` types.
The `expect` can fire only if the host and target `pddl-core` differ, which one build never has (section 5).

Implementation: one `fn expand(kind: Kind, input: proc_macro2::TokenStream) -> syn::Result<TokenStream2>`
behind eight one-line `#[proc_macro]` shims (`domain`, `problem`, `pddl`, `validated`, each with a `_file` form). The input grammar is exactly one string literal, plus an optional
trailing comma. Reject anything else with "expected a string literal", which leaves room for the
named arguments of #131.

## 7. Test plan

- **Unit tests in `pddl-macros`**: call `expand` on `proc_macro2` tokens directly, which works outside rustc.
  Cover valid and broken input, wrong kind, several `check` errors (one `compile_error!` each),
  escapes in non-raw strings, a missing file, and non-UTF-8 content.
- **Integration tests (`pddl-macros/tests/*.rs`)**: `domain_file!`/`problem_file!` over a few corpus files from
  `../tests/data`, asserting the `LazyLock` result equals `Domain::from_str` of the same file.
  The path is relative to `CARGO_MANIFEST_DIR`. `tests/data` is excluded from the `pddl` package
  (`Cargo.toml:12`), which is fine for workspace tests.
- **trybuild compile-fail** (`tests/ui/*.rs` + `.stderr`) covers four cases:
  - a typo with a `help` line
  - two errors in one literal
  - a problem passed to `domain!`
  - a non-literal argument

  The `.stderr` files depend on the rustc version, so run trybuild only on the pinned stable toolchain, not the MSRV job.
  Inline literals keep the UI tests independent of trybuild's generated `CARGO_MANIFEST_DIR`.
- **Rebuild check** (#122 acceptance), as a CI step script:
  1. Build a fixture crate.
  2. Append a broken line to its `.pddl` file.
  3. Assert that `cargo build` now fails.
  4. Restore the file and assert the build passes again.

## 8. Decisions (maintainer, 2026-11-19)

1. **Names.** `domain!`, `problem!`, `domain_file!` and `problem_file!` as recommended.
   `pddl!`/`pddl_file!` also ship, for the `PddlFile` case: any number of definitions, giving `LazyLock<PddlFile>`.
   They do no kind inference.
2. **Option (a).** Ships too: `validated!`/`validated_file!` give a checked `&'static str`.
   Same `expand` front end, but no parse in the output.
3. **Crate path.** The expansion hardcodes `::pddl`. There is no `crate = ..` argument and no `proc-macro-crate`.
4. **Problem against domain.** Wanted, as **v1.1** (after the v1 macros land).
   - **API:** `problem!(r#"..."#, domain = "domains/blocks.pddl")` and `problem_file!("p.pddl", domain = "d.pddl")`.
     The domain must be given as a path or literal. A proc macro cannot read the value of another `static`,
     so `domain = BLOCKS` is not possible. The extra argument fits the "one literal, then named args" grammar of section 6.
   - **Checker:** it exists already, in `pddl-lsp`:
     - `pddl-lsp/src/analysis.rs` collects definitions and references from `pddl::syntax` only, with no `lsp_types`.
     - `Workspace::problems` (`pddl-lsp/src/workspace.rs:296`) reports unknown names, wrong arity and duplicates,
       checking a problem against the domain named in `(:domain ..)` (tests at `workspace.rs:874`, `:1154`).
   - **Prerequisite refactor:**
     - Move the syntax-only analysis into `pddl` as `pddl::analysis`, behind the `parser` feature.
     - Split the cross-file check off `Workspace` into a function over `(&str domain, &str problem)` that returns
       byte ranges and messages.
     - `pddl-lsp` then maps those to `Url` + `LineIndex`, and `pddl-macros` maps them to compile errors.
     One checker, two front ends, no duplicated semantics.
   - Also check that the problem's `(:domain name)` matches the domain's name.
5. **MSRV.** `pddl-macros` uses Rust 1.88. `pddl` itself stays at 1.86, because the macro crate needs no
   newer `pddl`. 1.88 changes three things:
   - `*_file!` paths resolve against the calling file via `Span::local_file()`, like `include_str!`.
     If that returns `None` (some IDE expansions), fall back to `CARGO_MANIFEST_DIR`. This replaces section 2's rule.
   - Errors report the PDDL line in the `.rs` file for inline literals: the literal's `Span::start().line()` plus the PDDL line.
   - Spans still cover the whole literal (`Literal::subspan` stays nightly).

## 9. How later levels build on v1

- **B1 (#126).** Add `pddl-codegen` (a plain lib: AST to `TokenStream`). Its first user is a new macro shape,
  because B1 emits items, not an expression:
  `#[pddl_macros::domain(file = "blocks.pddl")] mod blocks {}` fills the module with `State`, the typed ids and the
  action enum. The same `expand` front end is reused: read, `check`, diagnostics, file tracking.
  `build.rs` calls `pddl_codegen::generate(&domain)` and `include!`s the result from `OUT_DIR`.
  The `unsupported: ...` errors from #132 come from codegen, with the same diagnostics path.
- **B2 (#127).** Same attribute, with both the domain and the problem given. Codegen emits `const` mask tables.
  This is where const data finally appears, in generated tables, not in the `pddl` AST (section 4).
- **#131 bindings.** These extend the attribute arguments that v1 deliberately rejects, e.g.
  `#[pddl_macros::domain(file = "..", types(robot = crate::Robot), attach(reachable = crate::nav::reachable))]`.
  Unknown PDDL names become spanned errors on those Rust tokens, so the bindings get exact spans even though the PDDL
  itself does not.
- **Facade.** If the extra dependency bothers users, do the `pddl-core` split from section 5. The
  macro code does not change, only the path it emits.
