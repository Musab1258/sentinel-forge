# Sentinel Forge

Advanced security, verification, and exploit analysis infrastructure for Soroban smart contracts.

[Architecture](docs/architecture/overview.md) · [Roadmap](ROADMAP.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md)

## Overview

Sentinel Forge is a modular security platform for Soroban development. The project is designed to grow from a credible static analysis baseline into a broader verification stack that includes fuzzing, symbolic execution, exploit replay, and evidence-rich reporting.

The immediate goal is practical: make Soroban security work easier to understand, automate, and integrate into developer workflows before contracts reach production.

## Why it exists

Soroban still has a relatively thin security tooling layer compared with more mature smart contract ecosystems. That creates three recurring problems:

- teams rely too heavily on manual review
- smaller projects struggle to afford deep audit coverage
- developers lack reusable tooling for authorization analysis, state safety, and execution-level reasoning

Sentinel Forge exists to close that gap with contributor-friendly, open infrastructure.

## Current state

The repository currently includes working project foundations and a documented architecture surface:

- `apps/landing`: public-facing project site built with Next.js
- `engines/static-analyzer`: initial Rust crate and CLI bootstrap
- `engines/fuzzer`, `engines/symbolic-executor`, `engines/verification-engine`: staged engine crates
- `docs/`: architecture, security, research, and contributor guides
- `examples/`: space for vulnerable and secure contract fixtures

The static analyzer binary is still a bootstrap entrypoint, not a full scanner yet. Phase 2 focuses on documenting the intended system clearly enough that contributors can extend it coherently.

## Capability map

| Capability | Status | Notes |
| --- | --- | --- |
| Landing page and project positioning | Active | Public explanation of modules, architecture, and roadmap |
| Static analyzer crate | Bootstrap | Rust workspace member and CLI banner exist |
| Detector architecture | Documented | Detector focus and plugin approach defined in docs |
| Reporting architecture | Documented | JSON, SARIF, HTML, and dashboard paths defined |
| Fuzzing | Planned | Architecture and contributor guidance drafted |
| Symbolic execution | Planned | Path exploration design documented |
| Formal verification | Planned | Verification engine reserved in workspace |

## Architecture overview

```mermaid
flowchart LR
    A[Contracts and fixtures] --> B[Source and WASM ingest]
    B --> C[AST and intermediate representations]
    C --> D[Detector and analysis engines]
    D --> E[Findings and evidence model]
    E --> F[CLI output]
    E --> G[CI and SARIF]
    E --> H[Dashboard and APIs]
```

The first implementation lane centers on static analysis because the phase 0 research identified authorization correctness, unsafe state access, arithmetic issues, and denial-of-service patterns as the highest-leverage initial detector classes.

See the deeper docs:

- [Analysis pipeline](docs/architecture/analysis-pipeline.md)
- [AST pipeline](docs/architecture/ast-pipeline.md)
- [WASM analysis](docs/architecture/wasm-analysis.md)
- [Plugin system](docs/architecture/plugin-system.md)
- [Reporting system](docs/architecture/reporting-system.md)

## Supported analysis types

Current emphasis:

- authorization flow analysis
- state access and mutation checks
- arithmetic risk detection
- denial-of-service pattern detection

Planned expansion:

- WASM-aware inspection
- symbolic execution
- property-based fuzzing
- exploit simulation
- invariant verification

## Quick start

Install workspace dependencies:

```bash
corepack pnpm install
cargo build
```

Run the landing page:

```bash
corepack pnpm --filter @sentinel-forge/landing dev
```

Run the current analyzer bootstrap:

```bash
cargo run -p static-analyzer --bin sentinel-forge
```

The workspace includes a local `pnpm` shim under `.bin/` so root Turborepo scripts still work when `pnpm` is not globally available.

## Repository structure

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
  architecture/
  security/
  research/
  guides/
examples/
scripts/
```

## Developer workflow

Common commands:

```bash
corepack pnpm build
corepack pnpm test
cargo test
```

See [docs/guides/local-development.md](docs/guides/local-development.md) for a fuller setup and workflow guide.

## Documentation map

- [Architecture overview](docs/architecture/overview.md)
- [Threat model](docs/security/threat-model.md)
- [Detector guide](docs/guides/writing-detectors.md)
- [Reporter guide](docs/guides/creating-reporters.md)
- [Phase 0 research summary](docs/research/phase-0-summary.md)
- [Contributor roadmap](docs/contributing/roadmap.md)

## Roadmap

The project is organized in phases:

- Phase 0: security research, scope definition, and MVP selection
- Phase 1: branding, monorepo infrastructure, landing page, and engine scaffolding
- Phase 2: documentation, architecture, threat modeling, and contributor guidance
- Phase 3: credible static analyzer MVP

See [ROADMAP.md](ROADMAP.md) for the full phase breakdown.

## Security notice

Sentinel Forge is experimental security infrastructure. Findings and future scan results are intended to support human review, not replace formal audits or expert assessment.
