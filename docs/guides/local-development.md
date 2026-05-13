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

## Working rules

- keep docs close to architecture changes
- prefer deterministic fixtures
- treat external contract inputs as hostile
- do not describe unsupported coverage as if it already exists
