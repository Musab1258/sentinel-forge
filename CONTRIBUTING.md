# Contributing to Sentinel Forge

Sentinel Forge is being built as modular Soroban security infrastructure. Contributions should optimize for clarity, reproducibility, and evidence quality before feature count.

## Where contributions help most

- detector development for the static analyzer
- reporting and finding-export formats
- Soroban security research and documentation
- vulnerable and secure contract fixtures
- dashboard, landing page, and developer workflow tooling

See the contributor planning docs before picking up work:

- [Issue taxonomy](docs/contributing/issue-taxonomy.md)
- [Review process](docs/contributing/review-process.md)
- [Contributor roadmap](docs/contributing/roadmap.md)

## Local setup

### Requirements

- Rust stable with `clippy` and `rustfmt`
- Node.js 22 or newer
- `corepack` enabled for `pnpm`

### Install and build

```bash
git clone <repository-url>
cd sentinel-forge

corepack pnpm install
cargo build
corepack pnpm build
```

### Useful commands

```bash
corepack pnpm --filter @sentinel-forge/landing dev
corepack pnpm test
cargo test
cargo run -p static-analyzer --bin sentinel-forge -- scan examples/vulnerable-contracts
```

See [docs/guides/local-development.md](docs/guides/local-development.md) for more detail.

## First contributor path

The recommended first pass through the repository is:

1. Read [README.md](README.md) for project scope and current phase.
2. Read [docs/architecture/overview.md](docs/architecture/overview.md) and [docs/architecture/plugin-system.md](docs/architecture/plugin-system.md).
3. Follow [docs/guides/local-development.md](docs/guides/local-development.md) and [docs/guides/testing-guide.md](docs/guides/testing-guide.md).
4. Pick an issue labeled `good-first-issue`, `detector`, `docs`, `testing`, or `frontend`.
5. Use the relevant extension guide before opening a pull request.

## Repository layout

```text
apps/       public and operational interfaces
engines/    Rust analysis crates
packages/   shared TypeScript code and config
docs/       architecture, research, guides, and security notes
examples/   secure, vulnerable, benchmark, and exploit-oriented fixtures
scripts/    CI and developer workflow helpers
```

## Contribution standards

- keep architecture changes documented alongside the code
- prefer deterministic fixtures and reproducible findings
- add tests when behavior changes
- explain false-positive and false-negative tradeoffs for new security logic
- avoid shipping placeholder behavior without clear scope notes

## Detector development guide

The initial detector focus is:

- authorization correctness
- unsafe storage access
- privilege escalation flows
- missing input validation
- arithmetic risk
- denial-of-service patterns

When adding or refining a detector:

1. Define the contract pattern and why it matters for Soroban.
2. Document severity, confidence, and known blind spots.
3. Add fixtures that demonstrate both a positive and negative case.
4. Keep the output tied to evidence a developer can inspect.
5. Update [docs/guides/writing-detectors.md](docs/guides/writing-detectors.md) if the detector shape changes.

The deeper implementation guide is [docs/guides/writing-detectors.md](docs/guides/writing-detectors.md).

## Analyzer extension guide

New engine work should preserve the separation between ingest, normalized IR, detection, and reporting. Before building a new analyzer or changing the engine boundaries, read:

- [docs/guides/adding-analyzers.md](docs/guides/adding-analyzers.md)
- [docs/guides/extending-engines.md](docs/guides/extending-engines.md)
- [docs/architecture/analysis-pipeline.md](docs/architecture/analysis-pipeline.md)

## Reporter and plugin standards

Reporter work should preserve a stable core finding model and only transform presentation. New reporters should:

- consume structured findings rather than re-running analysis
- preserve severity, confidence, evidence, and remediation fields
- target a clear consumer such as CLI, JSON export, SARIF, or dashboard views

See [docs/guides/creating-reporters.md](docs/guides/creating-reporters.md) and [docs/architecture/plugin-system.md](docs/architecture/plugin-system.md).

## Testing requirements

Frontend changes:

- run `corepack pnpm test`
- run `corepack pnpm build` if rendering or config changes

Rust engine changes:

- run `cargo test`
- run `cargo build`

Documentation changes:

- verify links and command examples
- keep diagrams aligned with the current codebase

Cross-cutting fixture or workflow changes:

- run the command path you documented
- keep vulnerable and secure fixtures paired where practical
- explain regression impact in the pull request

See [docs/guides/testing-guide.md](docs/guides/testing-guide.md) for the fuller matrix.

## Pull request requirements

Every pull request should include:

- the architectural reason for the change
- tests or a clear explanation of why tests were not needed
- documentation updates when public behavior or contributor workflow changed
- security and false-positive considerations for detector work
- performance notes when scan paths, reporters, or UI data volume changed

The repository ships a PR template under `.github/PULL_REQUEST_TEMPLATE.md` to keep this consistent.

## Labels and issue structure

Contributor work is organized around stable labels such as `detector`, `research`, `fuzzing`, `symbolic-execution`, `frontend`, `backend`, `docs`, `testing`, and `security`. Priority labels are described in [docs/contributing/issue-taxonomy.md](docs/contributing/issue-taxonomy.md).

## Fuzzing and hostile-input guidance

Some future contributions will operate on untrusted contracts, malformed WASM, or adversarial fixtures. Treat those inputs as hostile by default and follow [docs/security/sandboxing.md](docs/security/sandboxing.md) before introducing new execution paths.

## Vulnerability reports and sensitive changes

If your change involves a real vulnerability or a bypass in Sentinel Forge itself, do not open a public issue first. Follow [SECURITY.md](SECURITY.md) and [docs/security/disclosure-policy.md](docs/security/disclosure-policy.md).
