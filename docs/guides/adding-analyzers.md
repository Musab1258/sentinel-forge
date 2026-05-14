# Adding Analyzers

This guide describes how Sentinel Forge should grow beyond the initial static analyzer without collapsing the architecture.

## Supported analyzer families

- static analyzer
- fuzz engine
- symbolic executor
- verification engine

## Core rule

New analyzers should integrate with the shared reporting and contributor model rather than inventing a separate workflow for every engine.

## Integration flow

```text
Analyzer module
      ↓
Ingest and normalization
      ↓
Analysis execution
      ↓
Findings or proof artifacts
      ↓
Shared reporting surfaces
```

## Recommended steps

1. Add or extend a crate under `engines/`.
2. Define the analyzer input, output, and trust boundaries early.
3. Reuse shared finding shapes when the output represents actionable findings.
4. Keep engine-specific execution details out of UI or reporter code.
5. Add docs under `docs/architecture/` when the engine model becomes real.

## Interface guidance

Phase 5 does not require one concrete trait across every engine, but new analyzers should still expose one stable execution entry point with explicit input and output types.

An analyzer should answer:

- what inputs it accepts
- whether inputs are trusted
- what artifact it emits
- what deterministic guarantees it provides
- how contributors test it

## When to avoid coupling

Do not:

- make the dashboard depend on engine internals
- let reporters inspect private analyzer state
- merge hostile-input execution into harmless parsing paths without clear boundaries
- hide expensive analysis behind lightweight commands unexpectedly

## Required documentation

Before an analyzer is treated as contributor-ready, add:

- an architecture doc
- local development notes
- testing guidance
- at least one example or fixture path
