# Contributing to Sentinel Forge

Sentinel Forge is being built as modular Soroban security infrastructure. Contributions should optimize for clarity, reproducibility, and evidence quality before feature count.

## Where contributions help most

- detector development for the static analyzer
- reporting and finding-export formats
- Soroban security research and documentation
- vulnerable and secure contract fixtures
- dashboard, landing page, and developer workflow tooling

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

## Repository layout

```text
apps/       public and operational interfaces
engines/    Rust analysis crates
packages/   shared TypeScript code and config
docs/       architecture, research, guides, and security notes
examples/   secure and intentionally vulnerable fixtures
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

## Fuzzing and hostile-input guidance

Some future contributions will operate on untrusted contracts, malformed WASM, or adversarial fixtures. Treat those inputs as hostile by default and follow [docs/security/sandboxing.md](docs/security/sandboxing.md) before introducing new execution paths.

## Vulnerability reports and sensitive changes

If your change involves a real vulnerability or a bypass in Sentinel Forge itself, do not open a public issue first. Follow [SECURITY.md](SECURITY.md) and [docs/security/disclosure-policy.md](docs/security/disclosure-policy.md).
