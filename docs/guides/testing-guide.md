# Testing Guide

This guide defines the contributor testing baseline for phase 5.

## Principles

- prefer deterministic fixtures
- test the output shape, not only success or failure
- keep secure and vulnerable examples paired when possible
- make performance-sensitive changes measurable

## Frontend work

For landing-page or TypeScript changes:

```bash
corepack pnpm --filter @sentinel-forge/landing test
corepack pnpm build
```

Run broader workspace commands when the change spans multiple apps or packages:

```bash
corepack pnpm test
corepack pnpm build
```

## Rust engine work

For detector, CLI, or reporting changes:

```bash
cargo test -p static-analyzer
cargo build
```

Run the analyzer on the fixture path you changed when practical:

```bash
cargo run -p static-analyzer --bin sentinel-forge -- scan examples/vulnerable-contracts
```

## Fixture changes

When adding or modifying fixtures:

- explain whether the fixture is vulnerable, secure, benchmark, or exploit-oriented
- keep the fixture focused on one primary lesson
- add a matching safe or unsafe comparison when the detector could be noisy
- note if the fixture is for education only and not expected to parse in current engines

## Docs-only changes

For documentation work:

- verify links and command examples
- keep architecture and workflow language aligned with the current repo
- mention any intentionally deferred work instead of implying it exists already

## Pull request note

If you skip a test because the change is docs-only or the relevant engine is not implemented yet, say that explicitly in the pull request.
