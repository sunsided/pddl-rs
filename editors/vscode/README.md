# PDDL (pddl-lsp)

Language support for the Planning Domain Definition Language (PDDL 3.1):
error checking, navigation, rename, completion and formatting for domain and
problem files. It's powered by [`pddl-lsp`](https://github.com/sunsided/pddl-rs/tree/main/pddl-lsp), a language server
written in Rust, which comes with the extension. There's nothing else to
install.

## Features

### Error checking

Mistakes are underlined as you type: every syntax error in a file, not just
the first, and also:

- unknown predicates, functions, types, constants and objects,
- predicates and functions called with the wrong number of arguments,
- variables that no parameter list or quantifier declares,
- duplicate definitions.

A problem is checked against the domain named in its `(:domain ...)`. That
domain can be in the same file, in another open file, or in any `.pddl` file
in the workspace.

Types, constants, predicates, functions, objects and parameters that nothing
uses are shown faded.

A misspelled keyword, such as `(:predicats ...)`, comes with a quick fix
that corrects it.

### Navigation

- **Hover** over a name to see its definition, such as
  `(at ?t - truck ?p - place)` for a predicate.
- **Go to Definition** (F12) jumps from a predicate in a problem's `:init` to
  its declaration in the domain, from a variable to its parameter, and so on.
- **Find All References** (Shift+F12) lists every use across domain and
  problem files.
- **Outline** and **Go to Symbol**, in the file or the whole workspace, list
  types, constants, predicates, functions, actions and objects.
- Lists that span several lines can be folded.

### Editing

- **Rename** (F2) a predicate, function, type, constant, object, action,
  domain or variable everywhere it's used, including in other files.
- **Completion** suggests section and action keywords after `:`, requirements
  in `:requirements`, predicates and functions after `(`, types after `-` and
  in `(either ...)`, domain names in `(:domain ...)`, `minimize` and
  `maximize` in `:metric`, objects and constants, and the variables in scope
  after `?`. In `:init` it leaves out words that can't start a fact.
- **Signature help** shows a predicate's parameters while you type its
  arguments.
- **Inlay hints** show parameter names before arguments, such as
  `(at ?t: t1 ?p: home)` in a problem's `:init`. An argument that already has
  the parameter's name gets none. Turn them off with `editor.inlayHints.enabled`.
- **Snippets** for a domain, problem, action, durative action, derived
  predicate, `:functions`, `:constraints`, `:metric`, `forall`, `exists`,
  `when`, `imply`, `preference`, `increase`, `decrease`, `at start`, `at end`,
  `over all` and a timed initial literal (`at time`): type the name, such as
  `domain` or `:action`, and pick the snippet. Names of two words also work
  without the space, such as `atstart`.
- **Format Document** indents by nesting, puts each section and each
  `:keyword value` pair on its own line, and keeps short lists on one line.
  It keeps your comments and blank lines.

### Planning

**PDDL: Solve with planutils** runs a [planutils](https://pypi.org/project/planutils)
planner on the open problem and the domain its `(:domain ...)` names, as a
task in the problem's folder: `planutils run <planner> <domain> <problem>`.
Its terminal keeps the planner's output.
Planners such as `lama-first` write the plan to `sas_plan` there. Install
planutils with `pip install planutils` and a planner with
`planutils install lama-first`. Desktop only, in a trusted folder.

**PDDL: Validate Plan with VAL** checks the open plan (a `.plan`, `sas_plan`
or `sas_plan.N` file) with [VAL](https://github.com/KCL-Planning/VAL) against
its problem and domain, and marks the step whose precondition fails, or the
last step if the goal isn't reached, with VAL's repair advice. The problem is
the only one in the plan's folder, or the one you pick the first time. Go to
Definition on a step's action opens it in the domain. Install VAL with
`planutils install val` and set `pddl.validator` to `["val", "Validate"]`, or
put VAL's `Validate` on `PATH`. Desktop only, in a trusted folder.

**PDDL: Solve with Planning-as-a-Service** sends the open problem and its
domain to a [Planning-as-a-Service](https://github.com/AI-Planning/planning-as-a-service)
server, runs `pddl.planner` there and opens the plan it returns. It is off
until you set `pddl.planningAsAService.url` in your user settings, such as
`http://localhost:5001` for a server you run with
`planutils install server-environment`, and it asks before it sends the files
to a server for the first time. Works on the web too, where the server allows
it (CORS).

### Highlighting

Syntax highlighting for keywords, variables, numbers and comments. Semantic
highlighting then colors types, predicates, functions, actions, objects and
variables differently, depending on what each name refers to.

PDDL embedded elsewhere is highlighted too:

- Markdown code blocks marked ` ```pddl `.
- Strings in Rust, JavaScript, TypeScript and Python that follow a
  `language=pddl` comment, also inside Markdown code blocks:

  ```rust
  const DOMAIN: &str =
      // language=pddl
      r#"(define (domain d) (:predicates (at ?x)))"#;
  ```

  This covers Rust raw and normal strings, JavaScript and TypeScript
  template literals, and Python triple-quoted strings (`# language=pddl`).
  The string has to be the next one after the comment.

Only highlighting applies there: the Markdown preview, error checking and
the other features above don't cover embedded PDDL.

## Settings

| Setting             | Default | Description |
|---------------------|---------|-------------|
| `pddl.server.path`  | (empty) | Path to a native `pddl-lsp` binary. If empty, the extension uses the server it comes with. Ignored on the web. |
| `pddl.planner`      | `lama-first` | The planutils planner that **PDDL: Solve with planutils** runs. |
| `pddl.validator`    | `["Validate"]` | The VAL command, as a program and its first arguments; the extension appends `-v <domain> <problem> <plan>`. |
| `pddl.planningAsAService.url` | (empty) | A Planning-as-a-Service server for **PDDL: Solve with Planning-as-a-Service**; empty turns it off. User settings only. |
| `pddl.trace.server` | `off`   | Logs the messages between VS Code and the server to the "PDDL Language Server" output panel. |

The **PDDL: Restart Language Server** command restarts the server.

In [Restricted Mode](https://code.visualstudio.com/docs/editing/workspaces/workspace-trust),
only syntax highlighting is available. The server starts once you trust the
folder, and a workspace can only set `pddl.server.path` after that.

### On the web

The extension also works in VS Code for the Web, such as
[vscode.dev](https://vscode.dev) and github.dev, with the same bundled server.
A native server can't run there.
The server's WASI runtime needs a cross-origin isolated page
(`SharedArrayBuffer`). If the extension reports that the page isn't, add
`?vscode-coi=on` to the vscode.dev URL.

### Using a native server

The extension runs the server as WebAssembly, in Microsoft's
[WASI runtime](https://marketplace.visualstudio.com/items?itemName=ms-vscode.wasm-wasi-core),
which VS Code installs along with this extension. A native build of the
server skips that runtime, which may help with very large workspaces. To use
one, install it with Rust:

```shell
cargo install --locked pddl-lsp
```

Then set `pddl.server.path` to `pddl-lsp`, or to the binary's full path if
it's not on your `PATH`.

## How domains are found

When the server starts, it reads every `.pddl` file in the workspace folders.
It skips hidden folders, `target` and `node_modules`. It sees files change on
disk, and for open files it always uses the editor's text. The bundled server
can only read files inside the workspace folders, so a domain outside them is
found only while it's open.

When several domains share a name, a problem uses the one in its own file.
Otherwise it uses the one whose file path sorts first.

## Limitations

- Argument types are not checked: `(at truck1 truck2)` passes even if the
  second parameter is a `place`.
- Requirements are not checked: using `forall` without
  `:universal-preconditions` is not reported.
- After a syntax error, checking resumes at the next section or action, so a
  section reports at most one syntax error.
- Formatting always uses two-space indents and an 80-column limit, and skips
  a file whose parentheses don't match.
- If another PDDL extension, such as `jan-dolejsi.pddl`, is also installed,
  both check `.pddl` files and you'll see duplicate diagnostics. Disable one
  of them for the workspace.

## Feedback and contributing

Report bugs and ideas at
[github.com/sunsided/pddl-rs/issues](https://github.com/sunsided/pddl-rs/issues).
To build, test or publish the extension yourself, see
[CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either the European Union Public Licence 1.2 or the Mozilla
Public License 2.0, at your option; see [LICENSE.md](LICENSE.md).
