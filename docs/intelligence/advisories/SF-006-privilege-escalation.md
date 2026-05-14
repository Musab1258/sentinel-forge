# SF-006: Privilege Escalation Risk

## Summary

Contract logic appears to modify privileged roles or admin-like state without proving the caller is already authorized to do so.

## Why it matters

Privilege transitions are among the highest-risk contract operations. If they are weakly guarded, all later protections that assume a trusted admin can become meaningless.

## Typical exploit path

1. Attacker reaches an admin-rotation or delegated-execution path.
2. Contract writes a privileged state change.
3. Authorization is absent or incomplete.
4. Future admin-only flows now accept the attacker.

## Current fixture

- `examples/vulnerable-contracts/privilege-escalation/unsafe_admin_rotation.rs`

## Remediation

- require the current admin or trusted role to authorize the transition
- validate the new privileged address
- add regression coverage for both direct and delegated privilege flows
