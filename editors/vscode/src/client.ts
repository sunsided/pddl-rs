// Starts the pddl-lsp language server for PDDL files: the WebAssembly build
// bundled with the extension, or a native binary from `pddl.server.path`.
// Shared by the desktop entry point (extension.ts) and the web one (browser.ts).

import * as vscode from "vscode";
import { checkPlan } from "./check";
import { planDefinitions } from "./plans";
import { solveWithService } from "./service";
import { Wasm, type ProcessOptions } from "@vscode/wasm-wasi/v1";
import { createStdioOptions, createUriConverters, startServer } from "@vscode/wasm-wasi-lsp";
import type {
  BaseLanguageClient,
  LanguageClientOptions,
  MessageTransports,
} from "vscode-languageclient";

// The bundled server, built by `task server`.
const WASM_SERVER = ["server", "pddl-lsp.wasm"];

/** Creates the language client for a native server's path, or for the bundled one. */
export type CreateClient = (
  native: string | undefined,
  wasm: () => Promise<MessageTransports>,
  options: LanguageClientOptions,
) => BaseLanguageClient;

let client: BaseLanguageClient | undefined;

/** The running language client, or `undefined` before it starts or in Restricted Mode. */
export function languageClient(): BaseLanguageClient | undefined {
  return client;
}
let createClient: CreateClient;
let nativeAllowed: boolean;

/** Activates the extension. Without `native`, `pddl.server.path` is ignored. */
export async function activateWith(
  context: vscode.ExtensionContext,
  create: CreateClient,
  { native }: { native: boolean },
): Promise<void> {
  createClient = create;
  nativeAllowed = native;
  const output = vscode.window.createOutputChannel("PDDL Language Server", { log: true });
  const verdicts = vscode.languages.createDiagnosticCollection("pddl-validate");
  context.subscriptions.push(
    output,
    verdicts,
    vscode.commands.registerCommand("pddl.checkPlan", () => checkPlan(context, verdicts)),
    vscode.commands.registerCommand("pddl.restartServer", async () => {
      await stop();
      await start(context, output);
    }),
    vscode.workspace.onDidChangeConfiguration(async (event) => {
      if (event.affectsConfiguration("pddl.server.path")) {
        await stop();
        await start(context, output);
      }
    }),
    vscode.workspace.onDidGrantWorkspaceTrust(() => start(context, output)),
    vscode.languages.registerDefinitionProvider({ language: "pddl-plan" }, planDefinitions(context)),
    vscode.commands.registerCommand("pddl.solveWithService", () => solveWithService(context)),
  );
  await start(context, output);
}

export async function deactivate(): Promise<void> {
  await stop();
}

async function start(
  context: vscode.ExtensionContext,
  output: vscode.LogOutputChannel,
): Promise<void> {
  // In Restricted Mode, VS Code disables the WASI runtime the bundled server
  // needs, and a native server is a program the folder may have chosen. Only
  // highlighting works until the folder is trusted.
  if (!vscode.workspace.isTrusted) {
    output.info("Restricted Mode: the language server starts once you trust this folder.");
    return;
  }
  const path = vscode.workspace.getConfiguration("pddl").get<string>("server.path", "").trim();
  const native = nativeAllowed && path !== "";
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ language: "pddl" }],
    outputChannel: output,
    synchronize: {
      // Lets the server see domains change on disk while they are closed.
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.pddl"),
    },
    // The WebAssembly server sees the workspace folders mounted under `/workspace`.
    uriConverters: native ? undefined : createUriConverters(),
  };
  // The WASI runtime blocks on the server's stdin with SharedArrayBuffer and
  // Atomics.wait, which a web page only has when it is cross-origin isolated.
  // Without it the server would never answer, so say so instead of hanging.
  if (!native && (globalThis as { crossOriginIsolated?: boolean }).crossOriginIsolated === false) {
    output.error("This page is not cross-origin isolated, so the bundled server can't run.");
    vscode.window.showErrorMessage(
      "The PDDL language server needs a cross-origin isolated page. On vscode.dev, add ?vscode-coi=on to the URL.",
    );
    return;
  }
  const wasm = () => startWasmServer(context, output);
  client = createClient(native ? path : undefined, wasm, clientOptions);
  try {
    await client.start();
  } catch (error) {
    client = undefined;
    const server = native ? `\`${path}\`` : "bundled with the extension";
    const choice = await vscode.window.showErrorMessage(
      `Could not start the PDDL language server ${server}: ${error}`,
      "Open Settings",
    );
    if (choice === "Open Settings") {
      await vscode.commands.executeCommand("workbench.action.openSettings", "pddl.server.path");
    }
  }
}

// Runs the bundled WebAssembly server in the WASI runtime of `ms-vscode.wasm-wasi-core`.
async function startWasmServer(context: vscode.ExtensionContext, output: vscode.LogOutputChannel) {
  const wasm = await Wasm.load();
  const file = vscode.Uri.joinPath(context.extensionUri, ...WASM_SERVER);
  const module = await WebAssembly.compile(await vscode.workspace.fs.readFile(file));
  const options: ProcessOptions = {
    stdio: createStdioOptions(),
    mountPoints: [{ kind: "workspaceFolder" }],
  };
  // The server is built for `wasm32-wasip1`, without threads. The WASI runtime
  // still uses SharedArrayBuffer for its blocking calls, see `start`.
  const process = await wasm.createProcess("pddl-lsp", module, options);
  const decoder = new TextDecoder("utf-8");
  process.stderr?.onData((data) => output.append(decoder.decode(data)));
  return startServer(process);
}

async function stop(): Promise<void> {
  const running = client;
  client = undefined;
  await running?.stop();
}
