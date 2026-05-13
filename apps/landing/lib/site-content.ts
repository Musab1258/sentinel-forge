export const navigation = [
  { label: "Modules", href: "#modules" },
  { label: "Architecture", href: "#architecture" },
  { label: "Tooling", href: "#tooling" },
  { label: "Roadmap", href: "#roadmap" },
];

export const metrics = [
  { label: "Report formats", value: "04" },
  { label: "Frontend surfaces", value: "03" },
  { label: "Workflow bridge", value: "CLI + IDE" },
];

export const modules = [
  {
    name: "Static Analysis Engine",
    summary:
      "Rule-driven source and AST analysis for authorization flaws, unsafe storage access, arithmetic risks, and denial-of-service patterns.",
  },
  {
    name: "Fuzz Testing Engine",
    summary:
      "Property-based and state-mutating fuzz infrastructure built to uncover brittle assumptions before deployment.",
  },
  {
    name: "Symbolic Execution",
    summary:
      "Path exploration and constraint-driven inspection for high-confidence exploit discovery and branch coverage insight.",
  },
  {
    name: "Formal Verification",
    summary:
      "Invariant-oriented proofs and solver-backed correctness checks for contracts that need stronger safety guarantees.",
  },
  {
    name: "Exploit Simulation Lab",
    summary:
      "Replay hostile flows, visualize execution traces, and model adversarial scenarios in a controlled environment.",
  },
  {
    name: "Gas and Performance Profiler",
    summary:
      "Trace hotspots, estimate complexity, and surface optimization opportunities for Soroban execution paths.",
  },
];

export const architecture = [
  {
    stage: "Ingest",
    description:
      "Contracts and fixtures enter through CLI and future APIs, then move into source and WASM parsing pipelines.",
  },
  {
    stage: "Analyze",
    description:
      "Detectors operate over control-flow, data-flow, and authorization semantics, with future symbolic and fuzz hooks.",
  },
  {
    stage: "Explain",
    description:
      "Findings are ranked by severity, tied to evidence, and prepared for JSON, SARIF, HTML, and dashboard delivery.",
  },
  {
    stage: "Integrate",
    description:
      "Teams consume the output in local workflows, CI gates, IDE surfaces, and the future operational dashboard.",
  },
];

export const cliSteps = [
  "Scan a Soroban contract or fixture directory from the CLI.",
  "Export the same findings as text, JSON, SARIF, or HTML.",
  "Route the analyzer output into dashboard views, exploit replay, or editor diagnostics.",
  "Use CI mode to fail builds when the analyzer detects security issues.",
];

export const researchTracks = [
  "dashboard triage",
  "exploit replay",
  "HTML reporting",
  "VSCode diagnostics",
];

export const roadmap = [
  {
    phase: "Phase 1",
    focus: "Branding, monorepo infrastructure, landing page, analyzer skeleton, and contributor surface.",
  },
  {
    phase: "Phase 2",
    focus: "Architecture documentation, plugin standards, README maturity, and threat-model depth.",
  },
  {
    phase: "Phase 3",
    focus: "A credible static analyzer MVP with detector rules, text or JSON or SARIF reporting, and regression coverage.",
  },
  {
    phase: "Phase 4",
    focus: "Visualization and tooling: dashboard triage, exploit-lab storytelling, HTML reports, and IDE workflow scaffolding.",
  },
];
