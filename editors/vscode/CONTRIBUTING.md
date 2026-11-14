# Contributing to the VS Code extension

This folder holds the VS Code extension for [`pddl-lsp`](../../pddl-lsp).
[README.md](README.md) is the user guide, and it's also what the Marketplace
shows. This file covers building, testing and publishing.

## Requirements

- Node.js 22 or later
- [Yarn](https://classic.yarnpkg.com/) 1; run `corepack enable` if `yarn` is missing
- Rust with the WebAssembly target: `rustup target add wasm32-wasip1-threads`
- [Task](https://taskfile.dev)

## Building and installing

```shell
task install
```

Run that in this folder, or run `task vscode:install` from the repository root.
It builds the server as WebAssembly, compiles the extension, packages both as
`pddl-lsp-<version>.vsix` and installs that into VS Code. Reload the VS Code
window afterwards.

The [Taskfile](Taskfile.dist.yaml) has a task for each step. From the
repository root, put `vscode:` in front of each name, as in `task vscode:package`.

| Task           | What it does |
|----------------|--------------|
| `task deps`    | Installs the dependencies with `yarn install --frozen-lockfile` |
| `task build`   | Compiles `src/` to `out/` |
| `task watch`   | Recompiles on every change (alias: `task dev`) |
| `task server`  | Builds the server as WebAssembly into `server/pddl-lsp.wasm` |
| `task test`    | Runs the extension and its server in a downloaded VS Code |
| `task package` | Packages the extension and the server as `pddl-lsp-<version>.vsix` |
| `task install` | Packages the extension and installs it into VS Code |
| `task publish` | Publishes the extension on `main` to the Marketplace, via GitHub Actions |
| `task clean`   | Removes `out/`, `server/`, `node_modules/`, `.vsix` files and the downloaded VS Code |

Without Task, run the same steps by hand:

```shell
cargo build --release -p pddl-lsp --target wasm32-wasip1-threads
mkdir -p server
cp ../../target/wasm32-wasip1-threads/release/pddl-lsp.wasm server/
yarn install --frozen-lockfile
yarn run compile
yarn run package
code --install-extension pddl-lsp-<version>.vsix
```

## How the extension runs the server

By default, [`src/extension.ts`](src/extension.ts) runs `server/pddl-lsp.wasm`
with `@vscode/wasm-wasi-lsp`, in the WASI runtime of the
`ms-vscode.wasm-wasi-core` extension. The runtime mounts the workspace folders
under `/workspace` inside the server. The client's URI converters translate
between those paths and the real ones, so the server's workspace scan matches
the open documents, and the locations it returns lead to the real files. If
`pddl.server.path` is set, the extension starts that binary instead, without
URI conversion.

## Testing

`task test` downloads VS Code, installs `ms-vscode.wasm-wasi-core` into a
fresh profile, and runs [`test/suite.js`](test/suite.js) against the files in
[`test/workspace`](test/workspace). It checks diagnostics, go to definition
across files, and hover. It doesn't touch your own VS Code setup. On Linux
without a display, it runs VS Code under `xvfb-run`. CI runs the same test.

The server itself is tested with `cargo test -p pddl-lsp` from the repository
root.

## Publishing

The extension is published as `sunsided.pddl-lsp`. It's marked as a preview,
because the WASI runtime packages it uses are previews too. The
[`vscode-publish.yml`](../../.github/workflows/vscode-publish.yml) workflow
publishes it with the Marketplace's Trusted Publishing, so no access token is
stored anywhere.

1. Once, at [marketplace.visualstudio.com/manage](https://marketplace.visualstudio.com/manage),
   open the `sunsided` publisher, go to **Trusted Publishing**, and create a
   policy with:

   | Field            | Value                |
   |------------------|----------------------|
   | Repository owner | `sunsided`           |
   | Repository       | `pddl-rs`            |
   | Workflow         | `vscode-publish.yml` |
   | Environment      | (empty)              |

2. Raise `version` in `package.json`; the Marketplace rejects a version it
   already has. Merge that into `main`.

3. Run `task publish`, which asks for confirmation and then starts the
   workflow on `main` with the GitHub CLI. You can also start it from the
   repository's Actions tab. The workflow builds the server and the extension
   and publishes them.

Relative links in README.md become GitHub URLs on `main` when the extension is
packaged, so they work on the Marketplace page.
