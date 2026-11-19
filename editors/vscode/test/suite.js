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
  const terminal = await vscode.commands.executeCommand("pddl.solveWithPlanutils");
  assert.ok(terminal, "no terminal");
  const options = terminal.creationOptions;
  assert.strictEqual(options.shellPath, "planutils");
  assert.deepStrictEqual(options.shellArgs, [
    "run",
    "lama-first",
    vscode.Uri.joinPath(root, "domain.pddl").fsPath,
    problem.fsPath,
  ]);
  assert.strictEqual(options.cwd, vscode.Uri.joinPath(root, "problems").fsPath);
  terminal.dispose();
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
    assert.ok(found[0].message.startsWith("VAL: unsatisfied precondition in (drive t1 home shop)"), found[0].message);
    assert.ok(found[0].message.includes("(Set (at t1 home) to true)"), found[0].message);
  } finally {
    await config.update("validator", undefined, vscode.ConfigurationTarget.Global);
  }
})(exports.run);
