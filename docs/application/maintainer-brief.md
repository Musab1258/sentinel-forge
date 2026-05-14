# Maintainer Brief

Sentinel Forge is a Soroban security and verification infrastructure project.

## Current proof points

- a working Rust static analyzer with CLI, JSON, SARIF, and HTML outputs
- a landing page that explains architecture and roadmap
- dashboard and exploit-lab surfaces that make findings and attack flow legible
- contributor infrastructure with issue templates, review expectations, and structured fixtures

## Why the project matters

Soroban still lacks the depth of open security tooling seen in more mature contract ecosystems. Sentinel Forge is designed to close that gap incrementally, starting with static analysis and expanding into fuzzing, symbolic execution, verification, and operational workflows.

## Local demo surfaces

- `apps/landing`: public explanation and project positioning
- `apps/dashboard`: findings triage and scan history surface
- `apps/exploit-lab`: attack-path and replay-oriented visualization surface
- `engines/static-analyzer`: local CLI and report-generation path

## What phase 6 adds

- a reproducible demo path
- a maintainership-facing brief
- clearer readiness criteria for public review
- CI and export scaffolding that helps convert prototypes into dependable repo surfaces
