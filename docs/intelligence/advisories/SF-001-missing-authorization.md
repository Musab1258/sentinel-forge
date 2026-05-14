# SF-001: Missing Authorization Check

## Summary

Privileged or state-changing logic executes without a visible `require_auth()` or equivalent authorization guard.

## Why it matters

Soroban contracts often rely on explicit authorization boundaries. If those checks are absent, callers can mutate protected state or trigger actions reserved for a trusted actor.

## Typical exploit path

1. Public function accepts attacker-controlled input.
2. Function writes state or triggers a privileged action.
3. No authorization gate stops the call.
4. Attacker gains control over state or workflow.

## Current fixture

- `examples/vulnerable-contracts/missing_authorization.rs`

## Remediation

- require authorization before the privileged operation
- separate read-only and admin paths clearly
- add a secure negative fixture alongside the vulnerable one
