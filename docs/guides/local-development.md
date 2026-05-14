# Local Development

This guide covers the local workflow for working in the Sentinel Forge monorepo.

## Requirements

- Rust stable
- `corepack` with `pnpm`
- Node.js 22 or newer

## Install

```bash
corepack pnpm install
cargo build
```

## Recommended onboarding sequence

1. Read `README.md` and `docs/architecture/overview.md`.
2. Read the guide that matches your intended change:
   `writing-detectors.md`, `adding-analyzers.md`, `creating-reporters.md`, or `extending-engines.md`.
3. Run the smallest relevant test loop before making changes.
4. Keep docs, fixtures, and implementation aligned in the same branch.

## Common tasks

Run the landing page:

```bash
corepack pnpm --filter @sentinel-forge/landing dev
```

Run the dashboard:

```bash
corepack pnpm --filter @sentinel-forge/dashboard dev
```

Run the exploit lab:

```bash
corepack pnpm --filter @sentinel-forge/exploit-lab dev
```

Run frontend tests:

```bash
corepack pnpm test
```

Run Rust tests:

```bash
cargo test
```

Run the analyzer bootstrap binary:

```bash
cargo run -p static-analyzer --bin sentinel-forge
```

Export an HTML report:

```bash
cargo run -p static-analyzer --bin sentinel-forge -- scan examples/vulnerable-contracts --format html --output reports/vulnerabilities.html
```

Run the landing page tests:

```bash
corepack pnpm --filter @sentinel-forge/landing test
```

Run static analyzer tests only:

```bash
cargo test -p static-analyzer
```

Run the CI-equivalent local checks:

```bash
bash scripts/ci/run-checks.sh
```

Export demo reports:

```bash
bash scripts/ci/export-demo-reports.sh
```

## Working rules

- keep docs close to architecture changes
- prefer deterministic fixtures
- treat external contract inputs as hostile
- do not describe unsupported coverage as if it already exists
- keep secure and vulnerable fixture pairs easy to compare
