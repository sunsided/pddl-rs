// Validates a plan with pddl-validate, which the language server runs: on the
// desktop and on the web, for saved and unsaved plans. Shows the verdict as a
// diagnostic on the plan.

import * as vscode from "vscode";
import { languageClient } from "./client";
import { domainOf } from "./planner";
import { problemFor } from "./plans";

/** The server's answer to `pddl/validatePlan`. */
export interface PlanVerdict {
  /** `error`: the domain or problem cannot be read, or they don't fit together. */
  status: "valid" | "invalid" | "unsupported" | "error";
  /** What the validator found, in one line; empty for a valid plan. */
  message: string;
  /** The 0-based plan line the message belongs to; `null` for the goal or the whole task. */
  line: number | null;
  /** A VAL-style repair hint for an invalid plan. */
  advice: string | null;
  /** The metric's value after a valid plan, if the problem has a defined one. */
  metric: number | null;
  /** Whether the metric is minimized; `null` without a metric. */
  minimize: boolean | null;
}

/** Validates the plan in the active editor and returns the verdict, or `undefined` if it couldn't run. */
export async function checkPlan(
  context: vscode.ExtensionContext,
  diagnostics: vscode.DiagnosticCollection,
): Promise<PlanVerdict | undefined> {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "pddl-plan") {
    vscode.window.showErrorMessage("Open a plan (a .plan or sas_plan file) to validate it.");
    return undefined;
  }
  const client = languageClient();
  if (!client) {
    vscode.window.showErrorMessage("The PDDL language server is not running, so the plan can't be validated.");
    return undefined;
  }
  const plan = editor.document;
  const problem = await problemFor(context, plan.uri, true);
  const domain = problem && (await domainOf(await vscode.workspace.openTextDocument(problem)));
  if (!problem || !domain) {
    vscode.window.showErrorMessage("Could not find the problem this plan solves, or its domain.");
    return undefined;
  }
  // Open documents give their current text, unsaved edits included.
  const text = async (uri: vscode.Uri) => (await vscode.workspace.openTextDocument(uri)).getText();
  const verdict = await client.sendRequest<PlanVerdict>("pddl/validatePlan", {
    domain: await text(domain),
    problem: await text(problem),
    plan: plan.getText(),
  });
  diagnostics.set(plan.uri, toDiagnostics(verdict, plan));
  if (verdict.status === "valid") {
    const metric = verdict.metric === null ? "" : ` (metric: ${verdict.metric})`;
    vscode.window.showInformationMessage(`The plan solves ${vscode.workspace.asRelativePath(problem)}${metric}.`);
  }
  return verdict;
}

/** The diagnostics that show `verdict` on `plan`: none for a valid plan. */
export function toDiagnostics(verdict: PlanVerdict, plan: vscode.TextDocument): vscode.Diagnostic[] {
  if (verdict.status === "valid") {
    return [];
  }
  // A failed goal belongs after the last step; the task as a whole at the top.
  const fallback = verdict.status === "invalid" ? lastStep(plan) : 0;
  const line = plan.lineAt(Math.min(verdict.line ?? fallback, plan.lineCount - 1));
  const message = verdict.advice ? `${verdict.message}\n${verdict.advice}` : verdict.message;
  const severity =
    verdict.status === "unsupported" ? vscode.DiagnosticSeverity.Warning : vscode.DiagnosticSeverity.Error;
  const diagnostic = new vscode.Diagnostic(line.range, message, severity);
  diagnostic.source = "pddl-validate";
  return [diagnostic];
}

/** The last line of `plan` with a step, or its last line. */
function lastStep(plan: vscode.TextDocument): number {
  for (let line = plan.lineCount - 1; line >= 0; line--) {
    if (plan.lineAt(line).text.trimStart().startsWith("(") || /^\s*[\d.]+\s*:/.test(plan.lineAt(line).text)) {
      return line;
    }
  }
  return plan.lineCount - 1;
}
