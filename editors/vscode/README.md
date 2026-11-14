# PDDL for VS Code (pddl-lsp)

Adds PDDL support to VS Code: syntax highlighting, plus everything the
[`pddl-lsp`](../../pddl-lsp) language server does. That includes
diagnostics, hover, go to definition, references, rename, completion,
signature help, formatting and quick fixes.

The extension bundles the server compiled to WebAssembly, so you don't need to
install anything else. VS Code runs the server with Microsoft's
[WASI runtime extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode.wasm-wasi-core)
(`ms-vscode.wasm-wasi-core`), which VS Code installs along with this extension.

## Setup

Building the extension needs:

- Node.js
- [Yarn](https://classic.yarnpkg.com/) 1; run `corepack enable` if `yarn` is missing
- Rust with the WebAssembly target: `rustup target add wasm32-wasip1-threads`
- [Task](https://taskfile.dev)

Build and install it with:

```shell
task install
```

Run that in this folder, or run `task vscode:install` from the repository root.
It builds the server as WebAssembly, compiles the extension, packages both as
`pddl-lsp-<version>.vsix` and installs that into VS Code. Then open a `.pddl`
file.

## Building

The [Taskfile](Taskfile.dist.yaml) has a task for each step. From the
repository root, put `vscode:` in front of each name, as in `task vscode:package`.

| Task           | What it does                                                         |
|----------------|----------------------------------------------------------------------|
| `task deps`    | Installs the dependencies with `yarn install --frozen-lockfile`      |
| `task build`   | Compiles `src/` to `out/`                                            |
| `task watch`   | Recompiles on every change (alias: `task dev`)                       |
| `task server`  | Builds the server as WebAssembly into `server/pddl-lsp.wasm`         |
| `task test`    | Runs the extension and its server in a downloaded VS Code            |
| `task package` | Packages the extension and the server as `pddl-lsp-<version>.vsix`   |
| `task install` | Packages the extension and installs it into VS Code                  |
| `task publish` | Publishes the extension and the server to the VS Code Marketplace    |
| `task clean`   | Removes `out/`, `server/`, `node_modules/`, `.vsix` files and the downloaded VS Code |

`task test` opens the PDDL files in [`test/workspace`](test/workspace) in a
fresh VS Code profile, so it doesn't touch your setup. On Linux without a
display, it runs VS Code under `xvfb-run`.

Without Task, run the same steps by hand:

```shell
cargo build --release -p pddl-lsp --target wasm32-wasip1-threads
mkdir -p server
cp ../../target/wasm32-wasip1-threads/release/pddl-lsp.wasm server/
yarn install --frozen-lockfile
yarn run compile
yarn run package
code --install-extension pddl-lsp-0.1.0.vsix
```

## Publishing

The extension is published as `sunsided.pddl-lsp` and marked as a preview,
because the WASI runtime packages it uses are previews too.

1. Create a personal access token in Azure DevOps with the
   **Marketplace (Manage)** scope, and log in with it once:

   ```shell
   npx vsce login sunsided
   ```

   In CI, set the `VSCE_PAT` environment variable to the token instead.

2. Raise `version` in `package.json`; the Marketplace rejects a version it already has.

3. Run `task publish`, which asks for confirmation first. It builds the
   server and the extension and publishes them. Relative links in this README
   are rewritten to GitHub URLs on the `main` branch, so push the README
   before publishing.

## Settings

| Setting             | Default | Description |
|---------------------|---------|-------------|
| `pddl.server.path`  | (empty) | Path to a native `pddl-lsp` binary. Set it to `pddl-lsp` to use the one on `PATH`. If empty, the extension runs its bundled WebAssembly server. |
| `pddl.trace.server` | `off`   | Logs the protocol messages to the "PDDL Language Server" output panel. |

The bundled server is the simplest choice. A native binary skips the
WebAssembly runtime, which may help with very large workspaces. Install it with
`cargo install --git https://github.com/sunsided/pddl-rs pddl-lsp`.

The **PDDL: Restart Language Server** command restarts the server, for example
after you installed a new version.

## How the bundled server sees files

The WASI runtime mounts the workspace folders under `/workspace` inside the
server, and the extension translates file URIs between the two. Go to
definition and references therefore lead to the real files. The server can
only read files inside the workspace folders, so it can't find a domain in a
`.pddl` file outside them unless that file is open.

## Other PDDL extensions

If you also have another PDDL extension installed, such as `jan-dolejsi.pddl`,
both will handle `.pddl` files and you'll see duplicate diagnostics. Disable
one of them for the workspace.
