// Validates a plan with VAL (desktop only: it starts a process) and shows its
// findings as diagnostics on the plan's steps.

import { execFile } from "child_process";
import * as vscode from "vscode";
import { domainOf } from "./planner";
import { problemFor } from "./plans";
import { parseVal } from "./val";

/** Runs `command` with `args` in `cwd`; resolves with its output whatever its exit code. */
function run(command: string[], args: string[], cwd: string): Promise<string> {
  return new Promise((resolve, reject) => {
    execFile(command[0], [...command.slice(1), ...args], { cwd, timeout: 60_000 }, (error, stdout, stderr) => {
      // VAL exits with the number of failed plans; only a failure to start is an error.
      if (error && typeof error.code !== "number") {
        reject(error);
      } else {
        resolve(stdout + stderr);
      }
    });
  });
}

/** Validates the plan in the active editor with the validator of `pddl.validator`. */
export async function validatePlan(
  context: vscode.ExtensionContext,
  diagnostics: vscode.DiagnosticCollection,
): Promise<boolean | undefined> {
  if (!vscode.workspace.isTrusted) {
    vscode.window.showErrorMessage("Trust this folder to run VAL on it.");
    return undefined;
  }
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "pddl-plan") {
    vscode.window.showErrorMessage("Open a plan (a .plan or sas_plan file) to validate it.");
    return undefined;
  }
  const plan = editor.document;
  const problem = await problemFor(context, plan.uri, true);
  const domain = problem && (await domainOf(await vscode.workspace.openTextDocument(problem)));
  if (!problem || !domain) {
    vscode.window.showErrorMessage("Could not find the problem this plan solves, or its domain.");
    return undefined;
  }
  await plan.save();
  const command = vscode.workspace.getConfiguration("pddl").get<string[]>("validator", ["Validate"]);
  let output: string;
  try {
    output = await run(command, ["-v", domain.fsPath, problem.fsPath, plan.uri.fsPath], vscode.Uri.joinPath(plan.uri, "..").fsPath);
  } catch (error) {
    vscode.window.showErrorMessage(
      `Could not run VAL (\`${command.join(" ")}\`): ${error}. Install it with \`planutils install val\` and set ` +
        `pddl.validator to ["val", "Validate"], or build it from https://github.com/KCL-Planning/VAL.`,
    );
    return undefined;
  }
  const result = parseVal(output, plan.getText());
  diagnostics.set(
    plan.uri,
    result.problems.map((p) => {
      const line = plan.lineAt(Math.min(p.line, plan.lineCount - 1));
      const diagnostic = new vscode.Diagnostic(line.range, p.message, vscode.DiagnosticSeverity.Error);
      diagnostic.source = "VAL";
      return diagnostic;
    }),
  );
  if (result.valid) {
    vscode.window.showInformationMessage(`VAL: the plan solves ${vscode.workspace.asRelativePath(problem)}.`);
  }
  return result.valid;
}
