import * as vscode from "vscode";

import { AnalyzerReport } from "./analyzer";

function diagnosticSeverity(severity: string) {
  switch (severity) {
    case "critical":
    case "high":
      return vscode.DiagnosticSeverity.Error;
    case "medium":
      return vscode.DiagnosticSeverity.Warning;
    case "low":
      return vscode.DiagnosticSeverity.Information;
    default:
      return vscode.DiagnosticSeverity.Hint;
  }
}

function diagnosticCode(severity: string) {
  return {
    critical: "critical",
    high: "high",
    medium: "medium",
    low: "low",
    informational: "info",
  }[severity] ?? "info";
}

export function publishDiagnostics(
  collection: vscode.DiagnosticCollection,
  document: vscode.TextDocument,
  report: AnalyzerReport,
) {
  const diagnostics = report.findings.map((finding) => {
    const line = Math.max(0, finding.line - 1);
    const lineText = document.lineAt(Math.min(line, document.lineCount - 1));
    const range = new vscode.Range(line, 0, line, lineText.text.length);
    const diagnostic = new vscode.Diagnostic(
      range,
      `${finding.title}: ${finding.description}`,
      diagnosticSeverity(finding.severity),
    );

    diagnostic.source = "Sentinel Forge";
    diagnostic.code = diagnosticCode(finding.severity);
    return diagnostic;
  });

  collection.set(document.uri, diagnostics);
}

export function registerHoverProvider(context: vscode.ExtensionContext) {
  const provider = vscode.languages.registerHoverProvider({ language: "rust" }, {
    provideHover(document, position) {
      const diagnostics =
        vscode.languages.getDiagnostics(document.uri).filter((item) =>
          item.range.contains(position),
        );

      if (diagnostics.length === 0) {
        return null;
      }

      const contents = diagnostics.map((diagnostic) =>
        new vscode.MarkdownString(
          `**Sentinel Forge**\n\n${diagnostic.message}\n\nSeverity: \`${diagnostic.code ?? "info"}\``,
        ),
      );

      return new vscode.Hover(contents);
    },
  });

  context.subscriptions.push(provider);
}
