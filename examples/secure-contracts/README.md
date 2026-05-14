# Secure Contract Fixtures

This directory contains guarded examples that the analyzer should treat as clean baselines when its heuristics are behaving as expected.

Top-level regression baseline:

- `guarded_admin.rs`

Phase 5 adds category-aligned secure fixtures so detector contributors can compare safe and unsafe patterns side by side:

- `authorization/require_auth_guard.rs`
- `storage/validated_storage_write.rs`
- `dos/bounded_distribution.rs`
- `arithmetic/checked_mint.rs`
