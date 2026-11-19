// Desktop entry point: runs the bundled WebAssembly server, or a native binary
// from `pddl.server.path`.

import { LanguageClient } from "vscode-languageclient/node";
import * as vscode from "vscode";
import { activateWith, deactivate } from "./client";
import { solveWithPlanutils } from "./planner";
import { validatePlan } from "./validate";

export { deactivate };

export function activate(context: vscode.ExtensionContext): Promise<void> {
  // Only the desktop can start processes.
  const val = vscode.languages.createDiagnosticCollection("VAL");
  context.subscriptions.push(
    val,
    vscode.commands.registerCommand("pddl.solveWithPlanutils", solveWithPlanutils),
    vscode.commands.registerCommand("pddl.validatePlan", () => validatePlan(context, val)),
  );
  return activateWith(
    context,
    (native, wasm, options) =>
      new LanguageClient("pddl", "PDDL Language Server", native ? { command: native } : wasm, options),
    { native: true },
  );
}
