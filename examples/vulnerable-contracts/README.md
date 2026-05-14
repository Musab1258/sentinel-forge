# Vulnerable Contract Fixtures

This directory contains intentionally unsafe Soroban-style fixtures used for detector work, regression tests, onboarding, and future benchmark comparisons.

Top-level phase 3 fixtures remain in place for simple analyzer demos:

- `missing_authorization.rs`
- `unchecked_arithmetic.rs`
- `unbounded_loop.rs`

Phase 5 adds categorized fixtures so contributors can work from narrower examples:

- `authorization/unsafe_delegate.rs`
- `storage/unsafe_storage_write.rs`
- `dos/dos_loop.rs`
- `arithmetic/overflow.rs`
- `privilege-escalation/unsafe_admin_rotation.rs`
