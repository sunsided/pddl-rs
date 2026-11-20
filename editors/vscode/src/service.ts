// Solves the open problem on a Planning-as-a-Service server
// (https://github.com/AI-Planning/planning-as-a-service). Off until the user
// sets `pddl.planningAsAService.url`, and it asks before it sends any file.

import * as vscode from "vscode";
import { activeTask } from "./planner";

/** The global state key of the servers the user allowed for good. */
const ALLOWED = "pddl.planningAsAService.allowed";

/** How long to wait for a plan. */
const TIMEOUT_MS = 5 * 60_000;

/** A finished Planning-as-a-Service task, as `/check/<id>` returns it. */
interface Finished {
  status: string;
  result?: { stdout?: string; stderr?: string; output?: Record<string, string> };
}

/** Asks whether to send the files to `url`; remembers "always" per server. */
async function consent(context: vscode.ExtensionContext, url: string, files: string[]): Promise<boolean> {
  const allowed = context.globalState.get<string[]>(ALLOWED, []);
  if (allowed.includes(url)) {
    return true;
  }
  const send = "Send";
  const always = "Always Send to This Server";
  const choice = await vscode.window.showWarningMessage(
    `Send ${files.join(" and ")} to ${url} to solve it there?`,
    {
      modal: true,
      detail: "Planning-as-a-Service runs the planner on that server, which receives the full text of both files.",
    },
    send,
    always,
  );
  if (choice === always) {
    await context.globalState.update(ALLOWED, [...allowed, url]);
  }
  return choice === send || choice === always;
}

/** POSTs `body` as JSON to `url` and returns the parsed answer. */
async function post(url: URL, body: unknown, signal: AbortSignal): Promise<any> {
  const response = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
    signal,
  });
  if (!response.ok) {
    throw new Error(`${url} answered ${response.status} ${response.statusText}`);
  }
  return response.json();
}

/**
 * Sends the open problem and its domain to the server of
 * `pddl.planningAsAService.url`, runs the planner of `pddl.planner` there and
 * opens the plan it returns. Returns the plan, or `undefined`.
 */
export async function solveWithService(context: vscode.ExtensionContext): Promise<string | undefined> {
  const config = vscode.workspace.getConfiguration("pddl");
  const url = config.get<string>("planningAsAService.url", "").trim().replace(/\/+$/, "");
  if (url === "") {
    const open = "Open Settings";
    const choice = await vscode.window.showInformationMessage(
      "Planning-as-a-Service is off. To use it, set pddl.planningAsAService.url to a server, " +
        "such as one you run with `planutils install server-environment`. The domain and problem are sent there.",
      open,
    );
    if (choice === open) {
      await vscode.commands.executeCommand("workbench.action.openSettings", "pddl.planningAsAService.url");
    }
    return undefined;
  }
  const task = await activeTask();
  if (!task) {
    return undefined;
  }
  const files = [task.domain, task.problem].map((uri) => vscode.workspace.asRelativePath(uri));
  if (!(await consent(context, url, files))) {
    return undefined;
  }
  const planner = config.get<string>("planner", "lama-first").trim();
  const read = async (uri: vscode.Uri) => (await vscode.workspace.openTextDocument(uri)).getText();
  const body = { domain: await read(task.domain), problem: await read(task.problem) };

  const finished = await vscode.window.withProgress(
    { location: vscode.ProgressLocation.Notification, title: `Solving with ${planner} on ${url}`, cancellable: true },
    async (_progress, token): Promise<Finished | undefined> => {
      const abort = new AbortController();
      token.onCancellationRequested(() => abort.abort());
      const deadline = Date.now() + TIMEOUT_MS;
      // A hung request ends at the deadline too, not only the polls between requests.
      const signal = () => AbortSignal.any([abort.signal, AbortSignal.timeout(Math.max(0, deadline - Date.now()))]);
      const base = new URL(`${url}/`);
      const submitted = await post(new URL(`package/${encodeURIComponent(planner)}/solve`, base), body, signal());
      if (typeof submitted?.result !== "string") {
        throw new Error(submitted?.Error ?? `unexpected answer: ${JSON.stringify(submitted)}`);
      }
      const check = new URL(submitted.result, base);
      while (Date.now() < deadline && !token.isCancellationRequested) {
        const answer: Finished = await post(check, {}, signal());
        if (answer.status !== "PENDING") {
          return answer;
        }
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }
      if (!token.isCancellationRequested) {
        throw new Error(`no plan after ${TIMEOUT_MS / 60_000} minutes`);
      }
      return undefined;
    },
  ).then(
    (answer) => answer,
    (error) => {
      if (error?.name === "AbortError") {
        // The user cancelled.
        return undefined;
      }
      if (error?.name === "TimeoutError") {
        error = new Error(`no plan after ${TIMEOUT_MS / 60_000} minutes`);
      }
      vscode.window.showErrorMessage(`Planning-as-a-Service failed: ${error.message ?? error}`);
      return undefined;
    },
  );
  if (!finished) {
    return undefined;
  }
  const output = finished.result?.output ?? {};
  const name = "sas_plan" in output ? "sas_plan" : Object.keys(output).find((key) => key.includes("plan"));
  if (finished.status !== "ok" || name === undefined) {
    const log = [finished.result?.stdout, finished.result?.stderr].filter(Boolean).join("\n").trim();
    vscode.window.showErrorMessage(`${planner} found no plan.${log ? ` Its output:\n${log.slice(-2000)}` : ""}`);
    return undefined;
  }
  const plan = output[name];
  const document = await vscode.workspace.openTextDocument({ language: "pddl-plan", content: plan });
  await vscode.window.showTextDocument(document);
  return plan;
}
