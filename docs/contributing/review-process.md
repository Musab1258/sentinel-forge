# Review Process

Sentinel Forge review should optimize for security correctness and contributor clarity before velocity.

## Contributor workflow

1. Read the relevant architecture and guide documents first.
2. Scope the change to one clear objective.
3. Add or update tests and fixtures alongside the implementation.
4. Update docs if the contributor workflow or public behavior changed.
5. Open a pull request with architectural and regression notes.

## Pull request expectations

Every pull request should explain:

- what changed
- why the change belongs in the current architecture
- which tests were run
- whether docs were updated
- what blind spots, risks, or performance costs remain

Use `.github/PULL_REQUEST_TEMPLATE.md` as the default structure.

## Review priorities

Reviewers should prioritize:

1. correctness
2. security implications
3. evidence quality
4. false-positive and false-negative tradeoffs
5. architectural consistency
6. contributor readability

## Detector-specific review questions

- does the rule map to a real Soroban risk
- is the evidence concrete enough to justify the finding
- is there at least one clean negative case
- are known blind spots documented
- does the detector stay narrow enough to evolve safely

## Reporter and frontend review questions

- is shared finding data preserved without reinterpretation
- is output stable for automation and UI consumers
- does the UI make severity, evidence, and scope obvious
- does the change increase maintenance burden disproportionately

## Benchmark and fixture review questions

- are fixtures intentionally small and reviewable
- is the exploit or regression path reproducible
- are safe and unsafe examples easy to compare
- does the change accidentally broaden test cost too much
