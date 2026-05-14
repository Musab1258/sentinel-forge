# Writing Detectors

This guide describes how contributors should approach detector work for the static analyzer.

## Detector goals

A detector should answer one clear question about contract safety and produce evidence a developer can act on.

## Start with a security statement

Document:

- the risky pattern
- why it matters for Soroban
- expected severity
- expected confidence
- known false-positive and false-negative conditions

## Current interface

The current detector contract lives in `engines/static-analyzer/src/rules/mod.rs`:

```rust
pub trait Detector: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn severity(&self) -> Severity;
    fn analyze(&self, contract: &IRContract) -> Vec<Finding>;
}
```

Built-in detectors are registered through `DetectorRegistry::with_builtins()` and implemented under `engines/static-analyzer/src/detectors/`.

## Recommended detector workflow

1. Start from one narrow security statement.
2. Inspect the current IR and operation kinds before adding heuristics.
3. Emit findings with locations, evidence, and remediation hints.
4. Add positive, negative, and edge-case fixtures.
5. Register the detector and verify output shape through tests.

## Initial focus areas

- authorization correctness
- storage access controls
- privilege escalation
- missing input validation
- arithmetic safety
- denial-of-service patterns

## Suggested implementation steps

1. Add a detector module under `engines/static-analyzer/src/detectors/`.
2. Implement the `Detector` trait with stable metadata.
3. Reuse shared helpers such as operation filtering and `base_finding`.
4. Export the detector from `src/detectors/mod.rs`.
5. Register it in `built_in_detectors()`.
6. Add fixtures under `examples/` and `engines/static-analyzer/tests/` where needed.

## Evidence expectations

Findings should usually include:

- the risky function name
- the operation or line that triggered the finding
- the missing guard, assumption, or unsafe transition
- a remediation hint that maps to actual contract changes

## Testing expectations

- add deterministic fixtures
- make assertions on the finding shape, not just count
- cover at least one safe case and one unsafe case
- add an edge case when the detector could become noisy

## Fixture strategy

Use the repository example suites deliberately:

- `examples/vulnerable-contracts/` for intentionally unsafe patterns
- `examples/secure-contracts/` for negative coverage
- `examples/benchmark-contracts/` when detector cost matters
- `examples/exploit-scenarios/` when the rule is easier to explain as an attack flow

## Common failure modes

- mixing multiple vulnerability classes into one detector
- reporting a conclusion without enough evidence
- encoding Soroban assumptions in string matching that cannot survive normal refactors
- adding a detector without a clean negative case

## Documentation expectations

If a detector introduces a new concept or changes the analysis model, update the architecture docs that describe that behavior.
