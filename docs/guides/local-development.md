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

## Working rules

- keep docs close to architecture changes
- prefer deterministic fixtures
- treat external contract inputs as hostile
- do not describe unsupported coverage as if it already exists
