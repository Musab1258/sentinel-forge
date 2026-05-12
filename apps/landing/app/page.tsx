import {
  architecture,
  cliSteps,
  metrics,
  modules,
  navigation,
  researchTracks,
  roadmap,
} from "../lib/site-content";

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
    <div className="max-w-2xl space-y-4">
      <p className="eyebrow">{eyebrow}</p>
      <h2 className="font-heading text-3xl font-semibold tracking-tight text-slate-50 sm:text-4xl">
        {title}
      </h2>
      <p className="text-base leading-7 text-slate-300">{description}</p>
    </div>
  );
}

export default function Home() {
  return (
    <main className="mx-auto flex min-h-screen w-full max-w-7xl flex-col px-6 pb-16 pt-6 sm:px-10 lg:px-12">
      <header className="surface-panel sticky top-4 z-30 mb-10 flex items-center justify-between gap-6 px-5 py-4 backdrop-blur">
        <div>
          <p className="font-heading text-lg font-semibold text-slate-50">Sentinel Forge</p>
          <p className="text-sm text-slate-400">
            Security and verification infrastructure for Soroban.
          </p>
        </div>
        <nav className="hidden items-center gap-5 text-sm text-slate-300 md:flex">
          {navigation.map((item) => (
            <a key={item.href} href={item.href} className="transition hover:text-slate-50">
              {item.label}
            </a>
          ))}
        </nav>
      </header>

      <section className="grid gap-8 lg:grid-cols-[1.2fr_0.8fr] lg:items-end">
        <div className="space-y-8">
          <div className="space-y-6">
            <p className="eyebrow">Phase 1 foundation</p>
            <h1 className="font-heading text-5xl font-semibold tracking-tight text-slate-50 sm:text-6xl">
              Advanced security and verification infrastructure for Soroban smart
              contracts.
            </h1>
            <p className="max-w-2xl text-lg leading-8 text-slate-300">
              Sentinel Forge is being shaped as the research, analysis, and developer
              workflow layer for secure Soroban development. The first release focuses on
              a credible public surface, a real analyzer skeleton, and a clear path
              toward detector-driven security automation.
            </p>
          </div>

          <div className="flex flex-wrap gap-4">
            <a href="#architecture" className="primary-button">
              Review architecture
            </a>
            <a href="#tooling" className="secondary-button">
              Inspect tooling path
            </a>
          </div>

          <div className="grid gap-4 sm:grid-cols-3">
            {metrics.map((metric) => (
              <article key={metric.label} className="surface-panel p-5">
                <p className="text-xs uppercase tracking-[0.24em] text-slate-400">
                  {metric.label}
                </p>
                <p className="mt-3 font-heading text-2xl font-semibold text-slate-50">
                  {metric.value}
                </p>
              </article>
            ))}
          </div>
        </div>

        <aside className="surface-panel relative overflow-hidden p-6">
          <div className="absolute inset-0 bg-[radial-gradient(circle_at_top_right,_rgba(59,130,246,0.18),_transparent_42%)]" />
          <div className="relative space-y-5">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm uppercase tracking-[0.28em] text-slate-400">
                  Live scan posture
                </p>
                <p className="mt-2 font-heading text-2xl font-semibold text-slate-50">
                  Analyzer bootstrap
                </p>
              </div>
              <span className="inline-flex items-center gap-2 rounded-full border border-emerald-500/30 bg-emerald-500/10 px-3 py-1 text-xs uppercase tracking-[0.24em] text-emerald-300">
                <span className="pulse-dot" />
                active
              </span>
            </div>

            <div className="rounded-3xl border border-white/10 bg-slate-950/75 p-4 font-mono text-sm text-slate-300 shadow-[0_24px_70px_rgba(0,0,0,0.3)]">
              <p className="text-slate-500">$ sentinel-forge scan ./contracts/treasury.rs</p>
              <p className="mt-3 text-rose-300">critical: authorization flow missing signer validation</p>
              <p className="mt-1 text-amber-300">medium: storage guard absent on state transition</p>
              <p className="mt-4 text-slate-400">report exported to ./reports/treasury.json</p>
            </div>

            <div className="grid gap-3 sm:grid-cols-2">
              <article className="rounded-3xl border border-white/10 bg-white/5 p-4">
                <p className="text-xs uppercase tracking-[0.24em] text-slate-400">
                  First engine
                </p>
                <p className="mt-2 text-sm leading-6 text-slate-200">
                  Static analyzer crate and CLI entrypoint initialized in the Rust
                  workspace.
                </p>
              </article>
              <article className="rounded-3xl border border-white/10 bg-white/5 p-4">
                <p className="text-xs uppercase tracking-[0.24em] text-slate-400">
                  Public surface
                </p>
                <p className="mt-2 text-sm leading-6 text-slate-200">
                  Landing page mirrors the serious, institutional tone defined in the
                  product notes.
                </p>
              </article>
            </div>
          </div>
        </aside>
      </section>

      <section id="modules" className="section-shell mt-24">
        <SectionHeading
          eyebrow="Security engine map"
          title="The platform is designed as a layered security research stack."
          description="The landing page centers the modules described in the PRD so contributors can understand the destination before the deeper engines arrive."
        />

        <div className="mt-10 grid gap-4 md:grid-cols-2 xl:grid-cols-3">
          {modules.map((module) => (
            <article key={module.name} className="surface-panel flex flex-col gap-4 p-6">
              <div className="flex items-center justify-between">
                <h3 className="font-heading text-xl font-semibold text-slate-50">
                  {module.name}
                </h3>
                <span className="h-2.5 w-2.5 rounded-full bg-sky-400 shadow-[0_0_24px_rgba(56,189,248,0.9)]" />
              </div>
              <p className="text-sm leading-7 text-slate-300">{module.summary}</p>
            </article>
          ))}
        </div>
      </section>

      <section id="architecture" className="section-shell mt-24 grid gap-12 lg:grid-cols-[0.95fr_1.05fr]">
        <SectionHeading
          eyebrow="Architecture visualization"
          title="A public research surface, a modular Rust core, and workflows that feed developers actionable evidence."
          description="Phase 0 and Phase 1 both emphasize maintainability and extensibility. The repository layout reflects that by separating public apps, engine crates, shared packages, and contributor documentation."
        />

        <div className="grid gap-4">
          {architecture.map((item, index) => (
            <article key={item.stage} className="surface-panel flex gap-4 p-5">
              <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl border border-sky-500/20 bg-sky-500/10 font-heading text-lg font-semibold text-sky-300">
                0{index + 1}
              </div>
              <div>
                <h3 className="font-heading text-xl font-semibold text-slate-50">
                  {item.stage}
                </h3>
                <p className="mt-2 text-sm leading-7 text-slate-300">{item.description}</p>
              </div>
            </article>
          ))}
        </div>
      </section>

      <section id="tooling" className="section-shell mt-24 grid gap-6 lg:grid-cols-[0.8fr_1.2fr]">
        <div className="surface-panel p-6">
          <p className="eyebrow">Exploit replay preview</p>
          <h2 className="mt-4 font-heading text-3xl font-semibold text-slate-50">
            The visual language stays technical, not theatrical.
          </h2>
          <p className="mt-4 text-sm leading-7 text-slate-300">
            Instead of generic crypto marketing, the site previews the kind of evidence
            Sentinel Forge is intended to produce: traces, severity context, and analysis
            steps that a developer can act on.
          </p>

          <div className="mt-8 grid gap-3">
            <div className="rounded-3xl border border-rose-500/20 bg-rose-500/8 p-4">
              <p className="font-mono text-xs uppercase tracking-[0.24em] text-rose-300">
                attack trace
              </p>
              <p className="mt-3 text-sm text-slate-200">
                signer delegated call -&gt; privileged state mutation -&gt; missing
                invariant guard -&gt; exploitable branch
              </p>
            </div>
            <div className="rounded-3xl border border-amber-500/20 bg-amber-500/8 p-4">
              <p className="font-mono text-xs uppercase tracking-[0.24em] text-amber-300">
                evidence bundle
              </p>
              <p className="mt-3 text-sm text-slate-200">
                source locations, detector rationale, remediation hint, and export-ready
                report metadata
              </p>
            </div>
          </div>
        </div>

        <div className="grid gap-4">
          <article className="surface-panel p-6">
            <p className="eyebrow">CLI workflow</p>
            <div className="mt-5 grid gap-4">
              {cliSteps.map((step, index) => (
                <div key={step} className="flex gap-4">
                  <div className="mt-1 flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-white/10 bg-white/5 text-sm text-slate-200">
                    {index + 1}
                  </div>
                  <p className="text-sm leading-7 text-slate-300">{step}</p>
                </div>
              ))}
            </div>
          </article>

          <article className="surface-panel p-6">
            <p className="eyebrow">Research priorities</p>
            <div className="mt-5 flex flex-wrap gap-3">
              {researchTracks.map((track) => (
                <span
                  key={track}
                  className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200"
                >
                  {track}
                </span>
              ))}
            </div>
          </article>

          <article className="surface-panel p-6">
            <p className="eyebrow">Open-source posture</p>
            <p className="mt-4 text-sm leading-7 text-slate-300">
              The repo already includes contributing, security, research, and architecture
              surfaces so the project reads like infrastructure in progress rather than a
              loosely scoped concept.
            </p>
          </article>
        </div>
      </section>

      <section id="roadmap" className="section-shell mt-24">
        <SectionHeading
          eyebrow="Execution path"
          title="The roadmap keeps the early scope narrow enough to look maintainable."
          description="The docs explicitly warn against trying to build every engine immediately. This setup follows that advice by giving the project a credible shell while leaving room for deeper implementation."
        />

        <div className="mt-10 grid gap-4 lg:grid-cols-4">
          {roadmap.map((item) => (
            <article key={item.phase} className="surface-panel p-5">
              <p className="font-mono text-xs uppercase tracking-[0.24em] text-sky-300">
                {item.phase}
              </p>
              <p className="mt-4 text-sm leading-7 text-slate-300">{item.focus}</p>
            </article>
          ))}
        </div>
      </section>

      <footer className="section-shell mt-24 flex flex-col gap-6 border-t border-white/10 pt-8 text-sm text-slate-400 md:flex-row md:items-center md:justify-between">
        <p>
          Sentinel Forge is being built as security and verification infrastructure for
          the Soroban ecosystem.
        </p>
        <div className="flex gap-5">
          <a href="#modules" className="transition hover:text-slate-50">
            Modules
          </a>
          <a href="#architecture" className="transition hover:text-slate-50">
            Architecture
          </a>
          <a href="#roadmap" className="transition hover:text-slate-50">
            Roadmap
          </a>
        </div>
      </footer>
    </main>
  );
}
