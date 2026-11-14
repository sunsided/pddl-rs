# PDDL (pddl-lsp)

Language support for the Planning Domain Definition Language (PDDL 3.1):
error checking, navigation, rename, completion and formatting for domain and
problem files. It's powered by [`pddl-lsp`](../../pddl-lsp), a language server
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
  in `:requirements`, predicates and functions after `(`, types after `-`,
  objects and constants, and the variables in scope after `?`.
- **Signature help** shows a predicate's parameters while you type its
  arguments.
- **Format Document** indents by nesting, puts each section and each
  `:keyword value` pair on its own line, and keeps short lists on one line.
  It keeps your comments and blank lines.

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
| `pddl.server.path`  | (empty) | Path to a native `pddl-lsp` binary. If empty, the extension uses the server it comes with. |
| `pddl.trace.server` | `off`   | Logs the messages between VS Code and the server to the "PDDL Language Server" output panel. |

The **PDDL: Restart Language Server** command restarts the server.

In [Restricted Mode](https://code.visualstudio.com/docs/editing/workspaces/workspace-trust),
only syntax highlighting is available. The server starts once you trust the
folder, and a workspace can only set `pddl.server.path` after that.

### Using a native server

The extension runs the server as WebAssembly, in Microsoft's
[WASI runtime](https://marketplace.visualstudio.com/items?itemName=ms-vscode.wasm-wasi-core),
which VS Code installs along with this extension. A native build of the
server skips that runtime, which may help with very large workspaces. To use
one, install it with Rust:

```shell
cargo install --git https://github.com/sunsided/pddl-rs pddl-lsp
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
