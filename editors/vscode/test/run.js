// Runs the extension in a downloaded VS Code against test/workspace.
// The suite in suite.js opens a problem and checks what the bundled server answers.

const path = require("path");
const cp = require("child_process");
const os = require("os");
const fs = require("fs");
const {
  downloadAndUnzipVSCode,
  resolveCliArgsFromVSCodeExecutablePath,
  runTests,
} = require("@vscode/test-electron");

async function main() {
  const extension = path.resolve(__dirname, "..");
  // A fresh profile, so the test neither sees nor changes the user's VS Code setup.
  const profile = fs.mkdtempSync(path.join(os.tmpdir(), "pddl-vscode-test-"));
  const dirs = [
    "--extensions-dir", path.join(profile, "extensions"),
    "--user-data-dir", path.join(profile, "user"),
  ];
  try {
    const vscode = await downloadAndUnzipVSCode({ cachePath: path.join(extension, ".vscode-test") });
    const [cli, ...args] = resolveCliArgsFromVSCodeExecutablePath(vscode);
    const install = cp.spawnSync(cli, [...args, ...dirs, "--install-extension", "ms-vscode.wasm-wasi-core"], {
      stdio: "inherit",
    });
    if (install.status !== 0) {
      throw new Error("could not install ms-vscode.wasm-wasi-core");
    }
    await runTests({
      vscodeExecutablePath: vscode,
      extensionDevelopmentPath: extension,
      extensionTestsPath: path.join(__dirname, "suite.js"),
      launchArgs: [path.join(__dirname, "workspace"), ...dirs, "--disable-workspace-trust", "--disable-gpu"],
    });
  } finally {
    fs.rmSync(profile, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
