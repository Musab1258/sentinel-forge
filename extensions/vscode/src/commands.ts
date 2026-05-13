import * as vscode from "vscode";

import { analyzeDocument } from "./analyzer";
import { publishDiagnostics } from "./diagnostics";

export function registerCommands(
  context: vscode.ExtensionContext,
  diagnostics: vscode.DiagnosticCollection,
) {
  const command = vscode.commands.registerCommand(
    "sentinel-forge.scanCurrentFile",
    async () => {
      const editor = vscode.window.activeTextEditor;

      if (!editor) {
        vscode.window.showInformationMessage("Open a Rust contract file to run Sentinel Forge.");
        return;
      }

      try {
        const report = await analyzeDocument(editor.document);
        publishDiagnostics(diagnostics, editor.document, report);

        vscode.window.showInformationMessage(
          `Sentinel Forge completed with ${report.findings.length} finding(s).`,
        );
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        vscode.window.showErrorMessage(`Sentinel Forge scan failed: ${message}`);
      }
    },
  );

  context.subscriptions.push(command);
}
