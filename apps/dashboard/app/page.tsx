import {
  countFindingsBySeverity,
  findingsByDetector,
  highestSeverity,
  sampleReport,
  sampleScanHistory,
  sampleSourceCode,
  severityColors,
  severityLabels,
} from "@sentinel-forge/shared";

function SectionHeading({
  eyebrow,
  title,
  description,
}: {
  eyebrow: string;
  title: string;
  description: string;
}) {
  return (
    <div className="space-y-3">
      <p className="eyebrow">{eyebrow}</p>
      <div className="space-y-2">
        <h2 className="font-heading text-3xl font-semibold tracking-tight text-slate-50">
          {title}
        </h2>
        <p className="max-w-3xl text-sm leading-7 text-slate-300">{description}</p>
      </div>
    </div>
  );
}

export default function DashboardPage() {
  const severityData = countFindingsBySeverity(sampleReport).filter((item) => item.value > 0);
  const detectorCoverage = findingsByDetector(sampleReport);
  const topFinding = sampleReport.findings[0];
  const activeSeverity = highestSeverity(sampleReport) ?? "informational";
  const codeLines = sampleSourceCode.split("\n");

  return (
    <main className="mx-auto flex min-h-screen w-full max-w-[1440px] gap-6 px-4 py-4 sm:px-6 lg:px-8">
      <aside className="dashboard-panel sticky top-4 hidden h-[calc(100vh-2rem)] w-72 shrink-0 flex-col justify-between p-5 lg:flex">
        <div className="space-y-8">
          <div className="space-y-2">
            <p className="font-heading text-2xl font-semibold text-slate-50">Sentinel Forge</p>
            <p className="text-sm leading-6 text-slate-400">
              Security dashboard for Soroban findings, trace evidence, and CI posture.
            </p>
          </div>

          <nav className="space-y-2 text-sm text-slate-300">
            {["Overview", "Findings", "Contracts", "Traces", "Reports", "Settings"].map(
              (item, index) => (
                <a
                  key={item}
                  href={`#section-${item.toLowerCase()}`}
                  className={`sidebar-link ${index === 0 ? "sidebar-link-active" : ""}`}
                >
                  <span className="sidebar-dot" />
                  {item}
                </a>
              ),
            )}
          </nav>
        </div>

        <div className="rounded-[1.6rem] border border-sky-500/20 bg-sky-500/10 p-4">
          <p className="eyebrow">Current gate</p>
          <h2 className="mt-3 font-heading text-xl font-semibold text-slate-50">
            {severityLabels[activeSeverity]} finding detected
          </h2>
          <p className="mt-3 text-sm leading-7 text-slate-300">
            CI should fail until the privilege escalation and authorization gaps are fixed.
          </p>
        </div>
      </aside>

      <div className="flex-1 space-y-6">
        <header className="dashboard-panel px-5 py-5 sm:px-6">
          <div className="flex flex-col gap-5 xl:flex-row xl:items-end xl:justify-between">
            <div className="max-w-3xl space-y-3">
              <p className="eyebrow">Phase 4 dashboard</p>
              <h1 className="font-heading text-4xl font-semibold tracking-tight text-slate-50 sm:text-5xl">
                Findings, scan history, and trace evidence in one security surface.
              </h1>
              <p className="text-base leading-8 text-slate-300">
                This dashboard is the visualization layer described in phase 4: it turns
                the analyzer&apos;s structured findings into triage cards, detector coverage,
                scan history, and code-linked exploit reasoning.
              </p>
            </div>

            <div className="grid gap-3 sm:grid-cols-3">
              <article className="metric-card">
                <span>Target</span>
                <strong>{sampleReport.target}</strong>
              </article>
              <article className="metric-card">
                <span>Findings</span>
                <strong>{sampleReport.summary.findings}</strong>
              </article>
              <article className="metric-card">
                <span>Formats</span>
                <strong>text json sarif html</strong>
              </article>
            </div>
          </div>
        </header>

        <section
          id="section-overview"
          className="grid gap-6 xl:grid-cols-[1.2fr_0.8fr]"
        >
          <div className="dashboard-panel p-5 sm:p-6">
            <SectionHeading
              eyebrow="Risk posture"
              title="Severity distribution"
              description="The first view keeps the signal narrow: what was found, how severe it is, and where contributor attention should go first."
            />

            <div className="mt-6 grid gap-4 md:grid-cols-2">
              {severityData.map((item) => {
                const ratio = Math.max(
                  12,
                  (item.value / sampleReport.summary.findings) * 100,
                );

                return (
                  <article key={item.severity} className="severity-card">
                    <div className="flex items-center justify-between">
                      <p className="font-heading text-lg font-semibold text-slate-50">
                        {severityLabels[item.severity]}
                      </p>
                      <span className="text-sm text-slate-300">{item.value}</span>
                    </div>
                    <div className="mt-4 h-3 rounded-full bg-white/5">
                      <div
                        className="h-full rounded-full"
                        style={{
                          width: `${ratio}%`,
                          backgroundColor: severityColors[item.severity],
                        }}
                      />
                    </div>
                  </article>
                );
              })}
            </div>
          </div>

          <div className="dashboard-panel p-5 sm:p-6">
            <SectionHeading
              eyebrow="Detector coverage"
              title="Rule concentration"
              description="Detector distribution helps show whether the project is seeing repeated authorization failures, arithmetic issues, or DoS-pattern drift."
            />

            <div className="mt-6 space-y-3">
              {detectorCoverage.map((item) => (
                <article key={item.detector} className="detector-row">
                  <div className="space-y-1">
                    <p className="font-mono text-sm text-slate-100">{item.detector}</p>
                    <p className="text-xs uppercase tracking-[0.18em] text-slate-500">
                      findings routed by detector
                    </p>
                  </div>
                  <span className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-sm text-slate-200">
                    {item.count}
                  </span>
                </article>
              ))}
            </div>
          </div>
        </section>

        <section id="section-findings" className="dashboard-panel p-5 sm:p-6">
          <SectionHeading
            eyebrow="Findings table"
            title="Triage entries stay attached to evidence and remediation."
            description="The table is intended to be filterable later, but even in the static MVP it already mirrors the analyzer schema closely enough to support real reporting workflows."
          />

          <div className="mt-6 overflow-x-auto">
            <table className="w-full min-w-[860px] border-separate border-spacing-y-3">
              <thead>
                <tr className="text-left text-xs uppercase tracking-[0.22em] text-slate-500">
                  <th className="pb-2 pr-4">Severity</th>
                  <th className="pb-2 pr-4">Finding</th>
                  <th className="pb-2 pr-4">Detector</th>
                  <th className="pb-2 pr-4">Location</th>
                  <th className="pb-2">Confidence</th>
                </tr>
              </thead>
              <tbody>
                {sampleReport.findings.map((finding) => (
                  <tr key={`${finding.id}-${finding.detector}`} className="table-row-card">
                    <td className="rounded-l-[1.5rem] px-4 py-4 align-top">
                      <span
                        className="severity-pill"
                        style={{
                          color: severityColors[finding.severity],
                          borderColor: `${severityColors[finding.severity]}55`,
                          backgroundColor: `${severityColors[finding.severity]}18`,
                        }}
                      >
                        {severityLabels[finding.severity]}
                      </span>
                    </td>
                    <td className="px-4 py-4 align-top">
                      <p className="font-heading text-lg font-semibold text-slate-50">
                        {finding.title}
                      </p>
                      <p className="mt-2 max-w-xl text-sm leading-7 text-slate-300">
                        {finding.description}
                      </p>
                    </td>
                    <td className="px-4 py-4 align-top font-mono text-sm text-slate-200">
                      {finding.detector}
                    </td>
                    <td className="px-4 py-4 align-top text-sm text-slate-300">
                      {finding.file}:{finding.line}
                    </td>
                    <td className="rounded-r-[1.5rem] px-4 py-4 align-top text-sm text-slate-300">
                      {finding.confidence}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>

        <section
          id="section-contracts"
          className="grid gap-6 xl:grid-cols-[0.92fr_1.08fr]"
        >
          <div className="dashboard-panel p-5 sm:p-6">
            <SectionHeading
              eyebrow="Code explorer"
              title="Source-linked context"
              description="Phase 4 calls for a code explorer with inline finding context. The MVP uses a static render, but the line-oriented structure mirrors how Monaco or Shiki integration can slot in later."
            />

            <div className="code-panel mt-6">
              {codeLines.map((line, index) => {
                const lineNumber = index + 1;
                const flagged = lineNumber === topFinding.line;

                return (
                  <div
                    key={lineNumber}
                    className={`code-line ${flagged ? "code-line-flagged" : ""}`}
                  >
                    <span className="code-gutter">{lineNumber}</span>
                    <code>{line || " "}</code>
                    {flagged ? (
                      <span className="code-callout">
                        {severityLabels[topFinding.severity]} · {topFinding.title}
                      </span>
                    ) : null}
                  </div>
                );
              })}
            </div>
          </div>

          <div id="section-traces" className="dashboard-panel p-5 sm:p-6">
            <SectionHeading
              eyebrow="Trace renderer"
              title="Exploit reasoning stays visible, not buried in raw text."
              description="Each step below is derived from the analyzer trace path. The point of the dashboard is to make a reviewer understand the exploit chain quickly without reading the detector code first."
            />

            <div className="mt-6 grid gap-4">
              {topFinding.trace.map((step, index) => (
                <article key={step} className="trace-step">
                  <div className="trace-index">{index + 1}</div>
                  <div>
                    <p className="font-heading text-lg font-semibold text-slate-50">{step}</p>
                    <p className="mt-1 text-sm leading-7 text-slate-300">
                      {index === 0
                        ? "Entry point into the vulnerable execution path."
                        : index === topFinding.trace.length - 1
                          ? "Terminal step where the analyzer maps exploit impact."
                          : "Intermediate execution state that connects evidence to impact."}
                    </p>
                  </div>
                </article>
              ))}
            </div>

            <div className="mt-6 rounded-[1.8rem] border border-white/10 bg-white/5 p-5">
              <p className="eyebrow">Evidence bundle</p>
              <ul className="mt-4 space-y-3 text-sm leading-7 text-slate-300">
                {topFinding.evidence.map((item) => (
                  <li key={item} className="flex gap-3">
                    <span className="mt-2 h-2 w-2 rounded-full bg-sky-400" />
                    <span>{item}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </section>

        <section id="section-reports" className="dashboard-panel p-5 sm:p-6">
          <SectionHeading
            eyebrow="Scan history"
            title="Audit timeline and CI posture"
            description="Phase 4 explicitly calls for scan history and regression visibility. The entries below are shaped so they can later move behind an API without changing the visualization contract."
          />

          <div className="mt-6 grid gap-4 lg:grid-cols-3">
            {sampleScanHistory.map((run) => (
              <article key={run.id} className="history-card">
                <div className="flex items-center justify-between gap-3">
                  <p className="font-mono text-sm text-slate-100">{run.commitHash}</p>
                  <span
                    className={`status-pill ${
                      run.status === "failed" ? "status-pill-failed" : "status-pill-passed"
                    }`}
                  >
                    {run.status}
                  </span>
                </div>
                <p className="mt-4 text-sm text-slate-300">
                  {new Date(run.timestamp).toLocaleString("en-US", {
                    dateStyle: "medium",
                    timeStyle: "short",
                    timeZone: "UTC",
                  })}
                </p>
                <div className="mt-5 grid grid-cols-2 gap-3 text-sm text-slate-300">
                  <div className="mini-stat">
                    <span>Findings</span>
                    <strong>{run.summary.findings}</strong>
                  </div>
                  <div className="mini-stat">
                    <span>Critical</span>
                    <strong>{run.summary.critical}</strong>
                  </div>
                </div>
              </article>
            ))}
          </div>
        </section>
      </div>
    </main>
  );
}
