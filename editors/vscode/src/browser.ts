// Web entry point, for vscode.dev and github.dev: only the bundled WebAssembly
// server can run there, so `pddl.server.path` is ignored.

import type * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/browser";
import { activateWith, deactivate } from "./client";

export { deactivate };

export function activate(context: vscode.ExtensionContext): Promise<void> {
  return activateWith(
    context,
    (_native, wasm, options) => new LanguageClient("pddl", "PDDL Language Server", wasm, options),
    { native: false },
  );
}
