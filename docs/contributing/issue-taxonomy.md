# Issue Taxonomy

This document turns the phase 5 contributor-readiness brief into a repository-side issue structure.

## Core categories

| Label | Use it for |
| --- | --- |
| `detector` | static-analysis rule work and detector regressions |
| `research` | exploratory Soroban, WASM, symbolic, or exploit-analysis questions |
| `fuzzing` | future fuzz engine design and fixture preparation |
| `symbolic-execution` | path exploration, constraints, and symbolic state work |
| `frontend` | landing page, dashboard, exploit-lab, and extension UX |
| `backend` | Rust engine implementation and shared services |
| `docs` | onboarding, architecture, and setup documentation |
| `testing` | regressions, benchmark corpora, and hostile-input validation |
| `security` | security-sensitive workflow or implementation changes |

## Supporting labels

| Label | Use it for |
| --- | --- |
| `good-first-issue` | narrow tasks that expose one subsystem clearly |
| `help-wanted` | tasks ready for outside contribution |
| `architecture` | design, interfaces, and system-boundary changes |
| `visualization` | rendering, dashboards, and exploit-lab interaction |
| `wasm` | low-level bytecode or runtime analysis work |
| `cli` | command-line behavior and workflow integration |
| `api` | shared interfaces and future service surfaces |
| `performance` | scan time, memory, and scaling work |
| `regression` | correctness fixes backed by reproducible fixtures |

## Priority labels

| Label | Meaning |
| --- | --- |
| `critical` | core infrastructure blocker or security-sensitive gap |
| `high-priority` | important near-term milestone work |
| `medium-priority` | useful scheduled work |
| `low-priority` | enhancement or cleanup |

## Recommended issue structure

Every issue should cover:

1. Objective
2. Context
3. Requirements
4. Suggested approach
5. References

That structure is encoded in the repository issue templates under `.github/ISSUE_TEMPLATE/`.

## Good first issue profile

Good onboarding tasks usually have these properties:

- one subsystem only
- no architecture rewrite required
- deterministic tests or fixture updates
- a clear doc reference
- a reviewer can validate success quickly

Examples:

- improve CLI output formatting
- add severity badges to HTML report output
- expand a secure or vulnerable fixture README
- tighten one detector regression case

## Suggested project boards

These boards map well to the current repo:

- `Core Engine`
- `Visualization`
- `Research`
- `Contributor Tasks`

Recommended columns:

- `Backlog`
- `In Progress`
- `Research`
- `Review`
- `Blocked`
- `Completed`
