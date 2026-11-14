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

2. Build and install the extension. This needs Node.js, [Yarn](https://classic.yarnpkg.com/)
   (run `corepack enable` if `yarn` is missing) and [Task](https://taskfile.dev):

   ```shell
   task install
   ```

   Run it in this folder, or run `task vscode:install` from the repository root.
   It installs the dependencies, compiles the extension, packages it as
   `pddl-lsp-<version>.vsix` and installs that into VS Code.

3. Open a `.pddl` file.

If the extension can't find the server, it shows an error with a button that
opens the settings.

## Building

The [Taskfile](Taskfile.dist.yaml) has a task for each step. From the
repository root, put `vscode:` in front of each name, as in `task vscode:package`.

| Task           | What it does                                                   |
|----------------|----------------------------------------------------------------|
| `task deps`    | Installs the dependencies with `yarn install --frozen-lockfile` |
| `task build`   | Compiles `src/` to `out/`                                      |
| `task watch`   | Recompiles on every change (alias: `task dev`)                 |
| `task package` | Packages the extension as `pddl-lsp-<version>.vsix`            |
| `task install` | Packages the extension and installs it into VS Code            |
| `task clean`   | Removes `out/`, `node_modules/` and `.vsix` files              |

Without Task, run the same steps with Yarn:

```shell
yarn install --frozen-lockfile
yarn run compile
yarn run package
code --install-extension pddl-lsp-0.1.0.vsix
```

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
