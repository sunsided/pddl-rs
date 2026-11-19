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
 * Solves the active problem with the planutils planner of `pddl.planner`, as a
 * task. Runs `planutils run <planner> <domain> <problem>` directly, without a
 * shell, in the problem's folder, so planners that write `sas_plan` put it next
 * to the problem. The task's terminal keeps the planner's output after it ends.
 */
export async function solveWithPlanutils(): Promise<vscode.TaskExecution | undefined> {
  if (!vscode.workspace.isTrusted) {
    vscode.window.showErrorMessage("Trust this folder to run a planner on it.");
    return undefined;
  }
  const solve = await activeTask();
  if (!solve) {
    return undefined;
  }
  await vscode.workspace.save(solve.problem);
  const planner = vscode.workspace.getConfiguration("pddl").get<string>("planner", "lama-first").trim();
  const task = new vscode.Task(
    { type: "pddl-planutils", planner },
    vscode.TaskScope.Workspace,
    `${planner} on ${vscode.workspace.asRelativePath(solve.problem)}`,
    "PDDL",
    new vscode.ProcessExecution("planutils", ["run", planner, solve.domain.fsPath, solve.problem.fsPath], {
      cwd: directory(solve.problem),
    }),
  );
  const execution = await vscode.tasks.executeTask(task);
  const ended = vscode.tasks.onDidEndTaskProcess((event) => {
    if (event.execution !== execution) {
      return;
    }
    ended.dispose();
    // planutils exits 1 when the planner isn't installed; a missing planutils can't start.
    if (event.exitCode !== 0) {
      vscode.window.showErrorMessage(
        `planutils run ${planner} failed. If planutils is missing: pip install planutils. ` +
          `If the planner is: planutils install ${planner}.`,
      );
    }
  });
  return execution;
}
