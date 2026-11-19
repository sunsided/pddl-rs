// Bundles the extension into dist/extension.js for the desktop and
// dist/browser.js for VS Code for the Web, which can't load node_modules.
//   node esbuild.mjs               development build, with source maps
//   node esbuild.mjs --production  minified, for packaging
//   node esbuild.mjs --watch       rebuilds on every change
import * as esbuild from "esbuild";

const production = process.argv.includes("--production");
const shared = {
  bundle: true,
  format: "cjs",
  external: ["vscode"],
  minify: production,
  sourcemap: !production,
  sourcesContent: false,
  logLevel: "info",
};
const contexts = await Promise.all([
  esbuild.context({ ...shared, entryPoints: ["src/extension.ts"], platform: "node", target: "node18", outfile: "dist/extension.js" }),
  esbuild.context({ ...shared, entryPoints: ["src/browser.ts"], platform: "browser", target: "es2022", outfile: "dist/browser.js" }),
]);
if (process.argv.includes("--watch")) {
  await Promise.all(contexts.map((context) => context.watch()));
} else {
  for (const context of contexts) {
    await context.rebuild();
    await context.dispose();
  }
}
