# Sandboxing Guidance

Sentinel Forge will eventually inspect hostile source code, malformed WASM, and adversarial execution inputs. Sandboxing is therefore a design requirement, not an optional hardening step.

## Principles

- treat all external contracts and corpora as untrusted
- separate parsing from execution whenever possible
- deny network access for hostile-input workloads unless there is a documented reason not to
- cap CPU, memory, and disk usage for expensive analyses

## Workload categories

### Safe by default

- static markdown and docs
- deterministic fixture checks that do not execute untrusted code

### Higher risk

- exploit replay
- fuzzing harnesses
- symbolic execution against malformed artifacts
- future WASM execution environments

## Minimum controls

- run execution-heavy workflows in isolated processes
- use ephemeral directories for generated artifacts
- make logs explicit about whether a path handled untrusted input
- avoid mixing secrets or production credentials into analysis environments

## Contributor expectation

If a change introduces new code paths that execute or simulate untrusted artifacts, the PR should document:

- what runs
- what is isolated
- which resources are capped
- which risks remain
