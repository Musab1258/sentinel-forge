export const navigation = [
  { label: "Modules", href: "#modules" },
  { label: "Architecture", href: "#architecture" },
  { label: "Tooling", href: "#tooling" },
  { label: "Roadmap", href: "#roadmap" },
];

export const metrics = [
  { label: "Analysis modes planned", value: "06" },
  { label: "Initial detector themes", value: "04" },
  { label: "Execution surfaces", value: "AST + WASM" },
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
  "Scan a Soroban contract locally with the analyzer CLI.",
  "Export structured findings as machine-readable JSON.",
  "Promote the same output into CI and future dashboard views.",
];

export const researchTracks = [
  "authorization correctness",
  "state mutation safety",
  "WASM-aware execution insight",
  "formal reasoning readiness",
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
    focus: "A credible static analyzer MVP with detector rules, evidence-rich reporting, and regression coverage.",
  },
  {
    phase: "Phase 4+",
    focus: "Visualization, fuzzing, symbolic execution, exploit replay, and ecosystem-facing security workflows.",
  },
];

