// Validates a plan with VAL (desktop only: it starts a process) and shows its
// findings as diagnostics on the plan's steps.

import { execFile } from "child_process";
import * as vscode from "vscode";
import { domainOf, saveAll } from "./planner";
import { problemFor } from "./plans";
import { parseVal } from "./val";

/** How long VAL may run. */
const TIMEOUT_MS = 60_000;

/** Runs `command` with `args` in `cwd`; resolves with its output whatever its exit code. */
function run(command: string[], args: string[], cwd: string): Promise<string> {
  return new Promise((resolve, reject) => {
    execFile(command[0], [...command.slice(1), ...args], { cwd, timeout: TIMEOUT_MS, maxBuffer: 64 * 1024 * 1024 }, (error, stdout, stderr) => {
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
  // Only an unsaved plan has no file; remote and virtual workspaces have other schemes.
  if (plan.isUntitled) {
    vscode.window.showErrorMessage("Save this plan to a file next to its problem, then validate it.");
    return undefined;
  }
  const problem = await problemFor(context, plan.uri, true);
  const domain = problem && (await domainOf(await vscode.workspace.openTextDocument(problem)));
  if (!problem || !domain) {
    vscode.window.showErrorMessage("Could not find the problem this plan solves, or its domain.");
    return undefined;
  }
  if (!(await saveAll([domain, problem, plan.uri]))) {
    return undefined;
  }
  const command = vscode.workspace.getConfiguration("pddl").get<string[]>("validator", ["Validate"]);
  let output: string;
  try {
    output = await run(command, ["-v", domain.fsPath, problem.fsPath, plan.uri.fsPath], vscode.Uri.joinPath(plan.uri, "..").fsPath);
  } catch (error: any) {
    if (error?.code === "ERR_CHILD_PROCESS_STDIO_MAXBUFFER") {
      vscode.window.showErrorMessage(`VAL (\`${command.join(" ")}\`) wrote more than 64 MiB of output; it was stopped.`);
      return undefined;
    }
    if (error?.killed) {
      vscode.window.showErrorMessage(`VAL (\`${command.join(" ")}\`) did not finish within ${TIMEOUT_MS / 1000} seconds; it was stopped.`);
      return undefined;
    }
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
