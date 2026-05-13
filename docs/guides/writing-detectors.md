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

## Recommended detector shape

1. Consume normalized analysis context.
2. Look for one category of unsafe behavior.
3. Emit findings with locations, evidence, and remediation hints.
4. Add positive and negative fixtures.

## Initial focus areas

- authorization correctness
- storage access controls
- privilege escalation
- missing input validation
- arithmetic safety
- denial-of-service patterns

## Testing expectations

- add deterministic fixtures
- make assertions on the finding shape, not just count
- cover at least one safe case and one unsafe case

## Documentation expectations

If a detector introduces a new concept or changes the analysis model, update the architecture docs that describe that behavior.
