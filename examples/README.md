# Example Suites

The `examples/` tree is part of the contributor infrastructure, not just demo content.

## Directories

- `vulnerable-contracts/`: intentionally unsafe fixtures for detector and onboarding work
- `secure-contracts/`: guarded examples that protect against false positives
- `benchmark-contracts/`: small corpora for scale and performance-oriented changes
- `exploit-scenarios/`: attack narratives that explain how findings connect to real exploit flow

## Contributor guidance

- keep each fixture focused on one primary lesson
- prefer readable, reviewable examples over realism-by-size
- pair safe and unsafe variants where practical
- document whether a fixture is meant for current analyzer coverage or future research
