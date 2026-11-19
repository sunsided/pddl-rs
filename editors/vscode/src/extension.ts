// Desktop entry point: runs the bundled WebAssembly server, or a native binary
// from `pddl.server.path`.

import type * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { activateWith, deactivate } from "./client";

export { deactivate };

export function activate(context: vscode.ExtensionContext): Promise<void> {
  return activateWith(
    context,
    (native, wasm, options) =>
      new LanguageClient("pddl", "PDDL Language Server", native ? { command: native } : wasm, options),
    { native: true },
  );
}
