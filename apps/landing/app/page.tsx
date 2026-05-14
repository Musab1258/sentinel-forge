import {
  architecture,
  cliSteps,
  contributorLanes,
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
          <p className="font-heading text-lg font-semibold text-slate-50">
            Sentinel Forge
          </p>
          <p className="text-sm text-slate-400">
            Security and verification infrastructure for Soroban.
          </p>
        </div>
        <nav className="hidden items-center gap-5 text-sm text-slate-300 md:flex">
          {navigation.map((item) => (
            <a
              key={item.href}
              href={item.href}
              className="transition hover:text-slate-50"
            >
              {item.label}
            </a>
          ))}
        </nav>
      </header>

      <section className="grid gap-8 lg:grid-cols-[1.2fr_0.8fr] lg:items-end">
        <div className="space-y-8">
          <div className="space-y-6">
            <p className="eyebrow">Phase 5 contributor readiness</p>
            <h1 className="font-heading text-5xl font-semibold tracking-tight text-slate-50 sm:text-6xl">
              Advanced security and verification infrastructure for Soroban
              smart contracts.
            </h1>
            <p className="max-w-2xl text-lg leading-8 text-slate-300">
              Sentinel Forge is being shaped as the research, analysis, and
              developer workflow layer for secure Soroban development. The
              current milestone hardens the open-source contributor layer with
              clearer issue structure, extension guides, benchmark fixtures, and
              review expectations on top of the phase 4 tooling surface.
            </p>
          </div>

          <div className="flex flex-wrap gap-4">
            <a href="#architecture" className="primary-button">
              Review architecture
            </a>
            <a href="#contribute" className="secondary-button">
              Inspect contributor path
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
                  Contributor layer active
                </p>
              </div>
              <span className="inline-flex items-center gap-2 rounded-full border border-emerald-500/30 bg-emerald-500/10 px-3 py-1 text-xs uppercase tracking-[0.24em] text-emerald-300">
                <span className="pulse-dot" />
                phase 5
              </span>
            </div>

            <div className="rounded-3xl border border-white/10 bg-slate-950/75 p-4 font-mono text-sm text-slate-300 shadow-[0_24px_70px_rgba(0,0,0,0.3)]">
              <p className="text-slate-500">
                $ sentinel-forge scan ./contracts/treasury.rs --format html
                --output ./reports/treasury.html
              </p>
              <p className="mt-3 text-rose-300">
                critical: authorization flow missing signer validation
              </p>
              <p className="mt-1 text-amber-300">
                medium: storage guard absent on state transition
              </p>
              <p className="mt-4 text-slate-400">
                report written to ./reports/treasury.html
              </p>
            </div>

            <div className="grid gap-3 sm:grid-cols-2">
              <article className="rounded-3xl border border-white/10 bg-white/5 p-4">
                <p className="text-xs uppercase tracking-[0.24em] text-slate-400">
                  First engine
                </p>
                <p className="mt-2 text-sm leading-6 text-slate-200">
                  Static analyzer, HTML reporting, and shared finding models now
                  feed multiple visualization surfaces in the workspace.
                </p>
              </article>
              <article className="rounded-3xl border border-white/10 bg-white/5 p-4">
                <p className="text-xs uppercase tracking-[0.24em] text-slate-400">
                  Public surface
                </p>
                <p className="mt-2 text-sm leading-6 text-slate-200">
                  Contributor guides, issue templates, and organized fixtures
                  now sit beside the apps and engine crates to demonstrate a
                  credible open-source workflow.
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
            <article
              key={module.name}
              className="surface-panel flex flex-col gap-4 p-6"
            >
              <div className="flex items-center justify-between">
                <h3 className="font-heading text-xl font-semibold text-slate-50">
                  {module.name}
                </h3>
                <span className="h-2.5 w-2.5 rounded-full bg-sky-400 shadow-[0_0_24px_rgba(56,189,248,0.9)]" />
              </div>
              <p className="text-sm leading-7 text-slate-300">
                {module.summary}
              </p>
            </article>
          ))}
        </div>
      </section>

      <section
        id="architecture"
        className="section-shell mt-24 grid gap-12 lg:grid-cols-[0.95fr_1.05fr]"
      >
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
                <p className="mt-2 text-sm leading-7 text-slate-300">
                  {item.description}
                </p>
              </div>
            </article>
          ))}
        </div>
      </section>

      <section
        id="tooling"
        className="section-shell mt-24 grid gap-6 lg:grid-cols-[0.8fr_1.2fr]"
      >
        <div className="surface-panel p-6">
          <p className="eyebrow">Visualization stack</p>
          <h2 className="mt-4 font-heading text-3xl font-semibold text-slate-50">
            The visual language stays technical, not theatrical.
          </h2>
          <p className="mt-4 text-sm leading-7 text-slate-300">
            Phase 4 turns the platform brief into usable tooling surfaces: the
            dashboard for triage, the exploit lab for visual reasoning, and the
            VSCode bridge for inline workflow adoption.
          </p>

          <div className="mt-8 grid gap-3">
            <div className="rounded-3xl border border-rose-500/20 bg-rose-500/8 p-4">
              <p className="font-mono text-xs uppercase tracking-[0.24em] text-rose-300">
                dashboard
              </p>
              <p className="mt-3 text-sm text-slate-200">
                severity charts, findings tables, code-linked evidence, and scan
                history from the real analyzer report shape
              </p>
            </div>
            <div className="rounded-3xl border border-amber-500/20 bg-amber-500/8 p-4">
              <p className="font-mono text-xs uppercase tracking-[0.24em] text-amber-300">
                exploit lab
              </p>
              <p className="mt-3 text-sm text-slate-200">
                attack-path graphs, replay timelines, and state transition maps
                that explain exploit progression visually
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
              The repo now includes contributor-facing infrastructure as code,
              not just roadmap promises: issue templates, PR standards,
              extension guides, and a structured examples suite that supports
              onboarding and regression work.
            </p>
          </article>
        </div>
      </section>

      <section
        id="contribute"
        className="section-shell mt-24 grid gap-12 lg:grid-cols-[0.9fr_1.1fr]"
      >
        <SectionHeading
          eyebrow="Contributor infrastructure"
          title="Phase 5 reduces contributor friction without flattening the architecture."
          description="The project now treats onboarding, fixture quality, and review structure as part of the product. That is how a security repository becomes maintainable instead of merely ambitious."
        />

        <div className="grid gap-4">
          {contributorLanes.map((lane) => (
            <article key={lane.name} className="surface-panel p-6">
              <h3 className="font-heading text-xl font-semibold text-slate-50">
                {lane.name}
              </h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">
                {lane.summary}
              </p>
            </article>
          ))}
        </div>
      </section>

      <section id="roadmap" className="section-shell mt-24">
        <SectionHeading
          eyebrow="Execution path"
          title="The roadmap keeps the early scope narrow enough to look maintainable."
          description="The docs explicitly warn against trying to build every engine immediately. This setup follows that advice by giving the project a credible shell while leaving room for deeper implementation."
        />

        <div className="mt-10 grid gap-4 md:grid-cols-2 xl:grid-cols-5">
          {roadmap.map((item) => (
            <article key={item.phase} className="surface-panel p-5">
              <p className="font-mono text-xs uppercase tracking-[0.24em] text-sky-300">
                {item.phase}
              </p>
              <p className="mt-4 text-sm leading-7 text-slate-300">
                {item.focus}
              </p>
            </article>
          ))}
        </div>
      </section>

      <footer className="section-shell mt-24 flex flex-col gap-6 border-t border-white/10 pt-8 text-sm text-slate-400 md:flex-row md:items-center md:justify-between">
        <p>
          Sentinel Forge is being built as security and verification
          infrastructure for the Soroban ecosystem.
        </p>
        <div className="flex gap-5">
          <a href="#modules" className="transition hover:text-slate-50">
            Modules
          </a>
          <a href="#architecture" className="transition hover:text-slate-50">
            Architecture
          </a>
          <a href="#contribute" className="transition hover:text-slate-50">
            Contribute
          </a>
          <a href="#roadmap" className="transition hover:text-slate-50">
            Roadmap
          </a>
        </div>
      </footer>
    </main>
  );
}
