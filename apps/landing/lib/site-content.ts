export const navigation = [
  { label: "Modules", href: "#modules" },
  { label: "Architecture", href: "#architecture" },
  { label: "Tooling", href: "#tooling" },
  { label: "Readiness", href: "#readiness" },
  { label: "Contribute", href: "#contribute" },
  { label: "Roadmap", href: "#roadmap" },
];

export const metrics = [
  { label: "Report formats", value: "04" },
  { label: "Frontend surfaces", value: "03" },
  { label: "Later-phase rails", value: "CI + SDK + registry" },
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
  "detector authoring",
  "reporting adapters",
  "benchmark fixtures",
  "VSCode diagnostics",
  "vulnerability advisories",
  "plugin manifests",
];

export const readinessTracks = [
  {
    phase: "Phase 6",
    title: "Application preparation",
    summary:
      "Maintainer-facing briefs, release-readiness criteria, and a repeatable demo walkthrough make the current implementation legible for external review.",
  },
  {
    phase: "Phase 7",
    title: "Expansion scaffolding",
    summary:
      "CI workflows, demo report export scripts, and starter engine plans create a dependable base for fuzzing, symbolic execution, and verification work.",
  },
  {
    phase: "Phase 8",
    title: "Ecosystem foundations",
    summary:
      "The advisory catalog, plugin registry, and SDK-facing types establish local contracts for a broader Soroban security ecosystem.",
  },
];

export const contributorLanes = [
  {
    name: "Detector and engine work",
    summary:
      "Clear detector authoring, analyzer extension, and engine hardening guides reduce the amount of reverse engineering new contributors need to do before shipping useful security logic.",
  },
  {
    name: "Issue and review workflow",
    summary:
      "Repository-side issue templates, label guidance, and a pull request checklist keep new work scoped, testable, and easier to review for security impact.",
  },
  {
    name: "Fixture and benchmark suites",
    summary:
      "Vulnerable, secure, benchmark, and exploit-scenario examples create a shared ground truth for regression work, onboarding, and future performance tracking.",
  },
];

export const ecosystemFoundations = [
  "application prep docs for demos and maintainer review",
  "GitHub Actions workflows for CI and report export",
  "starter plans for fuzzing, symbolic execution, and verification",
  "advisory catalog for vulnerability intelligence",
  "plugin registry manifests for detector and reporter extension points",
  "SDK types that stabilize integrations before a network API exists",
];

export const roadmap = [
  {
    phase: "Phase 1",
    focus:
      "Branding, monorepo infrastructure, landing page, analyzer skeleton, and contributor surface.",
  },
  {
    phase: "Phase 2",
    focus:
      "Architecture documentation, plugin standards, README maturity, and threat-model depth.",
  },
  {
    phase: "Phase 3",
    focus:
      "A credible static analyzer MVP with detector rules, text or JSON or SARIF reporting, and regression coverage.",
  },
  {
    phase: "Phase 4",
    focus:
      "Visualization and tooling: dashboard triage, exploit-lab storytelling, HTML reports, and IDE workflow scaffolding.",
  },
  {
    phase: "Phase 5",
    focus:
      "Contributor readiness: issue templates, review standards, richer fixtures, and extension guides for detector, reporter, and analyzer work.",
  },
  {
    phase: "Phase 6",
    focus:
      "Application preparation: maintainership-facing demo materials, release-readiness criteria, and repo polish for external review.",
  },
  {
    phase: "Phase 7",
    focus:
      "Post-application expansion scaffolding: CI workflows, demo artifacts, and starter plans for fuzzing, symbolic execution, and verification.",
  },
  {
    phase: "Phase 8",
    focus:
      "Security ecosystem scaling foundations: plugin registry, vulnerability intelligence catalog, and SDK-facing integration contracts.",
  },
];
