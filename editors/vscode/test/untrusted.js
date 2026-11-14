// Checks Restricted Mode, where VS Code disables the WASI runtime: the
// extension still provides highlighting, and no server reports diagnostics.

const assert = require("assert");
const vscode = require("vscode");

exports.run = async function () {
  assert.strictEqual(vscode.workspace.isTrusted, false, "the folder should open in Restricted Mode");
  const root = vscode.workspace.workspaceFolders[0].uri;
  const file = vscode.Uri.joinPath(root, "combined.pddl");
  const document = await vscode.workspace.openTextDocument(file);
  await vscode.window.showTextDocument(document);
  assert.strictEqual(document.languageId, "pddl");
  const extension = vscode.extensions.getExtension("sunsided.pddl-lsp");
  assert.ok(extension?.isActive, "the extension should be active for highlighting");
  // Give a server, had one started, time to report the unknown predicate.
  await new Promise((resolve) => setTimeout(resolve, 5000));
  assert.deepStrictEqual(vscode.languages.getDiagnostics(file), []);
};
