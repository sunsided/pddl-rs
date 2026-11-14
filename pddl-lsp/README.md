# pddl-lsp

A language server for the Planning Domain Definition Language (PDDL 3.1), built
on the [`pddl`](https://crates.io/crates/pddl) parser. It works with any editor
that speaks the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/).

## Features

- **Diagnostics**: every syntax error in a file, not just the first, plus:
  - unknown predicates, functions, types, constants and objects,
  - predicates and functions called with the wrong number of arguments,
  - variables that no parameter list or quantifier declares,
  - duplicate definitions.

  A problem is checked against the domain named in its `(:domain ...)`. That
  domain can be in the same file, another open file, or any `.pddl` file in the
  workspace.
- **Navigation**: hover, go to definition, find references, highlight uses,
  document outline, workspace symbol search, folding. All of it works across
  files, for example from a problem's `:init` to the domain's `:predicates`.
- **Rename** for predicates, functions, types, constants, objects, actions,
  domains and variables, across domain and problem files.
- **Completion** for section and action keywords, requirements, predicates,
  functions, types, objects, constants and the variables in scope.
- **Signature help** while you type the arguments of a predicate or function.
- **Formatting** that keeps comments and blank lines: one section per line,
  `:keyword value` pairs on their own lines, and lists kept on one line when
  they fit in 80 columns.
- **Quick fixes** for misspelled keywords, such as `:predicats` → `:predicates`.
- **Semantic highlighting**, so types, predicates, functions, actions, objects
  and variables each get their own color.

## Installation

Install the binary with Cargo. Cargo puts it in `~/.cargo/bin`, which needs to
be on your `PATH`:

```shell
cargo install --git https://github.com/sunsided/pddl-rs pddl-lsp
```

Or from a checkout of this repository:

```shell
cargo install --path pddl-lsp
```

Check that it works:

```shell
pddl-lsp --version
```

The server communicates over stdin and stdout and takes no options. It accepts
`--stdio` and ignores it, because some clients always pass that flag.

## Editor setup

Except for VS Code, each example below assumes `pddl-lsp` is on your `PATH`. If it isn't, use the
binary's full path instead.

### VS Code

Use the extension in [`editors/vscode`](../editors/vscode). It adds syntax
highlighting and bundles this server compiled to WebAssembly, so you don't need
to install the binary. It isn't on the Marketplace yet, so build and install it
yourself. This needs Node.js, [Yarn](https://classic.yarnpkg.com/), the
`wasm32-wasip1-threads` Rust target and [Task](https://taskfile.dev). From the
repository root, run:

```shell
task vscode:install
```

This packages the extension and the server as a `.vsix` file and installs it
into VS Code. To only build the `.vsix`, run `task vscode:package`. The
[extension README](../editors/vscode/README.md#building) lists every task and
the equivalent commands.

To use a native binary instead of the bundled one, set `pddl.server.path` to
its path, or to `pddl-lsp` to look it up on `PATH`.

### Neovim 0.11+

```lua
vim.filetype.add({ extension = { pddl = "pddl" } })

vim.lsp.config("pddl_lsp", {
  cmd = { "pddl-lsp" },
  filetypes = { "pddl" },
  root_markers = { ".git" },
})
vim.lsp.enable("pddl_lsp")
```

### Neovim 0.10

```lua
vim.filetype.add({ extension = { pddl = "pddl" } })

vim.api.nvim_create_autocmd("FileType", {
  pattern = "pddl",
  callback = function(args)
    vim.lsp.start({
      name = "pddl-lsp",
      cmd = { "pddl-lsp" },
      root_dir = vim.fs.root(args.buf, { ".git" }),
    })
  end,
})
```

### Vim with vim-lsp

With [vim-lsp](https://github.com/prabirshrestha/vim-lsp), add to your `.vimrc`:

```vim
autocmd BufRead,BufNewFile *.pddl setfiletype pddl

if executable('pddl-lsp')
  autocmd User lsp_setup call lsp#register_server({
        \ 'name': 'pddl-lsp',
        \ 'cmd': {server_info -> ['pddl-lsp']},
        \ 'allowlist': ['pddl'],
        \ })
endif
```

### Vim or Neovim with coc.nvim

With [coc.nvim](https://github.com/neoclide/coc.nvim), set the `pddl` filetype
as above, then add to `coc-settings.json` (open it with `:CocConfig`):

```json
{
  "languageserver": {
    "pddl": {
      "command": "pddl-lsp",
      "filetypes": ["pddl"]
    }
  }
}
```

### Helix

Add to `~/.config/helix/languages.toml`:

```toml
[language-server.pddl-lsp]
command = "pddl-lsp"

[[language]]
name = "pddl"
scope = "source.pddl"
file-types = ["pddl"]
comment-token = ";"
indent = { tab-width = 2, unit = "  " }
language-servers = ["pddl-lsp"]
```

Helix has no PDDL grammar, so there's no syntax highlighting. Everything else
works.

### Emacs with Eglot

This uses Eglot, which ships with Emacs 29 and later. Add to your init file:

```elisp
(define-derived-mode pddl-mode lisp-data-mode "PDDL"
  "Major mode for PDDL files.")
(add-to-list 'auto-mode-alist '("\\.pddl\\'" . pddl-mode))

(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs '(pddl-mode "pddl-lsp")))
(add-hook 'pddl-mode-hook #'eglot-ensure)
```

### Sublime Text

Install the [LSP](https://packagecontrol.io/packages/LSP) package and a PDDL
syntax package that provides the `source.pddl` scope. Then add to
`LSP.sublime-settings`:

```json
{
  "clients": {
    "pddl-lsp": {
      "enabled": true,
      "command": ["pddl-lsp"],
      "selector": "source.pddl"
    }
  }
}
```

## How files are found

At startup, the server reads every `.pddl` file under the workspace folders
(up to 10,000 files of at most 10 MB each). It skips hidden folders, `target`
and `node_modules`. Open files always use the editor's text.

If your editor reports file changes, as VS Code does, the server sees closed
files change on disk. Otherwise it reads a closed file again only when you
open or close it.

When several domains share a name, a problem uses the one in its own file.
Otherwise it uses the one whose file path sorts first.

## Limitations

- Argument types are not checked: `(at truck1 truck2)` passes even if the
  second parameter is a `location`.
- Requirements are not checked: using `forall` without
  `:universal-preconditions` is not reported.
- After a syntax error, checking resumes at the next section or action, so
  each section reports at most one syntax error.
- The formatter always uses two-space indents and an 80-column limit. It
  doesn't format a file whose parentheses are unbalanced.

## Releasing

`pddl-lsp` is versioned separately from `pddl`, with tags named
`pddl-lsp-vX.Y.Z` and its own [CHANGELOG](CHANGELOG.md).

1. If the server uses parser changes that aren't on crates.io yet, release
   `pddl` first (`cargo release <level> -p pddl --execute`). That also raises
   the `pddl` version `pddl-lsp` requires.
2. On `main`, run `cargo release <level> -p pddl-lsp --execute`. It turns the
   "Unreleased" section of the changelog into the release's section, commits,
   tags `pddl-lsp-vX.Y.Z` and pushes.
3. The tag starts [`release-pddl-lsp.yml`](../.github/workflows/release-pddl-lsp.yml).
   It runs the tests, publishes the crate to crates.io, creates a GitHub
   Release with the changelog section, and attaches binaries for Linux
   (x86-64 and ARM64), macOS (Apple silicon) and Windows (x86-64), each with
   a SHA-256 checksum.

The workflow publishes with crates.io's trusted publishing, which can only be
set up for a crate that already exists. Before the first release, publish
once by hand (`cargo publish -p pddl-lsp`). Then, in the crate's settings on
crates.io, add a trusted publisher for the repository `sunsided/pddl-rs`,
workflow `release-pddl-lsp.yml` and environment `release`.
