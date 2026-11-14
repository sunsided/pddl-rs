# PDDL for VS Code (pddl-lsp)

Adds PDDL support to VS Code: syntax highlighting, plus everything the
[`pddl-lsp`](../../pddl-lsp) language server does. That includes
diagnostics, hover, go to definition, references, rename, completion,
signature help, formatting and quick fixes.

## Setup

1. Install the language server, which needs Rust:

   ```shell
   cargo install --git https://github.com/sunsided/pddl-rs pddl-lsp
   ```

2. Build and install the extension, which needs Node.js:

   ```shell
   npm ci
   npx vsce package
   code --install-extension pddl-lsp-0.1.0.vsix
   ```

   With [Task](https://taskfile.dev), `task install` in this folder does the
   same, or `task vscode:install` from the repository root.

3. Open a `.pddl` file.

If the extension can't find the server, it shows an error with a button that
opens the settings.

## Settings

| Setting             | Default    | Description                                                     |
|---------------------|------------|-----------------------------------------------------------------|
| `pddl.server.path`  | `pddl-lsp` | Path to the server binary. The default looks it up on `PATH`.   |
| `pddl.trace.server` | `off`      | Logs the protocol messages to the "PDDL Language Server" output panel. |

The **PDDL: Restart Language Server** command restarts the server, for example
after you installed a new version.

## Other PDDL extensions

If you also have another PDDL extension installed, such as `jan-dolejsi.pddl`,
both will handle `.pddl` files and you'll see duplicate diagnostics. Disable
one of them for the workspace.
