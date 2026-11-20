// Plan files (`*.plan`, `sas_plan`): which problem a plan solves, and
// go-to-definition from a step to its action in the domain.

import * as vscode from "vscode";
import { domainOf } from "./planner";
import { normalizeStep } from "./val";

/** The workspace state key of the problem chosen for each plan. */
const CHOSEN = "pddl.planProblems";

/** Whether `text` is a PDDL problem. */
const isProblem = (text: string) => /\(\s*define\s*\(\s*problem\b/i.test(text);

/**
 * The problem that the plan `plan` solves: the one chosen for it before, the
 * only problem in its folder, or, if `ask`, one the user picks.
 */
export async function problemFor(
  context: vscode.ExtensionContext,
  plan: vscode.Uri,
  ask: boolean,
): Promise<vscode.Uri | undefined> {
  const chosen = context.workspaceState.get<Record<string, string>>(CHOSEN, {});
  if (chosen[plan.toString()]) {
    return vscode.Uri.parse(chosen[plan.toString()]);
  }
  const folder = vscode.Uri.joinPath(plan, "..");
  const nearby: vscode.Uri[] = [];
  for (const [name, type] of await vscode.workspace.fs.readDirectory(folder)) {
    const uri = vscode.Uri.joinPath(folder, name);
    if (type === vscode.FileType.File && name.toLowerCase().endsWith(".pddl") && isProblem(await read(uri))) {
      nearby.push(uri);
    }
  }
  if (nearby.length === 1) {
    return nearby[0];
  }
  if (!ask) {
    return undefined;
  }
  const problems: vscode.Uri[] = [];
  for (const uri of await vscode.workspace.findFiles("**/*.pddl")) {
    if (isProblem(await read(uri))) {
      problems.push(uri);
    }
  }
  const pick = await vscode.window.showQuickPick(
    problems.map((uri) => ({ label: vscode.workspace.asRelativePath(uri), uri })),
    { title: `Which problem does ${vscode.workspace.asRelativePath(plan)} solve?` },
  );
  if (pick) {
    await context.workspaceState.update(CHOSEN, { ...chosen, [plan.toString()]: pick.uri.toString() });
  }
  return pick?.uri;
}

/** The text of a file, open or not. */
async function read(uri: vscode.Uri): Promise<string> {
  const open = vscode.workspace.textDocuments.find((d) => d.uri.toString() === uri.toString());
  return open ? open.getText() : new TextDecoder().decode(await vscode.workspace.fs.readFile(uri));
}

/** Go-to-definition from a plan step's action name to the action in the domain. */
export function planDefinitions(context: vscode.ExtensionContext): vscode.DefinitionProvider {
  return {
    async provideDefinition(document, position) {
      const step = normalizeStep(document.lineAt(position.line).text);
      const name = step?.slice(1, -1).split(" ")[0];
      const word = document.getWordRangeAtPosition(position, /[^\s()]+/);
      if (!name || !word || document.getText(word).toLowerCase() !== name) {
        return undefined;
      }
      const problem = await problemFor(context, document.uri, false);
      const domain = problem && (await domainOf(await vscode.workspace.openTextDocument(problem)));
      if (!domain) {
        return undefined;
      }
      const text = await read(domain);
      const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      const match = new RegExp(`\\(\\s*:(?:durative-)?action\\s+(${escaped})(?=[\\s()])`, "i").exec(text);
      if (!match) {
        return undefined;
      }
      const start = match.index + match[0].length - match[1].length;
      const lines = text.slice(0, start).split("\n");
      const at = new vscode.Position(lines.length - 1, lines[lines.length - 1].length);
      return new vscode.Location(domain, new vscode.Range(at, at.translate(0, match[1].length)));
    },
  };
}
