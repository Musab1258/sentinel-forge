# Extending Engines

This guide covers internal extension points that sit below user-facing analyzers and reporters.

## Core extension areas

| Area | Why it matters |
| --- | --- |
| AST parsing | expands language coverage and semantic extraction |
| IR generation | normalizes source details into analysis-friendly data |
| tracing | explains how a finding was derived |
| reporting | keeps outputs consistent across CLI, CI, and UI |
| visualization | renders findings without redefining detector meaning |

## Safety principles

- preserve deterministic behavior where possible
- avoid breaking existing finding contracts casually
- isolate hostile-input execution paths
- keep scan-time cost visible when adding deeper analysis

## Change checklist

1. Identify which subsystem boundary you are crossing.
2. Check whether a guide or architecture doc already defines the current contract.
3. Add targeted tests for that boundary.
4. Update docs when the extension changes contributor expectations.

## Typical examples

- adding an operation kind to the IR so a detector can reason about new state transitions
- improving trace generation so HTML, dashboard, and VSCode views inherit better evidence
- extending report rendering without changing detector semantics
- preparing a benchmark corpus before optimizing a parser or detector path

## When to pause and redesign

Stop and document the design first if the change:

- forces multiple engines to share hidden assumptions
- changes the meaning of existing findings
- makes tests non-deterministic without a containment strategy
- introduces execution of hostile artifacts where only parsing existed before
