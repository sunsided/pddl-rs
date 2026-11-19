// Starts the pddl-lsp language server for PDDL files: the WebAssembly build
// bundled with the extension, or a native binary from `pddl.server.path`.

import * as vscode from "vscode";
import { Wasm, type ProcessOptions } from "@vscode/wasm-wasi/v1";
import { createStdioOptions, createUriConverters, startServer } from "@vscode/wasm-wasi-lsp";
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
} from "vscode-languageclient/node";

// The bundled server, built by `task server`.
const WASM_SERVER = ["server", "pddl-lsp.wasm"];

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const output = vscode.window.createOutputChannel("PDDL Language Server", { log: true });
  context.subscriptions.push(
    output,
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
  const native = path !== "";
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
  const serverOptions: ServerOptions = native
    ? { command: path, args: [] }
    : () => startWasmServer(context, output);
  client = new LanguageClient("pddl", "PDDL Language Server", serverOptions, clientOptions);
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
  // The module imports a shared memory of at most 1 GiB (16384 pages of 64 KiB);
  // the server starts with 10 MiB.
  const memory = { initial: 160, maximum: 16384, shared: true };
  const process = await wasm.createProcess("pddl-lsp", module, memory, options);
  const decoder = new TextDecoder("utf-8");
  process.stderr?.onData((data) => output.append(decoder.decode(data)));
  return startServer(process);
}

async function stop(): Promise<void> {
  const running = client;
  client = undefined;
  await running?.stop();
}
