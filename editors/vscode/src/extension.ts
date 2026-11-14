// Starts the pddl-lsp language server for PDDL files.

import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  context.subscriptions.push(
    vscode.commands.registerCommand("pddl.restartServer", async () => {
      await stop();
      await start();
    }),
    vscode.workspace.onDidChangeConfiguration(async (event) => {
      if (event.affectsConfiguration("pddl.server.path")) {
        await stop();
        await start();
      }
    }),
  );
  await start();
}

export async function deactivate(): Promise<void> {
  await stop();
}

async function start(): Promise<void> {
  const command = vscode.workspace
    .getConfiguration("pddl")
    .get<string>("server.path", "pddl-lsp");
  const serverOptions: ServerOptions = { command, args: [] };
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "pddl" }],
    synchronize: {
      // Lets the server see domains change on disk while they are closed.
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.pddl"),
    },
  };
  client = new LanguageClient("pddl", "PDDL Language Server", serverOptions, clientOptions);
  try {
    await client.start();
  } catch (error) {
    client = undefined;
    const choice = await vscode.window.showErrorMessage(
      `Could not start the PDDL language server \`${command}\`: ${error}. ` +
        "Install it with `cargo install pddl-lsp`, or set `pddl.server.path`.",
      "Open Settings",
    );
    if (choice === "Open Settings") {
      await vscode.commands.executeCommand("workbench.action.openSettings", "pddl.server.path");
    }
  }
}

async function stop(): Promise<void> {
  const running = client;
  client = undefined;
  await running?.stop();
}
