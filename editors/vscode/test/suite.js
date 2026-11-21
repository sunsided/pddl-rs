// Checks the bundled language server from inside VS Code.

const assert = require("assert");
const vscode = require("vscode");
const path = require("path");

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

/** Waits up to 30 seconds for the server to publish diagnostics for `uri`. */
async function diagnostics(uri) {
  for (let i = 0; i < 60; i++) {
    const found = vscode.languages.getDiagnostics(uri);
    if (found.length > 0) {
      return found;
    }
    await sleep(500);
  }
  return [];
}

exports.run = async function () {
  const root = vscode.workspace.workspaceFolders[0].uri;
  const problem = vscode.Uri.joinPath(root, "problems", "deliver.pddl");
  const document = await vscode.workspace.openTextDocument(problem);
  await vscode.window.showTextDocument(document);
  assert.strictEqual(document.languageId, "pddl");

  // The server found the domain in another folder, so it checks the problem's names.
  const found = await diagnostics(problem);
  assert.deepStrictEqual(
    found.map((d) => [d.range.start.line, d.range.start.character, d.message]),
    [[4, 16, "unknown object or constant `ham`"]],
  );

  // `at` in `(:init (at t1 home))` leads to the domain's `:predicates`, with its real path.
  const at = new vscode.Position(3, 10);
  const definitions = await vscode.commands.executeCommand("vscode.executeDefinitionProvider", problem, at);
  assert.strictEqual(definitions.length, 1);
  const definition = definitions[0];
  const uri = definition.uri ?? definition.targetUri;
  const range = definition.range ?? definition.targetRange;
  assert.strictEqual(uri.toString(), vscode.Uri.joinPath(root, "domain.pddl").toString());
  assert.deepStrictEqual([range.start.line, range.start.character], [3, 16]);

  const hovers = await vscode.commands.executeCommand("vscode.executeHoverProvider", problem, at);
  const text = hovers.flatMap((h) => h.contents.map((c) => c.value ?? String(c))).join("\n");
  assert.ok(text.includes("(at ?t - truck ?p - place)"), text);
};

// `PDDL: Solve with planutils` runs the configured planner on the problem and
// the domain its `(:domain ...)` names, in the problem's folder.
exports.run = ((checks) => async function () {
  await checks();
  const root = vscode.workspace.workspaceFolders[0].uri;
  const problem = vscode.Uri.joinPath(root, "problems", "deliver.pddl");
  await vscode.window.showTextDocument(await vscode.workspace.openTextDocument(problem));
  const execution = await vscode.commands.executeCommand("pddl.solveWithPlanutils");
  assert.ok(execution, "no task");
  const process = execution.task.execution;
  assert.strictEqual(process.process, "planutils");
  assert.deepStrictEqual(process.args, [
    "run",
    "lama-first",
    vscode.Uri.joinPath(root, "domain.pddl").fsPath,
    problem.fsPath,
  ]);
  assert.strictEqual(process.options.cwd, vscode.Uri.joinPath(root, "problems").fsPath);
})(exports.run);

// Plans: a step leads to its action, and VAL's findings land on the steps.
exports.run = ((checks) => async function () {
  await checks();
  const root = vscode.workspace.workspaceFolders[0].uri;
  const plan = vscode.Uri.joinPath(root, "problems", "deliver.plan");
  const document = await vscode.workspace.openTextDocument(plan);
  await vscode.window.showTextDocument(document);
  assert.strictEqual(document.languageId, "pddl-plan");

  // `DRIVE` on line 3 leads to `(:action drive` in the domain the problem names.
  const definitions = await vscode.commands.executeCommand(
    "vscode.executeDefinitionProvider", plan, new vscode.Position(2, 2),
  );
  assert.strictEqual(definitions.length, 1, JSON.stringify(definitions));
  assert.strictEqual(definitions[0].uri.toString(), vscode.Uri.joinPath(root, "domain.pddl").toString());
  assert.deepStrictEqual([definitions[0].range.start.line, definitions[0].range.start.character], [4, 11]);

  const config = vscode.workspace.getConfiguration("pddl");
  await config.update("validator", ["node", path.join(__dirname, "fake-val.js")], vscode.ConfigurationTarget.Global);
  try {
    const valid = await vscode.commands.executeCommand("pddl.validatePlan");
    assert.strictEqual(valid, false);
    const found = vscode.languages.getDiagnostics(plan);
    assert.strictEqual(found.length, 1, JSON.stringify(found));
    assert.strictEqual(found[0].range.start.line, 2);
    assert.ok(found[0].message.startsWith("VAL: unsatisfied precondition in (drive t1 shop home)"), found[0].message);
    assert.ok(found[0].message.includes("(Set (at t1 shop) to true)"), found[0].message);
  } finally {
    await config.update("validator", undefined, vscode.ConfigurationTarget.Global);
  }
})(exports.run);

// The built-in validator: the server checks the plan with pddl-validate.
exports.run = ((checks) => async function () {
  await checks();
  const root = vscode.workspace.workspaceFolders[0].uri;
  const plan = vscode.Uri.joinPath(root, "trip", "trip.plan");
  await vscode.window.showTextDocument(await vscode.workspace.openTextDocument(plan));
  const verdict = await vscode.commands.executeCommand("pddl.checkPlan");
  assert.strictEqual(verdict.status, "invalid", JSON.stringify(verdict));
  const found = vscode.languages.getDiagnostics(plan).filter((d) => d.source === "pddl-validate");
  assert.strictEqual(found.length, 1, JSON.stringify(found));
  assert.strictEqual(found[0].range.start.line, 2);
  assert.ok(found[0].message.startsWith("step 2 (drive t1 home shop): unsatisfied precondition"), found[0].message);

  // An unsaved edit counts: with one step the plan is valid, and the diagnostic goes.
  const editor = vscode.window.activeTextEditor;
  await editor.edit((edit) => edit.delete(editor.document.lineAt(2).rangeIncludingLineBreak));
  const valid = await vscode.commands.executeCommand("pddl.checkPlan");
  assert.strictEqual(valid.status, "valid", JSON.stringify(valid));
  assert.strictEqual(vscode.languages.getDiagnostics(plan).filter((d) => d.source === "pddl-validate").length, 0);
  await vscode.commands.executeCommand("workbench.action.files.revert");
})(exports.run);

// Planning-as-a-Service: off without a URL, asks before sending, then submits
// the files and polls until the plan is there. Runs against a local stub.
exports.run = ((checks) => async function () {
  await checks();
  const http = require("http");
  const root = vscode.workspace.workspaceFolders[0].uri;
  const problem = vscode.Uri.joinPath(root, "problems", "deliver.pddl");
  await vscode.window.showTextDocument(await vscode.workspace.openTextDocument(problem));

  const requests = [];
  let polls = 0;
  const server = http.createServer((request, response) => {
    let body = "";
    request.on("data", (chunk) => (body += chunk));
    request.on("end", () => {
      requests.push({ method: request.method, url: request.url, body: JSON.parse(body || "{}") });
      response.setHeader("Content-Type", "application/json");
      if (request.url === "/package/lama-first/solve") {
        response.end(JSON.stringify({ result: "/check/42?external=True" }));
      } else if (request.url.startsWith("/check/42")) {
        polls += 1;
        response.end(JSON.stringify(polls === 1
          ? { status: "PENDING" }
          : { status: "ok", result: { stdout: "", stderr: "", output: { sas_plan: "(drive t1 home shop)\n; cost = 1 (unit cost)\n" } } }));
      } else {
        response.statusCode = 404;
        response.end("{}");
      }
    });
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const url = `http://127.0.0.1:${server.address().port}`;

  const window = vscode.window;
  const { showWarningMessage, showInformationMessage } = window;
  const prompts = [];
  let answer;
  window.showWarningMessage = async (message) => (prompts.push(message), answer);
  window.showInformationMessage = async (message) => (prompts.push(message), undefined);
  const config = vscode.workspace.getConfiguration("pddl");
  try {
    // Off by default: nothing is sent.
    assert.strictEqual(await vscode.commands.executeCommand("pddl.solveWithService"), undefined);
    assert.ok(prompts.pop().startsWith("Planning-as-a-Service is off."));

    await config.update("planningAsAService.url", url, vscode.ConfigurationTarget.Global);
    // Declined: nothing is sent.
    answer = undefined;
    assert.strictEqual(await vscode.commands.executeCommand("pddl.solveWithService"), undefined);
    assert.strictEqual(prompts.pop(), `Send domain.pddl and problems/deliver.pddl to ${url} to solve it there?`);
    assert.strictEqual(requests.length, 0);

    answer = "Send";
    const plan = await vscode.commands.executeCommand("pddl.solveWithService");
    assert.strictEqual(plan, "(drive t1 home shop)\n; cost = 1 (unit cost)\n");
    assert.strictEqual(vscode.window.activeTextEditor.document.languageId, "pddl-plan");
    assert.deepStrictEqual(requests.map((r) => [r.method, r.url]), [
      ["POST", "/package/lama-first/solve"],
      ["POST", "/check/42?external=True"],
      ["POST", "/check/42?external=True"],
    ]);
    const domainText = (await vscode.workspace.openTextDocument(vscode.Uri.joinPath(root, "domain.pddl"))).getText();
    assert.strictEqual(requests[0].body.domain, domainText);
    assert.ok(requests[0].body.problem.includes("(define (problem deliver)"));
  } finally {
    window.showWarningMessage = showWarningMessage;
    window.showInformationMessage = showInformationMessage;
    await config.update("planningAsAService.url", undefined, vscode.ConfigurationTarget.Global);
    server.close();
  }
})(exports.run);
