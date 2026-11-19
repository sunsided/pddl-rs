// Desktop entry point: runs the bundled WebAssembly server, or a native binary
// from `pddl.server.path`.

import { LanguageClient } from "vscode-languageclient/node";
import * as vscode from "vscode";
import { activateWith, deactivate } from "./client";
import { solveWithPlanutils } from "./planner";

export { deactivate };

export function activate(context: vscode.ExtensionContext): Promise<void> {
  // Only the desktop can start processes.
  context.subscriptions.push(vscode.commands.registerCommand("pddl.solveWithPlanutils", solveWithPlanutils));
  return activateWith(
    context,
    (native, wasm, options) =>
      new LanguageClient("pddl", "PDDL Language Server", native ? { command: native } : wasm, options),
    { native: true },
  );
}
