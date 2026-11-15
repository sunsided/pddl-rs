// Checks the bundled language server from inside VS Code.

const assert = require("assert");
const vscode = require("vscode");

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
