export type { AnalysisReport, Finding, Severity } from "@sentinel-forge/shared";

export interface AdvisoryRecord {
  id: string;
  title: string;
  severity: Severity;
  category: string;
  detectors: string[];
  fixture: string;
  advisory: string;
}

export interface PluginManifest {
  id: string;
  name: string;
  kind: "detector" | "reporter";
  version: string;
  description: string;
  entrypoint: string;
  capabilities: string[];
  maturity: string;
}

export interface PluginRegistryEntry {
  id: string;
  kind: PluginManifest["kind"];
  manifest: string;
}

export function advisoriesBySeverity<T extends { severity: Severity }>(records: T[]) {
  return records.reduce<Record<Severity, T[]>>(
    (groups, record) => {
      groups[record.severity].push(record);
      return groups;
    },
    {
      critical: [],
      high: [],
      medium: [],
      low: [],
      informational: [],
    },
  );
}

export function pluginsByKind<T extends PluginRegistryEntry>(
  entries: T[],
  kind: PluginRegistryEntry["kind"],
) {
  return entries.filter((entry) => entry.kind === kind);
}
