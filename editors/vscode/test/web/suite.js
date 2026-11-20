// Runs in VS Code for the Web: checks that the bundled server, started by the
// web entry point (dist/browser.js), publishes diagnostics for a problem.
// Bundled for the browser by test/web.js, so it uses no Node modules.

const vscode = require("vscode");

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

exports.run = async function () {
  const root = vscode.workspace.workspaceFolders[0].uri;
  const problem = vscode.Uri.joinPath(root, "problems", "deliver.pddl");
  const document = await vscode.workspace.openTextDocument(problem);
  await vscode.window.showTextDocument(document);
  if (document.languageId !== "pddl") {
    throw new Error(`language is ${document.languageId}, not pddl`);
  }
  // The server found the domain in another folder, so it checks the problem's names.
  for (let i = 0; i < 120; i++) {
    const found = vscode.languages.getDiagnostics(problem).map((d) => d.message);
    if (found.length > 0) {
      const expected = "unknown object or constant `ham`";
      if (found.length !== 1 || found[0] !== expected) {
        throw new Error(`expected [${expected}], got ${JSON.stringify(found)}`);
      }
      return checkPlan(root);
    }
    await sleep(500);
  }
  const state = ["sunsided.pddl-lsp", "ms-vscode.wasm-wasi-core"].map((id) => {
    const extension = vscode.extensions.getExtension(id);
    return `${id}: ${extension ? (extension.isActive ? "active" : "inactive") : "missing"}`;
  });
  throw new Error(`the server published no diagnostics within 60 seconds (${state.join(", ")})`);
};

/** Validates trip.plan with the built-in validator, which runs in the browser too. */
async function checkPlan(root) {
  const plan = vscode.Uri.joinPath(root, "trip", "trip.plan");
  await vscode.window.showTextDocument(await vscode.workspace.openTextDocument(plan));
  const verdict = await vscode.commands.executeCommand("pddl.checkPlan");
  if (verdict?.status !== "invalid" || verdict.line !== 2) {
    throw new Error(`expected an invalid plan at line 2, got ${JSON.stringify(verdict)}`);
  }
}
