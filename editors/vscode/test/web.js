// Runs the extension's web entry point in VS Code for the Web, in a headless
// Chromium, against test/workspace. The suite in web/suite.js is bundled for
// the browser first, because the web extension host can't load node_modules.

const path = require("path");
const esbuild = require("esbuild");
const { runTests } = require("@vscode/test-web");

async function main() {
  const extension = path.resolve(__dirname, "..");
  const suite = path.join(extension, "dist", "test", "web-suite.js");
  await esbuild.build({
    entryPoints: [path.join(__dirname, "web", "suite.js")],
    bundle: true,
    format: "cjs",
    platform: "browser",
    external: ["vscode"],
    outfile: suite,
    logLevel: "warning",
  });
  await runTests({
    browserType: "chromium",
    headless: true,
    extensionDevelopmentPath: extension,
    extensionTestsPath: suite,
    extensionIds: [{ id: "ms-vscode.wasm-wasi-core" }],
    folderPath: path.join(__dirname, "workspace"),
    quality: "stable",
    // The WASI runtime blocks on the server's stdin with SharedArrayBuffer and
    // Atomics.wait, which need a cross-origin isolated page. Without it the
    // extension shows an error instead of starting the server.
    coi: true,
  });
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
