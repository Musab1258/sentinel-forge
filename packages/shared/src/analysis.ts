export type Severity = "critical" | "high" | "medium" | "low" | "informational";
export type Confidence = "high" | "medium" | "low";

export interface CodeSpan {
  start_line: number;
  end_line: number;
}

export interface Finding {
  id: string;
  title: string;
  severity: Severity;
  confidence: Confidence;
  description: string;
  remediation: string;
  file: string;
  function: string;
  line: number;
  detector: string;
  evidence: string[];
  trace: string[];
  code_span: CodeSpan;
}

export interface ScannedFile {
  file: string;
  functions: number;
  findings: number;
}

export interface AnalysisSummary {
  files: number;
  functions: number;
  findings: number;
  critical: number;
  high: number;
  medium: number;
  low: number;
  informational: number;
}

export interface AnalysisReport {
  analyzer: string;
  target: string;
  summary: AnalysisSummary;
  scanned_files: ScannedFile[];
  findings: Finding[];
}

export interface ScanRun {
  id: string;
  timestamp: string;
  commitHash: string;
  branch: string;
  status: "passed" | "failed";
  summary: AnalysisSummary;
}

export interface FlowNode {
  id: string;
  label: string;
  detail: string;
  severity?: Severity;
  x: number;
  y: number;
}

export interface FlowEdge {
  id: string;
  source: string;
  target: string;
  label: string;
}

export interface ReplayStep {
  id: string;
  tx: string;
  function: string;
  actor: string;
  effect: string;
  state: string;
  outcome: "setup" | "warning" | "exploit";
}

export interface StateTransition {
  id: string;
  label: string;
  before: string;
  after: string;
  note: string;
  severity: Severity;
}

export const severityLabels: Record<Severity, string> = {
  critical: "Critical",
  high: "High",
  medium: "Medium",
  low: "Low",
  informational: "Informational",
};

export const severityColors: Record<Severity, string> = {
  critical: "#ef4444",
  high: "#f97316",
  medium: "#eab308",
  low: "#22c55e",
  informational: "#64748b",
};

export const sampleReport: AnalysisReport = {
  analyzer: "Sentinel Forge static analyzer MVP",
  target: "examples/vulnerable-contracts",
  summary: {
    files: 3,
    functions: 3,
    findings: 7,
    critical: 2,
    high: 3,
    medium: 2,
    low: 0,
    informational: 0,
  },
  scanned_files: [
    {
      file: "examples/vulnerable-contracts/missing_authorization.rs",
      functions: 1,
      findings: 3,
    },
    {
      file: "examples/vulnerable-contracts/unbounded_loop.rs",
      functions: 1,
      findings: 1,
    },
    {
      file: "examples/vulnerable-contracts/unchecked_arithmetic.rs",
      functions: 1,
      findings: 3,
    },
  ],
  findings: [
    {
      id: "SF-001",
      title: "Missing authorization check",
      severity: "critical",
      confidence: "high",
      description:
        "Public function `transfer_admin` mutates storage without an authorization gate.",
      remediation:
        "Require authorization before any privileged or state-changing operation.",
      file: "examples/vulnerable-contracts/missing_authorization.rs",
      function: "transfer_admin",
      line: 4,
      detector: "missing-authorization",
      evidence: ["line 4: storage::set_admin", "missing require_auth() call"],
      trace: ["Function transfer_admin", "WriteStorage", "PrivilegedAction"],
      code_span: {
        start_line: 4,
        end_line: 4,
      },
    },
    {
      id: "SF-006",
      title: "Privilege escalation risk",
      severity: "critical",
      confidence: "high",
      description:
        "Function `transfer_admin` appears to manage privileged state without verifying the caller.",
      remediation:
        "Require the current admin, owner, or authorized role to approve privilege changes before state updates.",
      file: "examples/vulnerable-contracts/missing_authorization.rs",
      function: "transfer_admin",
      line: 4,
      detector: "privilege-escalation",
      evidence: [
        "line 4: storage::set_admin",
        "line 4: storage::set_admin",
        "privileged transition lacks authorization gate",
      ],
      trace: ["Function transfer_admin", "WriteStorage", "PrivilegedAction"],
      code_span: {
        start_line: 4,
        end_line: 4,
      },
    },
    {
      id: "SF-003",
      title: "Missing input validation",
      severity: "high",
      confidence: "medium",
      description:
        "Function `transfer_admin` accepts external parameters but performs risky work without a clear validation step.",
      remediation:
        "Validate externally supplied values before arithmetic, storage writes, or external calls.",
      file: "examples/vulnerable-contracts/missing_authorization.rs",
      function: "transfer_admin",
      line: 4,
      detector: "missing-validation",
      evidence: ["line 4: storage::set_admin", "no validation guard detected"],
      trace: ["Function transfer_admin", "WriteStorage", "PrivilegedAction"],
      code_span: {
        start_line: 4,
        end_line: 4,
      },
    },
    {
      id: "SF-002",
      title: "Unsafe storage access pattern",
      severity: "high",
      confidence: "medium",
      description:
        "Function `mint` writes contract state without an observable validation guard.",
      remediation:
        "Add explicit ownership, bounds, or invariant checks before updating storage.",
      file: "examples/vulnerable-contracts/unchecked_arithmetic.rs",
      function: "mint",
      line: 4,
      detector: "unsafe-storage-access",
      evidence: ["line 6: balances::set_total"],
      trace: ["Function mint", "RequireAuth", "Arithmetic", "WriteStorage"],
      code_span: {
        start_line: 4,
        end_line: 4,
      },
    },
    {
      id: "SF-003",
      title: "Missing input validation",
      severity: "high",
      confidence: "medium",
      description:
        "Function `mint` accepts external parameters but performs risky work without a clear validation step.",
      remediation:
        "Validate externally supplied values before arithmetic, storage writes, or external calls.",
      file: "examples/vulnerable-contracts/unchecked_arithmetic.rs",
      function: "mint",
      line: 4,
      detector: "missing-validation",
      evidence: [
        "line 6: balances::set_total",
        "line 5: amount + 1",
        "no validation guard detected",
      ],
      trace: ["Function mint", "RequireAuth", "Arithmetic", "WriteStorage"],
      code_span: {
        start_line: 4,
        end_line: 4,
      },
    },
    {
      id: "SF-005",
      title: "Potential denial-of-service pattern",
      severity: "medium",
      confidence: "medium",
      description:
        "Function `distribute` contains looping behavior that may scale with user-controlled input or expensive state work.",
      remediation:
        "Bound iteration, chunk large workloads, or move repeated work behind safer batching controls.",
      file: "examples/vulnerable-contracts/unbounded_loop.rs",
      function: "distribute",
      line: 4,
      detector: "denial-of-service-pattern",
      evidence: ["line 9: for loop", "line 10: storage::set_recipient"],
      trace: [
        "Function distribute",
        "RequireAuth",
        "Validation",
        "Validation",
        "Loop",
        "WriteStorage",
      ],
      code_span: {
        start_line: 4,
        end_line: 4,
      },
    },
    {
      id: "SF-004",
      title: "Unchecked arithmetic path",
      severity: "medium",
      confidence: "medium",
      description:
        "Function `mint` uses raw arithmetic without an observed checked_* or saturating_* operation.",
      remediation:
        "Prefer checked arithmetic or explicit bounds checks around externally influenced values.",
      file: "examples/vulnerable-contracts/unchecked_arithmetic.rs",
      function: "mint",
      line: 4,
      detector: "integer-overflow-risk",
      evidence: ["line 5: amount + 1"],
      trace: ["Function mint", "RequireAuth", "Arithmetic", "WriteStorage"],
      code_span: {
        start_line: 4,
        end_line: 4,
      },
    },
  ],
};

export const sampleScanHistory: ScanRun[] = [
  {
    id: "scan-2026-05-13-01",
    timestamp: "2026-05-13T09:42:00Z",
    commitHash: "d82f1a3",
    branch: "main",
    status: "failed",
    summary: sampleReport.summary,
  },
  {
    id: "scan-2026-05-12-02",
    timestamp: "2026-05-12T18:20:00Z",
    commitHash: "b9018af",
    branch: "main",
    status: "failed",
    summary: {
      files: 3,
      functions: 3,
      findings: 5,
      critical: 1,
      high: 2,
      medium: 2,
      low: 0,
      informational: 0,
    },
  },
  {
    id: "scan-2026-05-11-03",
    timestamp: "2026-05-11T11:06:00Z",
    commitHash: "7aa65f9",
    branch: "feat/reporting",
    status: "passed",
    summary: {
      files: 3,
      functions: 3,
      findings: 0,
      critical: 0,
      high: 0,
      medium: 0,
      low: 0,
      informational: 0,
    },
  },
];

export const sampleAttackPath = {
  nodes: [
    {
      id: "entry",
      label: "External input",
      detail: "New admin address supplied by caller",
      x: 60,
      y: 84,
    },
    {
      id: "mutation",
      label: "State mutation",
      detail: "storage::set_admin writes privileged state",
      severity: "high" as const,
      x: 320,
      y: 84,
    },
    {
      id: "missing-auth",
      label: "Missing auth",
      detail: "No require_auth() observed on current admin",
      severity: "critical" as const,
      x: 320,
      y: 248,
    },
    {
      id: "escalation",
      label: "Privilege escalation",
      detail: "Caller can replace admin and seize privileged flow",
      severity: "critical" as const,
      x: 580,
      y: 166,
    },
  ] satisfies FlowNode[],
  edges: [
    {
      id: "entry-mutation",
      source: "entry",
      target: "mutation",
      label: "trusted parameter enters storage flow",
    },
    {
      id: "mutation-auth",
      source: "mutation",
      target: "missing-auth",
      label: "write occurs before authorization",
    },
    {
      id: "auth-escalation",
      source: "missing-auth",
      target: "escalation",
      label: "privileged role changes become exploitable",
    },
  ] satisfies FlowEdge[],
};

export const sampleReplaySteps: ReplayStep[] = [
  {
    id: "tx-01",
    tx: "Tx 01",
    function: "initialize()",
    actor: "deployer",
    effect: "Creates contract state and assigns initial admin.",
    state: "admin = deployer",
    outcome: "setup",
  },
  {
    id: "tx-02",
    tx: "Tx 02",
    function: "transfer_admin(new_admin)",
    actor: "attacker",
    effect: "Calls privileged transition without current admin auth.",
    state: "admin = attacker",
    outcome: "warning",
  },
  {
    id: "tx-03",
    tx: "Tx 03",
    function: "withdraw_treasury()",
    actor: "attacker",
    effect: "Newly acquired admin role opens protected withdrawal flow.",
    state: "treasury drained",
    outcome: "exploit",
  },
];

export const sampleStateTransitions: StateTransition[] = [
  {
    id: "admin",
    label: "Admin role",
    before: "deployer",
    after: "attacker",
    note: "Privileged state changes without signer validation.",
    severity: "critical",
  },
  {
    id: "recipient-loop",
    label: "Recipient queue",
    before: "32 recipients",
    after: "user-controlled growth",
    note: "Looping workload scales with attacker-provided list size.",
    severity: "medium",
  },
  {
    id: "mint-total",
    label: "Total supply",
    before: "amount",
    after: "amount + 1",
    note: "Arithmetic path lacks checked bounds or saturation strategy.",
    severity: "high",
  },
];

export const sampleSourceCode = `use soroban_sdk::{Address, Env};

pub fn transfer_admin(e: Env, new_admin: Address) {
    storage::set_admin(&e, &new_admin);
}`;

export function countFindingsBySeverity(report: AnalysisReport) {
  return [
    { severity: "critical" as const, value: report.summary.critical },
    { severity: "high" as const, value: report.summary.high },
    { severity: "medium" as const, value: report.summary.medium },
    { severity: "low" as const, value: report.summary.low },
    { severity: "informational" as const, value: report.summary.informational },
  ];
}

export function findingsByDetector(report: AnalysisReport) {
  const grouped = new Map<string, number>();

  for (const finding of report.findings) {
    grouped.set(finding.detector, (grouped.get(finding.detector) ?? 0) + 1);
  }

  return [...grouped.entries()]
    .map(([detector, count]) => ({ detector, count }))
    .sort((left, right) => right.count - left.count || left.detector.localeCompare(right.detector));
}

export function highestSeverity(report: AnalysisReport): Severity | null {
  const order: Severity[] = ["critical", "high", "medium", "low", "informational"];

  return order.find((severity) => countFindingsBySeverity(report).some((item) => item.severity === severity && item.value > 0)) ?? null;
}
