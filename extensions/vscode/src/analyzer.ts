import { execFile } from "node:child_process";
import { promisify } from "node:util";

import * as vscode from "vscode";

const execFileAsync = promisify(execFile);

export interface AnalyzerFinding {
  id: string;
  title: string;
  severity: "critical" | "high" | "medium" | "low" | "informational";
  description: string;
  remediation: string;
  file: string;
  function: string;
  line: number;
  detector: string;
}

export interface AnalyzerReport {
  analyzer: string;
  target: string;
  findings: AnalyzerFinding[];
}

function buildAnalyzerInvocation(document: vscode.TextDocument) {
  const configuredCommand = vscode.workspace
    .getConfiguration("sentinelForge")
    .get<string>("analyzerCommand", "cargo run -p static-analyzer --bin sentinel-forge -- scan");

  const parts = configuredCommand.split(" ").filter(Boolean);

  if (parts.length === 0) {
    throw new Error("sentinelForge.analyzerCommand is empty.");
  }

  const [command, ...args] = parts;
  return {
    command,
    args: [...args, document.uri.fsPath, "--format", "json"],
  };
}

export async function analyzeDocument(document: vscode.TextDocument): Promise<AnalyzerReport> {
  const { command, args } = buildAnalyzerInvocation(document);
  const cwd = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  const { stdout, stderr } = await execFileAsync(command, args, { cwd });

  if (stderr.trim().length > 0) {
    vscode.window.showWarningMessage(stderr.trim());
  }

  return JSON.parse(stdout) as AnalyzerReport;
}
