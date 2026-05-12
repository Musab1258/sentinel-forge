# Sentinel Forge

Security and verification infrastructure for Soroban smart contracts.

## What is included

- `apps/landing`: Next.js landing page for the public-facing project site
- `engines/static-analyzer`: initial Rust analyzer and CLI skeleton
- `engines/*`: placeholder engine crates for fuzzing, symbolic execution, and verification
- `docs/*`: architecture, research, security, and contributor notes
- `packages/config`: shared TypeScript base configuration

## Quick start

```bash
corepack pnpm install
corepack pnpm build
corepack pnpm --filter @sentinel-forge/landing dev
```

The workspace includes a local `pnpm` shim under `.bin/` so the root Turborepo scripts work even when a global `pnpm` binary is unavailable.

## Project structure

```text
apps/
  landing/
  dashboard/
  exploit-lab/
engines/
  static-analyzer/
  fuzzer/
  symbolic-executor/
  verification-engine/
packages/
  config/
  sdk/
  shared/
  ui/
docs/
examples/
scripts/
```

## Current focus

Phase 1 establishes the monorepo, the landing page, the initial Rust workspace, and the core documentation surface that contributors need to understand the project.
