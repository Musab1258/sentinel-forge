import * as vscode from "vscode";

import { registerCommands } from "./commands";
import { registerHoverProvider } from "./diagnostics";

export function activate(context: vscode.ExtensionContext) {
  const diagnostics = vscode.languages.createDiagnosticCollection("sentinel-forge");
  context.subscriptions.push(diagnostics);

  registerCommands(context, diagnostics);
  registerHoverProvider(context);
}

export function deactivate() {
  return undefined;
}
