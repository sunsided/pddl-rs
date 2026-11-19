// Runs planners on the open problem: planutils on the desktop. Shared helpers
// find the domain a problem names, as the language server resolves it.

import * as vscode from "vscode";

/** The problem in the active editor and the domain its `(:domain ...)` names. */
export interface Task {
  problem: vscode.Uri;
  domain: vscode.Uri;
}

/**
 * Finds the domain that `document`'s `(:domain name)` names, by asking the
 * language server for the definition of `name`. `undefined` if the document
 * isn't a problem or the server knows no such domain.
 */
export async function domainOf(document: vscode.TextDocument): Promise<vscode.Uri | undefined> {
  const match = /\(\s*:domain\s+([^\s()]+)/i.exec(document.getText());
  if (!match) {
    return undefined;
  }
  const position = document.positionAt(match.index + match[0].length - match[1].length);
  const found = await vscode.commands.executeCommand<(vscode.Location | vscode.LocationLink)[]>(
    "vscode.executeDefinitionProvider",
    document.uri,
    position,
  );
  const first = found?.[0];
  return first && ("targetUri" in first ? first.targetUri : first.uri);
}

/** The problem in the active editor and its domain, or `undefined` after telling the user why not. */
export async function activeTask(): Promise<Task | undefined> {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "pddl") {
    vscode.window.showErrorMessage("Open a PDDL problem to solve it.");
    return undefined;
  }
  const domain = await domainOf(editor.document);
  if (!domain) {
    vscode.window.showErrorMessage(
      "Could not find the domain that this problem's (:domain ...) names. Is it a problem, and is its domain in the workspace?",
    );
    return undefined;
  }
  return { problem: editor.document.uri, domain };
}

/** The directory of a file URI, as a path. */
function directory(uri: vscode.Uri): string {
  return vscode.Uri.joinPath(uri, "..").fsPath;
}

/**
 * Solves the active problem with the planutils planner of `pddl.planner`, in a
 * terminal. Runs `planutils run <planner> <domain> <problem>` directly, without
 * a shell, in the problem's folder, so planners that write `sas_plan` put it
 * next to the problem.
 */
export async function solveWithPlanutils(): Promise<vscode.Terminal | undefined> {
  if (!vscode.workspace.isTrusted) {
    vscode.window.showErrorMessage("Trust this folder to run a planner on it.");
    return undefined;
  }
  const task = await activeTask();
  if (!task) {
    return undefined;
  }
  await vscode.workspace.save(task.problem);
  const planner = vscode.workspace.getConfiguration("pddl").get<string>("planner", "lama-first").trim();
  const terminal = vscode.window.createTerminal({
    name: `PDDL: ${planner}`,
    shellPath: "planutils",
    shellArgs: ["run", planner, task.domain.fsPath, task.problem.fsPath],
    cwd: directory(task.problem),
    message:
      `Running planutils run ${planner} on ${vscode.workspace.asRelativePath(task.problem)}. ` +
      `If planutils is missing: pip install planutils. If the planner is: planutils install ${planner}.`,
  });
  terminal.show();
  return terminal;
}
